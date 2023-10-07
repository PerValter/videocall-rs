// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `types/rsa_packet.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:RsaPacket)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RsaPacket {
    // message fields
    // @@protoc_insertion_point(field:RsaPacket.public_key_der)
    pub public_key_der: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:RsaPacket.username)
    pub username: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:RsaPacket.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RsaPacket {
    fn default() -> &'a RsaPacket {
        <RsaPacket as ::protobuf::Message>::default_instance()
    }
}

impl RsaPacket {
    pub fn new() -> RsaPacket {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "public_key_der",
            |m: &RsaPacket| { &m.public_key_der },
            |m: &mut RsaPacket| { &mut m.public_key_der },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "username",
            |m: &RsaPacket| { &m.username },
            |m: &mut RsaPacket| { &mut m.username },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RsaPacket>(
            "RsaPacket",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RsaPacket {
    const NAME: &'static str = "RsaPacket";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.public_key_der = is.read_bytes()?;
                },
                18 => {
                    self.username = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.public_key_der.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.public_key_der);
        }
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.username);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.public_key_der.is_empty() {
            os.write_bytes(1, &self.public_key_der)?;
        }
        if !self.username.is_empty() {
            os.write_string(2, &self.username)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RsaPacket {
        RsaPacket::new()
    }

    fn clear(&mut self) {
        self.public_key_der.clear();
        self.username.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RsaPacket {
        static instance: RsaPacket = RsaPacket {
            public_key_der: ::std::vec::Vec::new(),
            username: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RsaPacket {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RsaPacket").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RsaPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RsaPacket {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16types/rsa_packet.proto\"M\n\tRsaPacket\x12$\n\x0epublic_key_der\
    \x18\x01\x20\x01(\x0cR\x0cpublicKeyDer\x12\x1a\n\x08username\x18\x02\x20\
    \x01(\tR\x08usernameJ\x98\x01\n\x06\x12\x04\0\0\x05\x01\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x05\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\x02\x08\x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x02\x1b\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x02\x07\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x03\x08\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x19\x1a\
    \n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x02\x16\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03\x04\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\t\x11\
    \n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x14\x15b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RsaPacket::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
