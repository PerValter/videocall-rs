// This file is generated by rust-protobuf 3.2.0. Do not edit
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

//! Generated file from `types/packet_wrapper.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:PacketWrapper)
pub struct PacketWrapper {
    // message fields
    // @@protoc_insertion_point(field:PacketWrapper.packet_type)
    pub packet_type: ::protobuf::EnumOrUnknown<packet_wrapper::PacketType>,
    // @@protoc_insertion_point(field:PacketWrapper.email)
    pub email: ::std::string::String,
    // @@protoc_insertion_point(field:PacketWrapper.data)
    pub data: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:PacketWrapper.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PacketWrapper {
    fn default() -> &'a PacketWrapper {
        <PacketWrapper as ::protobuf::Message>::default_instance()
    }
}

impl PacketWrapper {
    pub fn new() -> PacketWrapper {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "packet_type",
            |m: &PacketWrapper| { &m.packet_type },
            |m: &mut PacketWrapper| { &mut m.packet_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "email",
            |m: &PacketWrapper| { &m.email },
            |m: &mut PacketWrapper| { &mut m.email },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &PacketWrapper| { &m.data },
            |m: &mut PacketWrapper| { &mut m.data },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PacketWrapper>(
            "PacketWrapper",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PacketWrapper {
    const NAME: &'static str = "PacketWrapper";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.packet_type = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.email = is.read_string()?;
                },
                26 => {
                    self.data = is.read_bytes()?;
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
        if self.packet_type != ::protobuf::EnumOrUnknown::new(packet_wrapper::PacketType::RSA_PUB_KEY) {
            my_size += ::protobuf::rt::int32_size(1, self.packet_type.value());
        }
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.email);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.packet_type != ::protobuf::EnumOrUnknown::new(packet_wrapper::PacketType::RSA_PUB_KEY) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.packet_type))?;
        }
        if !self.email.is_empty() {
            os.write_string(2, &self.email)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
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

    fn new() -> PacketWrapper {
        PacketWrapper::new()
    }

    fn clear(&mut self) {
        self.packet_type = ::protobuf::EnumOrUnknown::new(packet_wrapper::PacketType::RSA_PUB_KEY);
        self.email.clear();
        self.data.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PacketWrapper {
        static instance: PacketWrapper = PacketWrapper {
            packet_type: ::protobuf::EnumOrUnknown::from_i32(0),
            email: ::std::string::String::new(),
            data: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PacketWrapper {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PacketWrapper").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PacketWrapper {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PacketWrapper {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PacketWrapper`
pub mod packet_wrapper {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:PacketWrapper.PacketType)
    pub enum PacketType {
        // @@protoc_insertion_point(enum_value:PacketWrapper.PacketType.RSA_PUB_KEY)
        RSA_PUB_KEY = 0,
        // @@protoc_insertion_point(enum_value:PacketWrapper.PacketType.AES_KEY)
        AES_KEY = 1,
        // @@protoc_insertion_point(enum_value:PacketWrapper.PacketType.MEDIA)
        MEDIA = 2,
    }

    impl ::protobuf::Enum for PacketType {
        const NAME: &'static str = "PacketType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<PacketType> {
            match value {
                0 => ::std::option::Option::Some(PacketType::RSA_PUB_KEY),
                1 => ::std::option::Option::Some(PacketType::AES_KEY),
                2 => ::std::option::Option::Some(PacketType::MEDIA),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [PacketType] = &[
            PacketType::RSA_PUB_KEY,
            PacketType::AES_KEY,
            PacketType::MEDIA,
        ];
    }

    impl ::protobuf::EnumFull for PacketType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("PacketWrapper.PacketType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for PacketType {
        fn default() -> Self {
            PacketType::RSA_PUB_KEY
        }
    }

    impl PacketType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PacketType>("PacketWrapper.PacketType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1atypes/packet_wrapper.proto\"\xac\x01\n\rPacketWrapper\x12:\n\x0bpa\
    cket_type\x18\x01\x20\x01(\x0e2\x19.PacketWrapper.PacketTypeR\npacketTyp\
    e\x12\x14\n\x05email\x18\x02\x20\x01(\tR\x05email\x12\x12\n\x04data\x18\
    \x03\x20\x01(\x0cR\x04data\"5\n\nPacketType\x12\x0f\n\x0bRSA_PUB_KEY\x10\
    \0\x12\x0b\n\x07AES_KEY\x10\x01\x12\t\n\x05MEDIA\x10\x02J\xf8\x02\n\x06\
    \x12\x04\0\0\x0b\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\
    \x04\x02\0\x0b\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x15\n\x0c\n\x04\
    \x04\0\x04\0\x12\x04\x03\x02\x07\x03\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03\
    \x03\x07\x11\n\r\n\x06\x04\0\x04\0\x02\0\x12\x03\x04\x04\x14\n\x0e\n\x07\
    \x04\0\x04\0\x02\0\x01\x12\x03\x04\x04\x0f\n\x0e\n\x07\x04\0\x04\0\x02\0\
    \x02\x12\x03\x04\x12\x13\n\r\n\x06\x04\0\x04\0\x02\x01\x12\x03\x05\x04\
    \x10\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03\x05\x04\x0b\n\x0e\n\x07\
    \x04\0\x04\0\x02\x01\x02\x12\x03\x05\x0e\x0f\n\r\n\x06\x04\0\x04\0\x02\
    \x02\x12\x03\x06\x04\x0e\n\x0e\n\x07\x04\0\x04\0\x02\x02\x01\x12\x03\x06\
    \x04\t\n\x0e\n\x07\x04\0\x04\0\x02\x02\x02\x12\x03\x06\x0c\r\n\x0b\n\x04\
    \x04\0\x02\0\x12\x03\x08\x02\x1d\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x08\
    \x02\x0c\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x08\r\x18\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\x08\x1b\x1c\n\x0b\n\x04\x04\0\x02\x01\x12\x03\t\x02\
    \x13\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\t\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03\t\t\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\t\x11\
    \x12\n\x0b\n\x04\x04\0\x02\x02\x12\x03\n\x02\x11\n\x0c\n\x05\x04\0\x02\
    \x02\x05\x12\x03\n\x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\n\x08\
    \x0c\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\n\x0f\x10b\x06proto3\
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
            messages.push(PacketWrapper::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(packet_wrapper::PacketType::generated_enum_descriptor_data());
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
