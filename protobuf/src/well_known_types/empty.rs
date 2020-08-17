// This file is generated by rust-protobuf 3.0.0-pre. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `google/protobuf/empty.proto`

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(serde, derive(Serialize, Deserialize))]
pub struct Empty {
    // special fields
    #[cfg_attr(serde, serde(skip))]
    pub unknown_fields: crate::UnknownFields,
    #[cfg_attr(serde, serde(skip))]
    pub cached_size: crate::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a Empty {
    fn default() -> &'a Empty {
        <Empty as crate::Message>::default_instance()
    }
}

impl Empty {
    pub fn new() -> Empty {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> crate::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::new();
        crate::reflect::GeneratedMessageDescriptorData::new_2::<Empty>(
            "Empty",
            0,
            fields,
        )
    }
}

impl crate::Message for Empty {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut crate::CodedInputStream<'_>) -> crate::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    crate::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += crate::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut crate::CodedOutputStream<'_>) -> crate::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &crate::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut crate::UnknownFields {
        &mut self.unknown_fields
    }

    fn new() -> Empty {
        Empty::new()
    }

    fn descriptor_static() -> crate::reflect::MessageDescriptor {
        crate::reflect::MessageDescriptor::new_generated_2(file_descriptor(), 0)
    }

    fn default_instance() -> &'static Empty {
        static instance: Empty = Empty {
            unknown_fields: crate::UnknownFields::new(),
            cached_size: crate::rt::CachedSize::new(),
        };
        &instance
    }
}

impl crate::Clear for Empty {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Empty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        crate::text_format::fmt(self, f)
    }
}

impl crate::reflect::ProtobufValueSized for Empty {
    type RuntimeType = crate::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgoogle/protobuf/empty.proto\x12\x0fgoogle.protobuf\"\x07\n\x05Empt\
    yBv\n\x13com.google.protobufB\nEmptyProtoP\x01Z'github.com/golang/protob\
    uf/ptypes/empty\xf8\x01\x01\xa2\x02\x03GPB\xaa\x02\x1eGoogle.Protobuf.We\
    llKnownTypesJ\xfe\x10\n\x06\x12\x04\x1e\03\x10\n\xcc\x0c\n\x01\x0c\x12\
    \x03\x1e\0\x122\xc1\x0c\x20Protocol\x20Buffers\x20-\x20Google's\x20data\
    \x20interchange\x20format\n\x20Copyright\x202008\x20Google\x20Inc.\x20\
    \x20All\x20rights\x20reserved.\n\x20https://developers.google.com/protoc\
    ol-buffers/\n\n\x20Redistribution\x20and\x20use\x20in\x20source\x20and\
    \x20binary\x20forms,\x20with\x20or\x20without\n\x20modification,\x20are\
    \x20permitted\x20provided\x20that\x20the\x20following\x20conditions\x20a\
    re\n\x20met:\n\n\x20\x20\x20\x20\x20*\x20Redistributions\x20of\x20source\
    \x20code\x20must\x20retain\x20the\x20above\x20copyright\n\x20notice,\x20\
    this\x20list\x20of\x20conditions\x20and\x20the\x20following\x20disclaime\
    r.\n\x20\x20\x20\x20\x20*\x20Redistributions\x20in\x20binary\x20form\x20\
    must\x20reproduce\x20the\x20above\n\x20copyright\x20notice,\x20this\x20l\
    ist\x20of\x20conditions\x20and\x20the\x20following\x20disclaimer\n\x20in\
    \x20the\x20documentation\x20and/or\x20other\x20materials\x20provided\x20\
    with\x20the\n\x20distribution.\n\x20\x20\x20\x20\x20*\x20Neither\x20the\
    \x20name\x20of\x20Google\x20Inc.\x20nor\x20the\x20names\x20of\x20its\n\
    \x20contributors\x20may\x20be\x20used\x20to\x20endorse\x20or\x20promote\
    \x20products\x20derived\x20from\n\x20this\x20software\x20without\x20spec\
    ific\x20prior\x20written\x20permission.\n\n\x20THIS\x20SOFTWARE\x20IS\
    \x20PROVIDED\x20BY\x20THE\x20COPYRIGHT\x20HOLDERS\x20AND\x20CONTRIBUTORS\
    \n\x20\"AS\x20IS\"\x20AND\x20ANY\x20EXPRESS\x20OR\x20IMPLIED\x20WARRANTI\
    ES,\x20INCLUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20THE\x20IMPLIED\
    \x20WARRANTIES\x20OF\x20MERCHANTABILITY\x20AND\x20FITNESS\x20FOR\n\x20A\
    \x20PARTICULAR\x20PURPOSE\x20ARE\x20DISCLAIMED.\x20IN\x20NO\x20EVENT\x20\
    SHALL\x20THE\x20COPYRIGHT\n\x20OWNER\x20OR\x20CONTRIBUTORS\x20BE\x20LIAB\
    LE\x20FOR\x20ANY\x20DIRECT,\x20INDIRECT,\x20INCIDENTAL,\n\x20SPECIAL,\
    \x20EXEMPLARY,\x20OR\x20CONSEQUENTIAL\x20DAMAGES\x20(INCLUDING,\x20BUT\
    \x20NOT\n\x20LIMITED\x20TO,\x20PROCUREMENT\x20OF\x20SUBSTITUTE\x20GOODS\
    \x20OR\x20SERVICES;\x20LOSS\x20OF\x20USE,\n\x20DATA,\x20OR\x20PROFITS;\
    \x20OR\x20BUSINESS\x20INTERRUPTION)\x20HOWEVER\x20CAUSED\x20AND\x20ON\
    \x20ANY\n\x20THEORY\x20OF\x20LIABILITY,\x20WHETHER\x20IN\x20CONTRACT,\
    \x20STRICT\x20LIABILITY,\x20OR\x20TORT\n\x20(INCLUDING\x20NEGLIGENCE\x20\
    OR\x20OTHERWISE)\x20ARISING\x20IN\x20ANY\x20WAY\x20OUT\x20OF\x20THE\x20U\
    SE\n\x20OF\x20THIS\x20SOFTWARE,\x20EVEN\x20IF\x20ADVISED\x20OF\x20THE\
    \x20POSSIBILITY\x20OF\x20SUCH\x20DAMAGE.\n\n\x08\n\x01\x02\x12\x03\x20\0\
    \x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\x03\"\0;\n\x08\n\
    \x01\x08\x12\x03#\0>\n\t\n\x02\x08\x0b\x12\x03#\0>\n\x08\n\x01\x08\x12\
    \x03$\0,\n\t\n\x02\x08\x01\x12\x03$\0,\n\x08\n\x01\x08\x12\x03%\0+\n\t\n\
    \x02\x08\x08\x12\x03%\0+\n\x08\n\x01\x08\x12\x03&\0\"\n\t\n\x02\x08\n\
    \x12\x03&\0\"\n\x08\n\x01\x08\x12\x03'\0!\n\t\n\x02\x08$\x12\x03'\0!\n\
    \x08\n\x01\x08\x12\x03(\0\x1f\n\t\n\x02\x08\x1f\x12\x03(\0\x1f\n\xfb\x02\
    \n\x02\x04\0\x12\x033\0\x10\x1a\xef\x02\x20A\x20generic\x20empty\x20mess\
    age\x20that\x20you\x20can\x20re-use\x20to\x20avoid\x20defining\x20duplic\
    ated\n\x20empty\x20messages\x20in\x20your\x20APIs.\x20A\x20typical\x20ex\
    ample\x20is\x20to\x20use\x20it\x20as\x20the\x20request\n\x20or\x20the\
    \x20response\x20type\x20of\x20an\x20API\x20method.\x20For\x20instance:\n\
    \n\x20\x20\x20\x20\x20service\x20Foo\x20{\n\x20\x20\x20\x20\x20\x20\x20r\
    pc\x20Bar(google.protobuf.Empty)\x20returns\x20(google.protobuf.Empty);\
    \n\x20\x20\x20\x20\x20}\n\n\x20The\x20JSON\x20representation\x20for\x20`\
    Empty`\x20is\x20empty\x20JSON\x20object\x20`{}`.\n\n\n\n\x03\x04\0\x01\
    \x12\x033\x08\rb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static crate::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: crate::rt::LazyV2<crate::descriptor::FileDescriptorProto> = crate::rt::LazyV2::INIT;
    file_descriptor_proto_lazy.get(|| {
        crate::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> crate::reflect::FileDescriptor {
    static file_descriptor_lazy: crate::rt::LazyV2<crate::reflect::GeneratedFileDescriptor> = crate::rt::LazyV2::INIT;
    let file_descriptor = file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::new();
        let mut messages = ::std::vec::Vec::new();
        messages.push(Empty::generated_message_descriptor_data());
        let mut enums = ::std::vec::Vec::new();
        crate::reflect::GeneratedFileDescriptor::new_generated(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    });
    crate::reflect::FileDescriptor::new_generated_2(file_descriptor)
}
