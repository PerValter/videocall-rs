use anyhow::Result;
use image::RgbImage;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::RequestedFormat;
use nokhwa::utils::RequestedFormatType;
use nokhwa::{
    utils::{ApiBackend, CameraFormat, CameraIndex, FrameFormat},
    Camera,
};
use protobuf::Message;
use rav1e::prelude::*;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::{self, Sender};
use tracing::{debug, error, info, warn};

use types::protos::media_packet::media_packet::MediaType;
use types::protos::media_packet::{MediaPacket, VideoMetadata};
use types::protos::packet_wrapper::{packet_wrapper::PacketType, PacketWrapper};

type CameraPacket = (RgbImage, u128);

pub fn transform_video_chunk(chunk: &Packet<u8>, email: &str) -> PacketWrapper {
    let frame_type = if chunk.frame_type == FrameType::KEY {
        "key".to_string()
    } else {
        "delta".to_string()
    };
    let media_packet: MediaPacket = MediaPacket {
        data: chunk.data.clone(),
        frame_type,
        email: email.to_owned(),
        media_type: MediaType::VIDEO.into(),
        timestamp: since_the_epoch().as_micros() as f64,
        video_metadata: Some(VideoMetadata {
            sequence: chunk.input_frameno,
            ..Default::default()
        })
        .into(),
        ..Default::default()
    };
    let data = media_packet.write_to_bytes().unwrap();
    PacketWrapper {
        data,
        email: media_packet.email,
        packet_type: PacketType::MEDIA.into(),
        ..Default::default()
    }
}

static THRESHOLD_MILLIS: u128 = 1000;

pub fn since_the_epoch() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

#[derive(Copy, Clone, Debug)]
pub struct CameraConfig {
    pub width: u32,
    pub height: u32,
    pub framerate: u32,
    pub video_device_index: usize,
    pub frame_format: FrameFormat,
}

pub struct CameraDaemon {
    config: CameraConfig,
    fps_rx: Option<mpsc::Receiver<u128>>,
    fps_tx: Arc<mpsc::Sender<u128>>,
    cam_rx: Option<mpsc::Receiver<Option<CameraPacket>>>,
    cam_tx: Arc<mpsc::Sender<Option<CameraPacket>>>,
    quic_tx: Arc<Sender<Vec<u8>>>,
    quit: Arc<AtomicBool>,
    handles: Vec<JoinHandle<()>>,
}

impl CameraDaemon {
    pub fn from_config(config: CameraConfig, quic_tx: Sender<Vec<u8>>) -> CameraDaemon {
        let (fps_tx, fps_rx) = mpsc::channel(1);
        let (cam_tx, cam_rx) = mpsc::channel(1);
        CameraDaemon {
            config,
            fps_rx: Some(fps_rx),
            fps_tx: Arc::new(fps_tx),
            cam_rx: Some(cam_rx),
            cam_tx: Arc::new(cam_tx),
            quit: Arc::new(AtomicBool::new(false)),
            handles: vec![],
            quic_tx: Arc::new(quic_tx),
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.handles.push(self.camera_thread()?);
        let encoder = self.encoder_thread();
        self.handles.push(encoder);
        let fps = self.fps_thread();
        self.handles.push(fps);
        Ok(())
    }

    fn camera_thread(&self) -> Result<JoinHandle<()>> {
        let devices = nokhwa::query(ApiBackend::Auto)?;
        for i in 0..devices.len() {
            info!("available device index {}: {:?}", i, devices[i]);
        }
        let cam_tx = self.cam_tx.clone();
        let width = self.config.width;
        let height = self.config.height;
        let framerate = self.config.framerate;
        let frame_format = self.config.frame_format;
        let video_device_index = self.config.video_device_index as u32;
        let quit = self.quit.clone();
        Ok(std::thread::spawn(move || {
            info!("Camera opened... waiting for frames");
            let mut camera = Camera::new(
                CameraIndex::Index(video_device_index),
                RequestedFormat::new::<RgbFormat>(RequestedFormatType::Closest(
                    CameraFormat::new_from(width, height, frame_format, framerate),
                )),
            )
            .unwrap();
            camera.open_stream().unwrap();
            while let Ok(frame) = camera.frame() {
                if quit.load(std::sync::atomic::Ordering::Relaxed) {
                    return ();
                }
                debug!("source frame format: {:?}", frame.source_frame_format());
                if let Err(e) = cam_tx.try_send(Some((frame.decode_image::<RgbFormat>().unwrap(), since_the_epoch().as_millis()))) {
                    error!("Unable to send image: {}", e);
                }
            }
        }))
    }

    fn encoder_thread(&mut self) -> JoinHandle<()> {
        let mut enc = EncoderConfig::default();
        warn!("Using config: {:?}", self.config);
        let width = self.config.width;
        let height = self.config.height;
        enc.width = width as usize;
        enc.height = height as usize;
        enc.bit_depth = 8;
        enc.error_resilient = true;
        enc.speed_settings = SpeedSettings::from_preset(10);
        enc.speed_settings.rdo_lookahead_frames = 1;
        enc.min_key_frame_interval = 20;
        enc.max_key_frame_interval = 50;
        enc.low_latency = true;
        enc.min_quantizer = 100;
        enc.quantizer = 150;
        enc.tiles = 16;
        enc.bitrate = 1000;
        enc.chroma_sampling = ChromaSampling::Cs420;
        enc.chroma_sample_position = ChromaSamplePosition::Unknown;

        debug!("encoder config: {:?}", enc);

        // TODO What is this???
        // enc.tune = Tune::Psnr;

        let cfg = Config::new().with_encoder_config(enc).with_threads(16);
        let fps_tx = self.fps_tx.clone();
        let mut cam_rx = self.cam_rx.take().unwrap();
        let quic_tx = self.quic_tx.clone();
        let quit = self.quit.clone();
        std::thread::spawn(move || {
            loop {
                let mut ctx: Context<u8> = cfg.new_context().unwrap();
                while let Some(data) = cam_rx.blocking_recv() {
                    let start = Instant::now();
                    if quit.load(std::sync::atomic::Ordering::Relaxed) {
                        return ();
                    }
                    let (image, age) = data.unwrap();
                    // If age older than threshold, throw it away.
                    let image_age = since_the_epoch().as_millis() - age;
                    if image_age > THRESHOLD_MILLIS {
                        debug!("throwing away old image with age {} ms", image_age);
                        continue;
                    }
                    let mut frame = ctx.new_frame();
                    add_data_to_frame(&mut frame, image, width as usize);
                    let encoding_time = Instant::now();
                    match ctx.send_frame(frame) {
                        Ok(_) => {
                            debug!("queued frame");
                        }
                        Err(e) => match e {
                            EncoderStatus::EnoughData => {
                                debug!("Unable to append frame to the internal queue");
                            }
                            _ => {
                                panic!("Unable to send frame");
                            }
                        },
                    }
                    debug!("receiving encoded frame");
                    match ctx.receive_packet() {
                        Ok(pkt) => {
                            debug!(
                                "encoder thread: time encoding {:?}",
                                encoding_time.elapsed()
                            );
                            let packet_wrapper = transform_video_chunk(&pkt, "test");
                            if let Err(e) = fps_tx.try_send(since_the_epoch().as_millis()) {
                                error!("Unable to send fps: {:?}", e);
                            }
                            if let Err(e) =
                                quic_tx.try_send(packet_wrapper.write_to_bytes().unwrap())
                            {
                                error!("Unable to send packet: {:?}", e);
                            }
                        }
                        Err(e) => match e {
                            EncoderStatus::LimitReached => {
                                warn!("read thread: Limit reached");
                            }
                            EncoderStatus::Encoded => debug!("read thread: Encoded"),
                            EncoderStatus::NeedMoreData => debug!("read thread: Need more data"),
                            _ => {
                                warn!("read thread: Unable to receive packet");
                            }
                        },
                    }
                    debug!("encoder thread: time elapsed {:?}", start.elapsed());
                }
            }
        })
    }

    fn fps_thread(&mut self) -> JoinHandle<()> {
        let mut fps_rx = self.fps_rx.take().unwrap();
        let quit = self.quit.clone();
        std::thread::spawn(move || {
            let mut num_frames = 0;
            let mut now_plus_1 = since_the_epoch().as_millis() + 1000;
            warn!("Starting fps loop");
            while let Some(dur) = fps_rx.blocking_recv() {
                if quit.load(std::sync::atomic::Ordering::Relaxed) {
                    return ();
                }
                if now_plus_1 < dur {
                    warn!("FPS: {:?}", num_frames);
                    num_frames = 0;
                    now_plus_1 = since_the_epoch().as_millis() + 1000;
                } else {
                    num_frames += 1;
                }
            }
        })
    }

    pub fn stop(&mut self) -> Result<()> {
        self.quit.store(true, std::sync::atomic::Ordering::Relaxed);
        for handle in self.handles.drain(..) {
            handle.join().unwrap();
        }
        Ok(())
    }
}

fn clamp(val: f32) -> u8 {
    (val.round() as u8).max(0_u8).min(255_u8)
}

fn to_ycbcr(pixel: [u8; 3]) -> (u8, u8, u8) {
    let [r, g, b] = pixel;

    let y = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
    let u = (b as f32 - y) / 2.0 as f32;
    let v = (r as f32 - y) / 2.0 as f32;

    (clamp(y), clamp(u), clamp(v))
}

fn add_data_to_frame(frame: &mut Frame<u8>, img: RgbImage, width: usize) {
    let mut y_slice: Vec<u8> = vec![];
    let mut u_slice: Vec<u8> = vec![];
    let mut v_slice: Vec<u8> = vec![];
    for pixel in img.pixels() {
        let (y, u, v) = to_ycbcr(pixel.0);
        y_slice.push(y);
        u_slice.push(u);
        v_slice.push(v);
    }
    let planes = vec![y_slice, u_slice, v_slice];
    for (dst, src) in frame.planes.iter_mut().zip(planes) {
        dst.copy_from_raw_u8(&src, width, 1);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_ycbcr() {
        let pixel = [123, 69, 20];
        let output = to_ycbcr(pixel);
        assert_eq!(output, (84, 98, 155));
    }
}