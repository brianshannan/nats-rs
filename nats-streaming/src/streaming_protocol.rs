// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct PubMsg {
    // message fields
    pub clientID: ::std::string::String,
    pub guid: ::std::string::String,
    pub subject: ::std::string::String,
    pub reply: ::std::string::String,
    pub data: ::std::vec::Vec<u8>,
    pub sha256: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PubMsg {}

impl PubMsg {
    pub fn new() -> PubMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PubMsg {
        static mut instance: ::protobuf::lazy::Lazy<PubMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PubMsg,
        };
        unsafe {
            instance.get(PubMsg::new)
        }
    }

    // string clientID = 1;

    pub fn clear_clientID(&mut self) {
        self.clientID.clear();
    }

    // Param is passed by value, moved
    pub fn set_clientID(&mut self, v: ::std::string::String) {
        self.clientID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientID(&mut self) -> &mut ::std::string::String {
        &mut self.clientID
    }

    // Take field
    pub fn take_clientID(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.clientID, ::std::string::String::new())
    }

    pub fn get_clientID(&self) -> &str {
        &self.clientID
    }

    fn get_clientID_for_reflect(&self) -> &::std::string::String {
        &self.clientID
    }

    fn mut_clientID_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.clientID
    }

    // string guid = 2;

    pub fn clear_guid(&mut self) {
        self.guid.clear();
    }

    // Param is passed by value, moved
    pub fn set_guid(&mut self, v: ::std::string::String) {
        self.guid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_guid(&mut self) -> &mut ::std::string::String {
        &mut self.guid
    }

    // Take field
    pub fn take_guid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.guid, ::std::string::String::new())
    }

    pub fn get_guid(&self) -> &str {
        &self.guid
    }

    fn get_guid_for_reflect(&self) -> &::std::string::String {
        &self.guid
    }

    fn mut_guid_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.guid
    }

    // string subject = 3;

    pub fn clear_subject(&mut self) {
        self.subject.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject(&mut self, v: ::std::string::String) {
        self.subject = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // Take field
    pub fn take_subject(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subject, ::std::string::String::new())
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    fn get_subject_for_reflect(&self) -> &::std::string::String {
        &self.subject
    }

    fn mut_subject_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // string reply = 4;

    pub fn clear_reply(&mut self) {
        self.reply.clear();
    }

    // Param is passed by value, moved
    pub fn set_reply(&mut self, v: ::std::string::String) {
        self.reply = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reply(&mut self) -> &mut ::std::string::String {
        &mut self.reply
    }

    // Take field
    pub fn take_reply(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.reply, ::std::string::String::new())
    }

    pub fn get_reply(&self) -> &str {
        &self.reply
    }

    fn get_reply_for_reflect(&self) -> &::std::string::String {
        &self.reply
    }

    fn mut_reply_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.reply
    }

    // bytes data = 5;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // bytes sha256 = 10;

    pub fn clear_sha256(&mut self) {
        self.sha256.clear();
    }

    // Param is passed by value, moved
    pub fn set_sha256(&mut self, v: ::std::vec::Vec<u8>) {
        self.sha256 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sha256(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.sha256
    }

    // Take field
    pub fn take_sha256(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.sha256, ::std::vec::Vec::new())
    }

    pub fn get_sha256(&self) -> &[u8] {
        &self.sha256
    }

    fn get_sha256_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.sha256
    }

    fn mut_sha256_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.sha256
    }
}

impl ::protobuf::Message for PubMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.clientID)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.guid)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subject)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.reply)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.sha256)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.clientID.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.clientID);
        };
        if !self.guid.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.guid);
        };
        if !self.subject.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.subject);
        };
        if !self.reply.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.reply);
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.data);
        };
        if !self.sha256.is_empty() {
            my_size += ::protobuf::rt::bytes_size(10, &self.sha256);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.clientID.is_empty() {
            os.write_string(1, &self.clientID)?;
        };
        if !self.guid.is_empty() {
            os.write_string(2, &self.guid)?;
        };
        if !self.subject.is_empty() {
            os.write_string(3, &self.subject)?;
        };
        if !self.reply.is_empty() {
            os.write_string(4, &self.reply)?;
        };
        if !self.data.is_empty() {
            os.write_bytes(5, &self.data)?;
        };
        if !self.sha256.is_empty() {
            os.write_bytes(10, &self.sha256)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PubMsg {
    fn new() -> PubMsg {
        PubMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<PubMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientID",
                    PubMsg::get_clientID_for_reflect,
                    PubMsg::mut_clientID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "guid",
                    PubMsg::get_guid_for_reflect,
                    PubMsg::mut_guid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subject",
                    PubMsg::get_subject_for_reflect,
                    PubMsg::mut_subject_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reply",
                    PubMsg::get_reply_for_reflect,
                    PubMsg::mut_reply_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    PubMsg::get_data_for_reflect,
                    PubMsg::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "sha256",
                    PubMsg::get_sha256_for_reflect,
                    PubMsg::mut_sha256_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PubMsg>(
                    "PubMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PubMsg {
    fn clear(&mut self) {
        self.clear_clientID();
        self.clear_guid();
        self.clear_subject();
        self.clear_reply();
        self.clear_data();
        self.clear_sha256();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PubMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PubMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PubAck {
    // message fields
    pub guid: ::std::string::String,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PubAck {}

impl PubAck {
    pub fn new() -> PubAck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PubAck {
        static mut instance: ::protobuf::lazy::Lazy<PubAck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PubAck,
        };
        unsafe {
            instance.get(PubAck::new)
        }
    }

    // string guid = 1;

    pub fn clear_guid(&mut self) {
        self.guid.clear();
    }

    // Param is passed by value, moved
    pub fn set_guid(&mut self, v: ::std::string::String) {
        self.guid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_guid(&mut self) -> &mut ::std::string::String {
        &mut self.guid
    }

    // Take field
    pub fn take_guid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.guid, ::std::string::String::new())
    }

    pub fn get_guid(&self) -> &str {
        &self.guid
    }

    fn get_guid_for_reflect(&self) -> &::std::string::String {
        &self.guid
    }

    fn mut_guid_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.guid
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for PubAck {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.guid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.guid.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.guid);
        };
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.guid.is_empty() {
            os.write_string(1, &self.guid)?;
        };
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PubAck {
    fn new() -> PubAck {
        PubAck::new()
    }

    fn descriptor_static(_: ::std::option::Option<PubAck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "guid",
                    PubAck::get_guid_for_reflect,
                    PubAck::mut_guid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    PubAck::get_error_for_reflect,
                    PubAck::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PubAck>(
                    "PubAck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PubAck {
    fn clear(&mut self) {
        self.clear_guid();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PubAck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PubAck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MsgProto {
    // message fields
    pub sequence: u64,
    pub subject: ::std::string::String,
    pub reply: ::std::string::String,
    pub data: ::std::vec::Vec<u8>,
    pub timestamp: i64,
    pub redelivered: bool,
    pub CRC32: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MsgProto {}

impl MsgProto {
    pub fn new() -> MsgProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MsgProto {
        static mut instance: ::protobuf::lazy::Lazy<MsgProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MsgProto,
        };
        unsafe {
            instance.get(MsgProto::new)
        }
    }

    // uint64 sequence = 1;

    pub fn clear_sequence(&mut self) {
        self.sequence = 0;
    }

    // Param is passed by value, moved
    pub fn set_sequence(&mut self, v: u64) {
        self.sequence = v;
    }

    pub fn get_sequence(&self) -> u64 {
        self.sequence
    }

    fn get_sequence_for_reflect(&self) -> &u64 {
        &self.sequence
    }

    fn mut_sequence_for_reflect(&mut self) -> &mut u64 {
        &mut self.sequence
    }

    // string subject = 2;

    pub fn clear_subject(&mut self) {
        self.subject.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject(&mut self, v: ::std::string::String) {
        self.subject = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // Take field
    pub fn take_subject(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subject, ::std::string::String::new())
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    fn get_subject_for_reflect(&self) -> &::std::string::String {
        &self.subject
    }

    fn mut_subject_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // string reply = 3;

    pub fn clear_reply(&mut self) {
        self.reply.clear();
    }

    // Param is passed by value, moved
    pub fn set_reply(&mut self, v: ::std::string::String) {
        self.reply = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reply(&mut self) -> &mut ::std::string::String {
        &mut self.reply
    }

    // Take field
    pub fn take_reply(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.reply, ::std::string::String::new())
    }

    pub fn get_reply(&self) -> &str {
        &self.reply
    }

    fn get_reply_for_reflect(&self) -> &::std::string::String {
        &self.reply
    }

    fn mut_reply_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.reply
    }

    // bytes data = 4;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // int64 timestamp = 5;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &i64 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut i64 {
        &mut self.timestamp
    }

    // bool redelivered = 6;

    pub fn clear_redelivered(&mut self) {
        self.redelivered = false;
    }

    // Param is passed by value, moved
    pub fn set_redelivered(&mut self, v: bool) {
        self.redelivered = v;
    }

    pub fn get_redelivered(&self) -> bool {
        self.redelivered
    }

    fn get_redelivered_for_reflect(&self) -> &bool {
        &self.redelivered
    }

    fn mut_redelivered_for_reflect(&mut self) -> &mut bool {
        &mut self.redelivered
    }

    // uint32 CRC32 = 10;

    pub fn clear_CRC32(&mut self) {
        self.CRC32 = 0;
    }

    // Param is passed by value, moved
    pub fn set_CRC32(&mut self, v: u32) {
        self.CRC32 = v;
    }

    pub fn get_CRC32(&self) -> u32 {
        self.CRC32
    }

    fn get_CRC32_for_reflect(&self) -> &u32 {
        &self.CRC32
    }

    fn mut_CRC32_for_reflect(&mut self) -> &mut u32 {
        &mut self.CRC32
    }
}

impl ::protobuf::Message for MsgProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.sequence = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subject)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.reply)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.timestamp = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.redelivered = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.CRC32 = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.sequence != 0 {
            my_size += ::protobuf::rt::value_size(1, self.sequence, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.subject.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.subject);
        };
        if !self.reply.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.reply);
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.data);
        };
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(5, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.redelivered != false {
            my_size += 2;
        };
        if self.CRC32 != 0 {
            my_size += ::protobuf::rt::value_size(10, self.CRC32, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.sequence != 0 {
            os.write_uint64(1, self.sequence)?;
        };
        if !self.subject.is_empty() {
            os.write_string(2, &self.subject)?;
        };
        if !self.reply.is_empty() {
            os.write_string(3, &self.reply)?;
        };
        if !self.data.is_empty() {
            os.write_bytes(4, &self.data)?;
        };
        if self.timestamp != 0 {
            os.write_int64(5, self.timestamp)?;
        };
        if self.redelivered != false {
            os.write_bool(6, self.redelivered)?;
        };
        if self.CRC32 != 0 {
            os.write_uint32(10, self.CRC32)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MsgProto {
    fn new() -> MsgProto {
        MsgProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<MsgProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sequence",
                    MsgProto::get_sequence_for_reflect,
                    MsgProto::mut_sequence_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subject",
                    MsgProto::get_subject_for_reflect,
                    MsgProto::mut_subject_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reply",
                    MsgProto::get_reply_for_reflect,
                    MsgProto::mut_reply_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    MsgProto::get_data_for_reflect,
                    MsgProto::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    MsgProto::get_timestamp_for_reflect,
                    MsgProto::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "redelivered",
                    MsgProto::get_redelivered_for_reflect,
                    MsgProto::mut_redelivered_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "CRC32",
                    MsgProto::get_CRC32_for_reflect,
                    MsgProto::mut_CRC32_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MsgProto>(
                    "MsgProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MsgProto {
    fn clear(&mut self) {
        self.clear_sequence();
        self.clear_subject();
        self.clear_reply();
        self.clear_data();
        self.clear_timestamp();
        self.clear_redelivered();
        self.clear_CRC32();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MsgProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Ack {
    // message fields
    pub subject: ::std::string::String,
    pub sequence: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Ack {}

impl Ack {
    pub fn new() -> Ack {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Ack {
        static mut instance: ::protobuf::lazy::Lazy<Ack> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Ack,
        };
        unsafe {
            instance.get(Ack::new)
        }
    }

    // string subject = 1;

    pub fn clear_subject(&mut self) {
        self.subject.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject(&mut self, v: ::std::string::String) {
        self.subject = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // Take field
    pub fn take_subject(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subject, ::std::string::String::new())
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    fn get_subject_for_reflect(&self) -> &::std::string::String {
        &self.subject
    }

    fn mut_subject_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // uint64 sequence = 2;

    pub fn clear_sequence(&mut self) {
        self.sequence = 0;
    }

    // Param is passed by value, moved
    pub fn set_sequence(&mut self, v: u64) {
        self.sequence = v;
    }

    pub fn get_sequence(&self) -> u64 {
        self.sequence
    }

    fn get_sequence_for_reflect(&self) -> &u64 {
        &self.sequence
    }

    fn mut_sequence_for_reflect(&mut self) -> &mut u64 {
        &mut self.sequence
    }
}

impl ::protobuf::Message for Ack {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subject)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.sequence = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.subject.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.subject);
        };
        if self.sequence != 0 {
            my_size += ::protobuf::rt::value_size(2, self.sequence, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.subject.is_empty() {
            os.write_string(1, &self.subject)?;
        };
        if self.sequence != 0 {
            os.write_uint64(2, self.sequence)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Ack {
    fn new() -> Ack {
        Ack::new()
    }

    fn descriptor_static(_: ::std::option::Option<Ack>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subject",
                    Ack::get_subject_for_reflect,
                    Ack::mut_subject_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sequence",
                    Ack::get_sequence_for_reflect,
                    Ack::mut_sequence_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Ack>(
                    "Ack",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Ack {
    fn clear(&mut self) {
        self.clear_subject();
        self.clear_sequence();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Ack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Ack {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConnectRequest {
    // message fields
    pub clientID: ::std::string::String,
    pub heartbeatInbox: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConnectRequest {}

impl ConnectRequest {
    pub fn new() -> ConnectRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConnectRequest {
        static mut instance: ::protobuf::lazy::Lazy<ConnectRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConnectRequest,
        };
        unsafe {
            instance.get(ConnectRequest::new)
        }
    }

    // string clientID = 1;

    pub fn clear_clientID(&mut self) {
        self.clientID.clear();
    }

    // Param is passed by value, moved
    pub fn set_clientID(&mut self, v: ::std::string::String) {
        self.clientID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientID(&mut self) -> &mut ::std::string::String {
        &mut self.clientID
    }

    // Take field
    pub fn take_clientID(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.clientID, ::std::string::String::new())
    }

    pub fn get_clientID(&self) -> &str {
        &self.clientID
    }

    fn get_clientID_for_reflect(&self) -> &::std::string::String {
        &self.clientID
    }

    fn mut_clientID_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.clientID
    }

    // string heartbeatInbox = 2;

    pub fn clear_heartbeatInbox(&mut self) {
        self.heartbeatInbox.clear();
    }

    // Param is passed by value, moved
    pub fn set_heartbeatInbox(&mut self, v: ::std::string::String) {
        self.heartbeatInbox = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_heartbeatInbox(&mut self) -> &mut ::std::string::String {
        &mut self.heartbeatInbox
    }

    // Take field
    pub fn take_heartbeatInbox(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.heartbeatInbox, ::std::string::String::new())
    }

    pub fn get_heartbeatInbox(&self) -> &str {
        &self.heartbeatInbox
    }

    fn get_heartbeatInbox_for_reflect(&self) -> &::std::string::String {
        &self.heartbeatInbox
    }

    fn mut_heartbeatInbox_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.heartbeatInbox
    }
}

impl ::protobuf::Message for ConnectRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.clientID)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.heartbeatInbox)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.clientID.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.clientID);
        };
        if !self.heartbeatInbox.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.heartbeatInbox);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.clientID.is_empty() {
            os.write_string(1, &self.clientID)?;
        };
        if !self.heartbeatInbox.is_empty() {
            os.write_string(2, &self.heartbeatInbox)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConnectRequest {
    fn new() -> ConnectRequest {
        ConnectRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConnectRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientID",
                    ConnectRequest::get_clientID_for_reflect,
                    ConnectRequest::mut_clientID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "heartbeatInbox",
                    ConnectRequest::get_heartbeatInbox_for_reflect,
                    ConnectRequest::mut_heartbeatInbox_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConnectRequest>(
                    "ConnectRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConnectRequest {
    fn clear(&mut self) {
        self.clear_clientID();
        self.clear_heartbeatInbox();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConnectRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConnectRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConnectResponse {
    // message fields
    pub pubPrefix: ::std::string::String,
    pub subRequests: ::std::string::String,
    pub unsubRequests: ::std::string::String,
    pub closeRequests: ::std::string::String,
    pub error: ::std::string::String,
    pub subCloseRequests: ::std::string::String,
    pub publicKey: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConnectResponse {}

impl ConnectResponse {
    pub fn new() -> ConnectResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConnectResponse {
        static mut instance: ::protobuf::lazy::Lazy<ConnectResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConnectResponse,
        };
        unsafe {
            instance.get(ConnectResponse::new)
        }
    }

    // string pubPrefix = 1;

    pub fn clear_pubPrefix(&mut self) {
        self.pubPrefix.clear();
    }

    // Param is passed by value, moved
    pub fn set_pubPrefix(&mut self, v: ::std::string::String) {
        self.pubPrefix = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pubPrefix(&mut self) -> &mut ::std::string::String {
        &mut self.pubPrefix
    }

    // Take field
    pub fn take_pubPrefix(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pubPrefix, ::std::string::String::new())
    }

    pub fn get_pubPrefix(&self) -> &str {
        &self.pubPrefix
    }

    fn get_pubPrefix_for_reflect(&self) -> &::std::string::String {
        &self.pubPrefix
    }

    fn mut_pubPrefix_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pubPrefix
    }

    // string subRequests = 2;

    pub fn clear_subRequests(&mut self) {
        self.subRequests.clear();
    }

    // Param is passed by value, moved
    pub fn set_subRequests(&mut self, v: ::std::string::String) {
        self.subRequests = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subRequests(&mut self) -> &mut ::std::string::String {
        &mut self.subRequests
    }

    // Take field
    pub fn take_subRequests(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subRequests, ::std::string::String::new())
    }

    pub fn get_subRequests(&self) -> &str {
        &self.subRequests
    }

    fn get_subRequests_for_reflect(&self) -> &::std::string::String {
        &self.subRequests
    }

    fn mut_subRequests_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.subRequests
    }

    // string unsubRequests = 3;

    pub fn clear_unsubRequests(&mut self) {
        self.unsubRequests.clear();
    }

    // Param is passed by value, moved
    pub fn set_unsubRequests(&mut self, v: ::std::string::String) {
        self.unsubRequests = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unsubRequests(&mut self) -> &mut ::std::string::String {
        &mut self.unsubRequests
    }

    // Take field
    pub fn take_unsubRequests(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.unsubRequests, ::std::string::String::new())
    }

    pub fn get_unsubRequests(&self) -> &str {
        &self.unsubRequests
    }

    fn get_unsubRequests_for_reflect(&self) -> &::std::string::String {
        &self.unsubRequests
    }

    fn mut_unsubRequests_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.unsubRequests
    }

    // string closeRequests = 4;

    pub fn clear_closeRequests(&mut self) {
        self.closeRequests.clear();
    }

    // Param is passed by value, moved
    pub fn set_closeRequests(&mut self, v: ::std::string::String) {
        self.closeRequests = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_closeRequests(&mut self) -> &mut ::std::string::String {
        &mut self.closeRequests
    }

    // Take field
    pub fn take_closeRequests(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.closeRequests, ::std::string::String::new())
    }

    pub fn get_closeRequests(&self) -> &str {
        &self.closeRequests
    }

    fn get_closeRequests_for_reflect(&self) -> &::std::string::String {
        &self.closeRequests
    }

    fn mut_closeRequests_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.closeRequests
    }

    // string error = 5;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // string subCloseRequests = 6;

    pub fn clear_subCloseRequests(&mut self) {
        self.subCloseRequests.clear();
    }

    // Param is passed by value, moved
    pub fn set_subCloseRequests(&mut self, v: ::std::string::String) {
        self.subCloseRequests = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subCloseRequests(&mut self) -> &mut ::std::string::String {
        &mut self.subCloseRequests
    }

    // Take field
    pub fn take_subCloseRequests(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subCloseRequests, ::std::string::String::new())
    }

    pub fn get_subCloseRequests(&self) -> &str {
        &self.subCloseRequests
    }

    fn get_subCloseRequests_for_reflect(&self) -> &::std::string::String {
        &self.subCloseRequests
    }

    fn mut_subCloseRequests_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.subCloseRequests
    }

    // string publicKey = 100;

    pub fn clear_publicKey(&mut self) {
        self.publicKey.clear();
    }

    // Param is passed by value, moved
    pub fn set_publicKey(&mut self, v: ::std::string::String) {
        self.publicKey = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_publicKey(&mut self) -> &mut ::std::string::String {
        &mut self.publicKey
    }

    // Take field
    pub fn take_publicKey(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.publicKey, ::std::string::String::new())
    }

    pub fn get_publicKey(&self) -> &str {
        &self.publicKey
    }

    fn get_publicKey_for_reflect(&self) -> &::std::string::String {
        &self.publicKey
    }

    fn mut_publicKey_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.publicKey
    }
}

impl ::protobuf::Message for ConnectResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pubPrefix)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subRequests)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.unsubRequests)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.closeRequests)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subCloseRequests)?;
                },
                100 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.publicKey)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.pubPrefix.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.pubPrefix);
        };
        if !self.subRequests.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.subRequests);
        };
        if !self.unsubRequests.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.unsubRequests);
        };
        if !self.closeRequests.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.closeRequests);
        };
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.error);
        };
        if !self.subCloseRequests.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.subCloseRequests);
        };
        if !self.publicKey.is_empty() {
            my_size += ::protobuf::rt::string_size(100, &self.publicKey);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pubPrefix.is_empty() {
            os.write_string(1, &self.pubPrefix)?;
        };
        if !self.subRequests.is_empty() {
            os.write_string(2, &self.subRequests)?;
        };
        if !self.unsubRequests.is_empty() {
            os.write_string(3, &self.unsubRequests)?;
        };
        if !self.closeRequests.is_empty() {
            os.write_string(4, &self.closeRequests)?;
        };
        if !self.error.is_empty() {
            os.write_string(5, &self.error)?;
        };
        if !self.subCloseRequests.is_empty() {
            os.write_string(6, &self.subCloseRequests)?;
        };
        if !self.publicKey.is_empty() {
            os.write_string(100, &self.publicKey)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConnectResponse {
    fn new() -> ConnectResponse {
        ConnectResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConnectResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pubPrefix",
                    ConnectResponse::get_pubPrefix_for_reflect,
                    ConnectResponse::mut_pubPrefix_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subRequests",
                    ConnectResponse::get_subRequests_for_reflect,
                    ConnectResponse::mut_subRequests_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "unsubRequests",
                    ConnectResponse::get_unsubRequests_for_reflect,
                    ConnectResponse::mut_unsubRequests_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "closeRequests",
                    ConnectResponse::get_closeRequests_for_reflect,
                    ConnectResponse::mut_closeRequests_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    ConnectResponse::get_error_for_reflect,
                    ConnectResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subCloseRequests",
                    ConnectResponse::get_subCloseRequests_for_reflect,
                    ConnectResponse::mut_subCloseRequests_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "publicKey",
                    ConnectResponse::get_publicKey_for_reflect,
                    ConnectResponse::mut_publicKey_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConnectResponse>(
                    "ConnectResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConnectResponse {
    fn clear(&mut self) {
        self.clear_pubPrefix();
        self.clear_subRequests();
        self.clear_unsubRequests();
        self.clear_closeRequests();
        self.clear_error();
        self.clear_subCloseRequests();
        self.clear_publicKey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConnectResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConnectResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SubscriptionRequest {
    // message fields
    pub clientID: ::std::string::String,
    pub subject: ::std::string::String,
    pub qGroup: ::std::string::String,
    pub inbox: ::std::string::String,
    pub maxInFlight: i32,
    pub ackWaitInSecs: i32,
    pub durableName: ::std::string::String,
    pub startPosition: StartPosition,
    pub startSequence: u64,
    pub startTimeDelta: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SubscriptionRequest {}

impl SubscriptionRequest {
    pub fn new() -> SubscriptionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubscriptionRequest {
        static mut instance: ::protobuf::lazy::Lazy<SubscriptionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubscriptionRequest,
        };
        unsafe {
            instance.get(SubscriptionRequest::new)
        }
    }

    // string clientID = 1;

    pub fn clear_clientID(&mut self) {
        self.clientID.clear();
    }

    // Param is passed by value, moved
    pub fn set_clientID(&mut self, v: ::std::string::String) {
        self.clientID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientID(&mut self) -> &mut ::std::string::String {
        &mut self.clientID
    }

    // Take field
    pub fn take_clientID(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.clientID, ::std::string::String::new())
    }

    pub fn get_clientID(&self) -> &str {
        &self.clientID
    }

    fn get_clientID_for_reflect(&self) -> &::std::string::String {
        &self.clientID
    }

    fn mut_clientID_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.clientID
    }

    // string subject = 2;

    pub fn clear_subject(&mut self) {
        self.subject.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject(&mut self, v: ::std::string::String) {
        self.subject = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // Take field
    pub fn take_subject(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subject, ::std::string::String::new())
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    fn get_subject_for_reflect(&self) -> &::std::string::String {
        &self.subject
    }

    fn mut_subject_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // string qGroup = 3;

    pub fn clear_qGroup(&mut self) {
        self.qGroup.clear();
    }

    // Param is passed by value, moved
    pub fn set_qGroup(&mut self, v: ::std::string::String) {
        self.qGroup = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_qGroup(&mut self) -> &mut ::std::string::String {
        &mut self.qGroup
    }

    // Take field
    pub fn take_qGroup(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.qGroup, ::std::string::String::new())
    }

    pub fn get_qGroup(&self) -> &str {
        &self.qGroup
    }

    fn get_qGroup_for_reflect(&self) -> &::std::string::String {
        &self.qGroup
    }

    fn mut_qGroup_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.qGroup
    }

    // string inbox = 4;

    pub fn clear_inbox(&mut self) {
        self.inbox.clear();
    }

    // Param is passed by value, moved
    pub fn set_inbox(&mut self, v: ::std::string::String) {
        self.inbox = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inbox(&mut self) -> &mut ::std::string::String {
        &mut self.inbox
    }

    // Take field
    pub fn take_inbox(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.inbox, ::std::string::String::new())
    }

    pub fn get_inbox(&self) -> &str {
        &self.inbox
    }

    fn get_inbox_for_reflect(&self) -> &::std::string::String {
        &self.inbox
    }

    fn mut_inbox_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.inbox
    }

    // int32 maxInFlight = 5;

    pub fn clear_maxInFlight(&mut self) {
        self.maxInFlight = 0;
    }

    // Param is passed by value, moved
    pub fn set_maxInFlight(&mut self, v: i32) {
        self.maxInFlight = v;
    }

    pub fn get_maxInFlight(&self) -> i32 {
        self.maxInFlight
    }

    fn get_maxInFlight_for_reflect(&self) -> &i32 {
        &self.maxInFlight
    }

    fn mut_maxInFlight_for_reflect(&mut self) -> &mut i32 {
        &mut self.maxInFlight
    }

    // int32 ackWaitInSecs = 6;

    pub fn clear_ackWaitInSecs(&mut self) {
        self.ackWaitInSecs = 0;
    }

    // Param is passed by value, moved
    pub fn set_ackWaitInSecs(&mut self, v: i32) {
        self.ackWaitInSecs = v;
    }

    pub fn get_ackWaitInSecs(&self) -> i32 {
        self.ackWaitInSecs
    }

    fn get_ackWaitInSecs_for_reflect(&self) -> &i32 {
        &self.ackWaitInSecs
    }

    fn mut_ackWaitInSecs_for_reflect(&mut self) -> &mut i32 {
        &mut self.ackWaitInSecs
    }

    // string durableName = 7;

    pub fn clear_durableName(&mut self) {
        self.durableName.clear();
    }

    // Param is passed by value, moved
    pub fn set_durableName(&mut self, v: ::std::string::String) {
        self.durableName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_durableName(&mut self) -> &mut ::std::string::String {
        &mut self.durableName
    }

    // Take field
    pub fn take_durableName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.durableName, ::std::string::String::new())
    }

    pub fn get_durableName(&self) -> &str {
        &self.durableName
    }

    fn get_durableName_for_reflect(&self) -> &::std::string::String {
        &self.durableName
    }

    fn mut_durableName_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.durableName
    }

    // .pb.StartPosition startPosition = 10;

    pub fn clear_startPosition(&mut self) {
        self.startPosition = StartPosition::NewOnly;
    }

    // Param is passed by value, moved
    pub fn set_startPosition(&mut self, v: StartPosition) {
        self.startPosition = v;
    }

    pub fn get_startPosition(&self) -> StartPosition {
        self.startPosition
    }

    fn get_startPosition_for_reflect(&self) -> &StartPosition {
        &self.startPosition
    }

    fn mut_startPosition_for_reflect(&mut self) -> &mut StartPosition {
        &mut self.startPosition
    }

    // uint64 startSequence = 11;

    pub fn clear_startSequence(&mut self) {
        self.startSequence = 0;
    }

    // Param is passed by value, moved
    pub fn set_startSequence(&mut self, v: u64) {
        self.startSequence = v;
    }

    pub fn get_startSequence(&self) -> u64 {
        self.startSequence
    }

    fn get_startSequence_for_reflect(&self) -> &u64 {
        &self.startSequence
    }

    fn mut_startSequence_for_reflect(&mut self) -> &mut u64 {
        &mut self.startSequence
    }

    // int64 startTimeDelta = 12;

    pub fn clear_startTimeDelta(&mut self) {
        self.startTimeDelta = 0;
    }

    // Param is passed by value, moved
    pub fn set_startTimeDelta(&mut self, v: i64) {
        self.startTimeDelta = v;
    }

    pub fn get_startTimeDelta(&self) -> i64 {
        self.startTimeDelta
    }

    fn get_startTimeDelta_for_reflect(&self) -> &i64 {
        &self.startTimeDelta
    }

    fn mut_startTimeDelta_for_reflect(&mut self) -> &mut i64 {
        &mut self.startTimeDelta
    }
}

impl ::protobuf::Message for SubscriptionRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.clientID)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subject)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.qGroup)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.inbox)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.maxInFlight = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.ackWaitInSecs = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.durableName)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.startPosition = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.startSequence = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.startTimeDelta = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.clientID.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.clientID);
        };
        if !self.subject.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.subject);
        };
        if !self.qGroup.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.qGroup);
        };
        if !self.inbox.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.inbox);
        };
        if self.maxInFlight != 0 {
            my_size += ::protobuf::rt::value_size(5, self.maxInFlight, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.ackWaitInSecs != 0 {
            my_size += ::protobuf::rt::value_size(6, self.ackWaitInSecs, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.durableName.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.durableName);
        };
        if self.startPosition != StartPosition::NewOnly {
            my_size += ::protobuf::rt::enum_size(10, self.startPosition);
        };
        if self.startSequence != 0 {
            my_size += ::protobuf::rt::value_size(11, self.startSequence, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.startTimeDelta != 0 {
            my_size += ::protobuf::rt::value_size(12, self.startTimeDelta, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.clientID.is_empty() {
            os.write_string(1, &self.clientID)?;
        };
        if !self.subject.is_empty() {
            os.write_string(2, &self.subject)?;
        };
        if !self.qGroup.is_empty() {
            os.write_string(3, &self.qGroup)?;
        };
        if !self.inbox.is_empty() {
            os.write_string(4, &self.inbox)?;
        };
        if self.maxInFlight != 0 {
            os.write_int32(5, self.maxInFlight)?;
        };
        if self.ackWaitInSecs != 0 {
            os.write_int32(6, self.ackWaitInSecs)?;
        };
        if !self.durableName.is_empty() {
            os.write_string(7, &self.durableName)?;
        };
        if self.startPosition != StartPosition::NewOnly {
            os.write_enum(10, self.startPosition.value())?;
        };
        if self.startSequence != 0 {
            os.write_uint64(11, self.startSequence)?;
        };
        if self.startTimeDelta != 0 {
            os.write_int64(12, self.startTimeDelta)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SubscriptionRequest {
    fn new() -> SubscriptionRequest {
        SubscriptionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubscriptionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientID",
                    SubscriptionRequest::get_clientID_for_reflect,
                    SubscriptionRequest::mut_clientID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subject",
                    SubscriptionRequest::get_subject_for_reflect,
                    SubscriptionRequest::mut_subject_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "qGroup",
                    SubscriptionRequest::get_qGroup_for_reflect,
                    SubscriptionRequest::mut_qGroup_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "inbox",
                    SubscriptionRequest::get_inbox_for_reflect,
                    SubscriptionRequest::mut_inbox_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "maxInFlight",
                    SubscriptionRequest::get_maxInFlight_for_reflect,
                    SubscriptionRequest::mut_maxInFlight_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ackWaitInSecs",
                    SubscriptionRequest::get_ackWaitInSecs_for_reflect,
                    SubscriptionRequest::mut_ackWaitInSecs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "durableName",
                    SubscriptionRequest::get_durableName_for_reflect,
                    SubscriptionRequest::mut_durableName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StartPosition>>(
                    "startPosition",
                    SubscriptionRequest::get_startPosition_for_reflect,
                    SubscriptionRequest::mut_startPosition_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "startSequence",
                    SubscriptionRequest::get_startSequence_for_reflect,
                    SubscriptionRequest::mut_startSequence_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "startTimeDelta",
                    SubscriptionRequest::get_startTimeDelta_for_reflect,
                    SubscriptionRequest::mut_startTimeDelta_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubscriptionRequest>(
                    "SubscriptionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubscriptionRequest {
    fn clear(&mut self) {
        self.clear_clientID();
        self.clear_subject();
        self.clear_qGroup();
        self.clear_inbox();
        self.clear_maxInFlight();
        self.clear_ackWaitInSecs();
        self.clear_durableName();
        self.clear_startPosition();
        self.clear_startSequence();
        self.clear_startTimeDelta();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubscriptionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubscriptionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SubscriptionResponse {
    // message fields
    pub ackInbox: ::std::string::String,
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SubscriptionResponse {}

impl SubscriptionResponse {
    pub fn new() -> SubscriptionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubscriptionResponse {
        static mut instance: ::protobuf::lazy::Lazy<SubscriptionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubscriptionResponse,
        };
        unsafe {
            instance.get(SubscriptionResponse::new)
        }
    }

    // string ackInbox = 2;

    pub fn clear_ackInbox(&mut self) {
        self.ackInbox.clear();
    }

    // Param is passed by value, moved
    pub fn set_ackInbox(&mut self, v: ::std::string::String) {
        self.ackInbox = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ackInbox(&mut self) -> &mut ::std::string::String {
        &mut self.ackInbox
    }

    // Take field
    pub fn take_ackInbox(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ackInbox, ::std::string::String::new())
    }

    pub fn get_ackInbox(&self) -> &str {
        &self.ackInbox
    }

    fn get_ackInbox_for_reflect(&self) -> &::std::string::String {
        &self.ackInbox
    }

    fn mut_ackInbox_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.ackInbox
    }

    // string error = 3;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for SubscriptionResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ackInbox)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.ackInbox.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.ackInbox);
        };
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.error);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.ackInbox.is_empty() {
            os.write_string(2, &self.ackInbox)?;
        };
        if !self.error.is_empty() {
            os.write_string(3, &self.error)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SubscriptionResponse {
    fn new() -> SubscriptionResponse {
        SubscriptionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubscriptionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ackInbox",
                    SubscriptionResponse::get_ackInbox_for_reflect,
                    SubscriptionResponse::mut_ackInbox_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    SubscriptionResponse::get_error_for_reflect,
                    SubscriptionResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubscriptionResponse>(
                    "SubscriptionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubscriptionResponse {
    fn clear(&mut self) {
        self.clear_ackInbox();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubscriptionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubscriptionResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnsubscribeRequest {
    // message fields
    pub clientID: ::std::string::String,
    pub subject: ::std::string::String,
    pub inbox: ::std::string::String,
    pub durableName: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnsubscribeRequest {}

impl UnsubscribeRequest {
    pub fn new() -> UnsubscribeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnsubscribeRequest {
        static mut instance: ::protobuf::lazy::Lazy<UnsubscribeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnsubscribeRequest,
        };
        unsafe {
            instance.get(UnsubscribeRequest::new)
        }
    }

    // string clientID = 1;

    pub fn clear_clientID(&mut self) {
        self.clientID.clear();
    }

    // Param is passed by value, moved
    pub fn set_clientID(&mut self, v: ::std::string::String) {
        self.clientID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientID(&mut self) -> &mut ::std::string::String {
        &mut self.clientID
    }

    // Take field
    pub fn take_clientID(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.clientID, ::std::string::String::new())
    }

    pub fn get_clientID(&self) -> &str {
        &self.clientID
    }

    fn get_clientID_for_reflect(&self) -> &::std::string::String {
        &self.clientID
    }

    fn mut_clientID_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.clientID
    }

    // string subject = 2;

    pub fn clear_subject(&mut self) {
        self.subject.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject(&mut self, v: ::std::string::String) {
        self.subject = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // Take field
    pub fn take_subject(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subject, ::std::string::String::new())
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    fn get_subject_for_reflect(&self) -> &::std::string::String {
        &self.subject
    }

    fn mut_subject_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // string inbox = 3;

    pub fn clear_inbox(&mut self) {
        self.inbox.clear();
    }

    // Param is passed by value, moved
    pub fn set_inbox(&mut self, v: ::std::string::String) {
        self.inbox = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inbox(&mut self) -> &mut ::std::string::String {
        &mut self.inbox
    }

    // Take field
    pub fn take_inbox(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.inbox, ::std::string::String::new())
    }

    pub fn get_inbox(&self) -> &str {
        &self.inbox
    }

    fn get_inbox_for_reflect(&self) -> &::std::string::String {
        &self.inbox
    }

    fn mut_inbox_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.inbox
    }

    // string durableName = 4;

    pub fn clear_durableName(&mut self) {
        self.durableName.clear();
    }

    // Param is passed by value, moved
    pub fn set_durableName(&mut self, v: ::std::string::String) {
        self.durableName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_durableName(&mut self) -> &mut ::std::string::String {
        &mut self.durableName
    }

    // Take field
    pub fn take_durableName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.durableName, ::std::string::String::new())
    }

    pub fn get_durableName(&self) -> &str {
        &self.durableName
    }

    fn get_durableName_for_reflect(&self) -> &::std::string::String {
        &self.durableName
    }

    fn mut_durableName_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.durableName
    }
}

impl ::protobuf::Message for UnsubscribeRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.clientID)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subject)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.inbox)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.durableName)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.clientID.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.clientID);
        };
        if !self.subject.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.subject);
        };
        if !self.inbox.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.inbox);
        };
        if !self.durableName.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.durableName);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.clientID.is_empty() {
            os.write_string(1, &self.clientID)?;
        };
        if !self.subject.is_empty() {
            os.write_string(2, &self.subject)?;
        };
        if !self.inbox.is_empty() {
            os.write_string(3, &self.inbox)?;
        };
        if !self.durableName.is_empty() {
            os.write_string(4, &self.durableName)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UnsubscribeRequest {
    fn new() -> UnsubscribeRequest {
        UnsubscribeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnsubscribeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientID",
                    UnsubscribeRequest::get_clientID_for_reflect,
                    UnsubscribeRequest::mut_clientID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subject",
                    UnsubscribeRequest::get_subject_for_reflect,
                    UnsubscribeRequest::mut_subject_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "inbox",
                    UnsubscribeRequest::get_inbox_for_reflect,
                    UnsubscribeRequest::mut_inbox_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "durableName",
                    UnsubscribeRequest::get_durableName_for_reflect,
                    UnsubscribeRequest::mut_durableName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnsubscribeRequest>(
                    "UnsubscribeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnsubscribeRequest {
    fn clear(&mut self) {
        self.clear_clientID();
        self.clear_subject();
        self.clear_inbox();
        self.clear_durableName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnsubscribeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnsubscribeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CloseRequest {
    // message fields
    pub clientID: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CloseRequest {}

impl CloseRequest {
    pub fn new() -> CloseRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CloseRequest {
        static mut instance: ::protobuf::lazy::Lazy<CloseRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CloseRequest,
        };
        unsafe {
            instance.get(CloseRequest::new)
        }
    }

    // string clientID = 1;

    pub fn clear_clientID(&mut self) {
        self.clientID.clear();
    }

    // Param is passed by value, moved
    pub fn set_clientID(&mut self, v: ::std::string::String) {
        self.clientID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientID(&mut self) -> &mut ::std::string::String {
        &mut self.clientID
    }

    // Take field
    pub fn take_clientID(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.clientID, ::std::string::String::new())
    }

    pub fn get_clientID(&self) -> &str {
        &self.clientID
    }

    fn get_clientID_for_reflect(&self) -> &::std::string::String {
        &self.clientID
    }

    fn mut_clientID_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.clientID
    }
}

impl ::protobuf::Message for CloseRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.clientID)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.clientID.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.clientID);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.clientID.is_empty() {
            os.write_string(1, &self.clientID)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CloseRequest {
    fn new() -> CloseRequest {
        CloseRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CloseRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientID",
                    CloseRequest::get_clientID_for_reflect,
                    CloseRequest::mut_clientID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CloseRequest>(
                    "CloseRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CloseRequest {
    fn clear(&mut self) {
        self.clear_clientID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CloseRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CloseRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CloseResponse {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CloseResponse {}

impl CloseResponse {
    pub fn new() -> CloseResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CloseResponse {
        static mut instance: ::protobuf::lazy::Lazy<CloseResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CloseResponse,
        };
        unsafe {
            instance.get(CloseResponse::new)
        }
    }

    // string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for CloseResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CloseResponse {
    fn new() -> CloseResponse {
        CloseResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CloseResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    CloseResponse::get_error_for_reflect,
                    CloseResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CloseResponse>(
                    "CloseResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CloseResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CloseResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CloseResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum StartPosition {
    NewOnly = 0,
    LastReceived = 1,
    TimeDeltaStart = 2,
    SequenceStart = 3,
    First = 4,
}

impl ::protobuf::ProtobufEnum for StartPosition {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StartPosition> {
        match value {
            0 => ::std::option::Option::Some(StartPosition::NewOnly),
            1 => ::std::option::Option::Some(StartPosition::LastReceived),
            2 => ::std::option::Option::Some(StartPosition::TimeDeltaStart),
            3 => ::std::option::Option::Some(StartPosition::SequenceStart),
            4 => ::std::option::Option::Some(StartPosition::First),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [StartPosition] = &[
            StartPosition::NewOnly,
            StartPosition::LastReceived,
            StartPosition::TimeDeltaStart,
            StartPosition::SequenceStart,
            StartPosition::First,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<StartPosition>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("StartPosition", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for StartPosition {
}

impl ::std::default::Default for StartPosition {
    fn default() -> Self {
        StartPosition::NewOnly
    }
}

impl ::protobuf::reflect::ProtobufValue for StartPosition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x2d, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x02, 0x70, 0x62, 0x22, 0x94,
    0x01, 0x0a, 0x06, 0x50, 0x75, 0x62, 0x4d, 0x73, 0x67, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x49, 0x44, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x49, 0x44, 0x12, 0x12, 0x0a, 0x04, 0x67, 0x75, 0x69, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x04, 0x67, 0x75, 0x69, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x73, 0x75, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x05, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x16, 0x0a,
    0x06, 0x73, 0x68, 0x61, 0x32, 0x35, 0x36, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x73,
    0x68, 0x61, 0x32, 0x35, 0x36, 0x22, 0x32, 0x0a, 0x06, 0x50, 0x75, 0x62, 0x41, 0x63, 0x6b, 0x12,
    0x12, 0x0a, 0x04, 0x67, 0x75, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x67,
    0x75, 0x69, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0xc0, 0x01, 0x0a, 0x08, 0x4d, 0x73,
    0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1a, 0x0a, 0x08, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e,
    0x63, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e,
    0x63, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x07, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x14, 0x0a, 0x05,
    0x72, 0x65, 0x70, 0x6c, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x72, 0x65, 0x70,
    0x6c, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x12, 0x20, 0x0a, 0x0b, 0x72, 0x65, 0x64, 0x65, 0x6c, 0x69, 0x76, 0x65,
    0x72, 0x65, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x72, 0x65, 0x64, 0x65, 0x6c,
    0x69, 0x76, 0x65, 0x72, 0x65, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x43, 0x52, 0x43, 0x33, 0x32, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x43, 0x52, 0x43, 0x33, 0x32, 0x22, 0x3b, 0x0a, 0x03,
    0x41, 0x63, 0x6b, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x1a, 0x0a,
    0x08, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x08, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x22, 0x54, 0x0a, 0x0e, 0x43, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x44, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x44, 0x12, 0x26, 0x0a, 0x0e, 0x68, 0x65, 0x61, 0x72, 0x74,
    0x62, 0x65, 0x61, 0x74, 0x49, 0x6e, 0x62, 0x6f, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0e, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x49, 0x6e, 0x62, 0x6f, 0x78, 0x22,
    0xfd, 0x01, 0x0a, 0x0f, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x70, 0x75, 0x62, 0x50, 0x72, 0x65, 0x66, 0x69, 0x78,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x70, 0x75, 0x62, 0x50, 0x72, 0x65, 0x66, 0x69,
    0x78, 0x12, 0x20, 0x0a, 0x0b, 0x73, 0x75, 0x62, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x73, 0x75, 0x62, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x73, 0x12, 0x24, 0x0a, 0x0d, 0x75, 0x6e, 0x73, 0x75, 0x62, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x75, 0x6e, 0x73, 0x75,
    0x62, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x12, 0x24, 0x0a, 0x0d, 0x63, 0x6c, 0x6f,
    0x73, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x0d, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x12,
    0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x2a, 0x0a, 0x10, 0x73, 0x75, 0x62, 0x43, 0x6c, 0x6f, 0x73,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x10, 0x73, 0x75, 0x62, 0x43, 0x6c, 0x6f, 0x73, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x73, 0x12, 0x1c, 0x0a, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x18, 0x64,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x22,
    0xea, 0x02, 0x0a, 0x13, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x49, 0x44, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x49, 0x44, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x16, 0x0a,
    0x06, 0x71, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x71,
    0x47, 0x72, 0x6f, 0x75, 0x70, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x62, 0x6f, 0x78, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x69, 0x6e, 0x62, 0x6f, 0x78, 0x12, 0x20, 0x0a, 0x0b, 0x6d,
    0x61, 0x78, 0x49, 0x6e, 0x46, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0b, 0x6d, 0x61, 0x78, 0x49, 0x6e, 0x46, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x12, 0x24, 0x0a,
    0x0d, 0x61, 0x63, 0x6b, 0x57, 0x61, 0x69, 0x74, 0x49, 0x6e, 0x53, 0x65, 0x63, 0x73, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x61, 0x63, 0x6b, 0x57, 0x61, 0x69, 0x74, 0x49, 0x6e, 0x53,
    0x65, 0x63, 0x73, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x75, 0x72, 0x61, 0x62, 0x6c, 0x65, 0x4e, 0x61,
    0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x75, 0x72, 0x61, 0x62, 0x6c,
    0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x37, 0x0a, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x50, 0x6f,
    0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x70,
    0x62, 0x2e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x24,
    0x0a, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x18,
    0x0b, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x53, 0x65, 0x71, 0x75,
    0x65, 0x6e, 0x63, 0x65, 0x12, 0x26, 0x0a, 0x0e, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d,
    0x65, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0e, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x22, 0x48, 0x0a, 0x14,
    0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x61, 0x63, 0x6b, 0x49, 0x6e, 0x62, 0x6f, 0x78,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x61, 0x63, 0x6b, 0x49, 0x6e, 0x62, 0x6f, 0x78,
    0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x82, 0x01, 0x0a, 0x12, 0x55, 0x6e, 0x73, 0x75, 0x62,
    0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1a, 0x0a,
    0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x44, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x44, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x73, 0x75, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x62, 0x6f, 0x78, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x05, 0x69, 0x6e, 0x62, 0x6f, 0x78, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x75, 0x72,
    0x61, 0x62, 0x6c, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b,
    0x64, 0x75, 0x72, 0x61, 0x62, 0x6c, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0x2a, 0x0a, 0x0c, 0x43,
    0x6c, 0x6f, 0x73, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x44, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x44, 0x22, 0x25, 0x0a, 0x0d, 0x43, 0x6c, 0x6f, 0x73, 0x65,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2a, 0x60,
    0x0a, 0x0d, 0x53, 0x74, 0x61, 0x72, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x0b, 0x0a, 0x07, 0x4e, 0x65, 0x77, 0x4f, 0x6e, 0x6c, 0x79, 0x10, 0x00, 0x12, 0x10, 0x0a, 0x0c,
    0x4c, 0x61, 0x73, 0x74, 0x52, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x10, 0x01, 0x12, 0x12,
    0x0a, 0x0e, 0x54, 0x69, 0x6d, 0x65, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x53, 0x74, 0x61, 0x72, 0x74,
    0x10, 0x02, 0x12, 0x11, 0x0a, 0x0d, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x53, 0x74,
    0x61, 0x72, 0x74, 0x10, 0x03, 0x12, 0x09, 0x0a, 0x05, 0x46, 0x69, 0x72, 0x73, 0x74, 0x10, 0x04,
    0x42, 0x1e, 0x0a, 0x1a, 0x69, 0x6f, 0x2e, 0x6e, 0x61, 0x74, 0x73, 0x2e, 0x73, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x50, 0x01,
    0x4a, 0xd5, 0x30, 0x0a, 0x06, 0x12, 0x04, 0x05, 0x00, 0x78, 0x01, 0x0a, 0xb2, 0x01, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x05, 0x00, 0x12, 0x32, 0xa7, 0x01, 0x20, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69,
    0x67, 0x68, 0x74, 0x20, 0x32, 0x30, 0x31, 0x36, 0x20, 0x41, 0x70, 0x63, 0x65, 0x72, 0x61, 0x20,
    0x49, 0x6e, 0x63, 0x2e, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x72, 0x69, 0x67, 0x68, 0x74, 0x73, 0x20,
    0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x20, 0x55, 0x73, 0x65, 0x73,
    0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e,
    0x63, 0x6f, 0x6d, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x0a, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x76, 0x69, 0x61, 0x20,
    0x60, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x20, 0x2d, 0x49, 0x3d, 0x2e, 0x20, 0x2d, 0x49, 0x3d,
    0x24, 0x47, 0x4f, 0x50, 0x41, 0x54, 0x48, 0x2f, 0x73, 0x72, 0x63, 0x20, 0x20, 0x2d, 0x2d, 0x67,
    0x6f, 0x67, 0x6f, 0x66, 0x61, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x6f, 0x75, 0x74, 0x3d, 0x2e, 0x20,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x60, 0x0a,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x06, 0x08, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x10, 0x00, 0x33, 0x0a, 0xff, 0x01, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x10,
    0x00, 0x33, 0x1a, 0x17, 0x20, 0x4a, 0x61, 0x76, 0x61, 0x2d, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66,
    0x69, 0x63, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x32, 0xd8, 0x01, 0x20, 0x47,
    0x6f, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x0a, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x67, 0x6f, 0x5f, 0x70, 0x61, 0x63,
    0x6b, 0x61, 0x67, 0x65, 0x20, 0x3d, 0x20, 0x22, 0x70, 0x62, 0x22, 0x3b, 0x0a, 0x6f, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x28, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x6d,
    0x61, 0x72, 0x73, 0x68, 0x61, 0x6c, 0x65, 0x72, 0x5f, 0x61, 0x6c, 0x6c, 0x29, 0x20, 0x3d, 0x20,
    0x74, 0x72, 0x75, 0x65, 0x3b, 0x0a, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x28, 0x67, 0x6f,
    0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x73, 0x69, 0x7a, 0x65, 0x72, 0x5f, 0x61, 0x6c,
    0x6c, 0x29, 0x20, 0x3d, 0x20, 0x74, 0x72, 0x75, 0x65, 0x3b, 0x0a, 0x6f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x28, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x75, 0x6e, 0x6d,
    0x61, 0x72, 0x73, 0x68, 0x61, 0x6c, 0x65, 0x72, 0x5f, 0x61, 0x6c, 0x6c, 0x29, 0x20, 0x3d, 0x20,
    0x74, 0x72, 0x75, 0x65, 0x3b, 0x0a, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x28, 0x67, 0x6f,
    0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f,
    0x67, 0x65, 0x74, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x61, 0x6c, 0x6c, 0x29, 0x20, 0x3d, 0x20, 0x66,
    0x61, 0x6c, 0x73, 0x65, 0x3b, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x10, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x10, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x10, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x10, 0x16,
    0x32, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x11, 0x00, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x01, 0x12, 0x03, 0x11, 0x00, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01,
    0x02, 0x12, 0x03, 0x11, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x11, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x11, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03,
    0x11, 0x1d, 0x21, 0x0a, 0x76, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x16, 0x00, 0x1e, 0x01, 0x1a,
    0x30, 0x20, 0x48, 0x6f, 0x77, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x61,
    0x72, 0x65, 0x20, 0x64, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x53, 0x54, 0x41, 0x4e, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x0a, 0x32, 0x38, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x22, 0x67, 0x69, 0x74, 0x68, 0x75,
    0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f,
    0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3b, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x16, 0x08, 0x0e, 0x0a, 0x17, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x17, 0x02, 0x16, 0x22, 0x0a, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x44, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x17, 0x02, 0x16, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x17, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x14, 0x15, 0x0a, 0x13, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x18, 0x02, 0x16, 0x22, 0x06, 0x20, 0x67, 0x75, 0x69, 0x64, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x18, 0x02, 0x17, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x18, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x18, 0x14, 0x15, 0x0a, 0x16, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x19, 0x02, 0x16, 0x22, 0x09, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x19, 0x02, 0x18, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x19, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x19, 0x14, 0x15, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x1a, 0x02, 0x16, 0x22, 0x10, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x20, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x1a, 0x02, 0x19, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x1a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1a,
    0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1a, 0x14, 0x15,
    0x0a, 0x16, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x16, 0x22, 0x09, 0x20,
    0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x1b, 0x02, 0x1a, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x1b, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x1b, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1b, 0x14,
    0x15, 0x0a, 0x26, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x1d, 0x02, 0x16, 0x22, 0x19,
    0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x73, 0x68, 0x61, 0x32, 0x35, 0x36,
    0x20, 0x6f, 0x66, 0x20, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x04, 0x12, 0x04, 0x1d, 0x02, 0x1b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x1d, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x1d, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1d,
    0x13, 0x15, 0x0a, 0x27, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x21, 0x00, 0x24, 0x01, 0x1a, 0x1b,
    0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x41, 0x43, 0x4b, 0x20, 0x74, 0x6f, 0x20,
    0x70, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x21, 0x08, 0x0e, 0x0a, 0x13, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x22, 0x02, 0x13, 0x22, 0x06, 0x20, 0x67, 0x75, 0x69, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x22, 0x02, 0x21, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x22, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x22, 0x11, 0x12, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x23,
    0x02, 0x13, 0x22, 0x27, 0x20, 0x65, 0x72, 0x72, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x2c,
    0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2f, 0x6f, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x69,
    0x66, 0x20, 0x6e, 0x6f, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x23, 0x02, 0x22, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x23, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x23, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x23, 0x11, 0x12, 0x0a, 0x7d, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x28, 0x00, 0x31, 0x01,
    0x1a, 0x71, 0x20, 0x4d, 0x73, 0x67, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x2e, 0x20, 0x53,
    0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x69, 0x73, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67,
    0x6e, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x20, 0x6f,
    0x72, 0x64, 0x65, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x62, 0x79, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x65, 0x72, 0x20, 0x68, 0x61, 0x73, 0x20,
    0x62, 0x65, 0x65, 0x6e, 0x20, 0x61, 0x63, 0x6b, 0x6e, 0x6f, 0x77, 0x6c, 0x65, 0x64, 0x67, 0x65,
    0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x28, 0x08, 0x10, 0x0a,
    0x49, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x19, 0x22, 0x3c, 0x20, 0x67,
    0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x65, 0x64, 0x20,
    0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x27,
    0x73, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x29, 0x02, 0x28, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x29, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x29, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x29, 0x17, 0x18, 0x0a, 0x16, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x19,
    0x22, 0x09, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x2a, 0x02, 0x29, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x2a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x2a, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x2a, 0x17, 0x18, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x02,
    0x19, 0x22, 0x10, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x70,
    0x6c, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x2b, 0x02,
    0x2a, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2b, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x09, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x17, 0x18, 0x0a, 0x16, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x02, 0x19, 0x22, 0x09, 0x20, 0x70, 0x61, 0x79, 0x6c,
    0x6f, 0x61, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x2c,
    0x02, 0x2b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2c, 0x02,
    0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2c, 0x09, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2c, 0x17, 0x18, 0x0a, 0x21, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x19, 0x22, 0x14, 0x20, 0x72, 0x65, 0x63,
    0x65, 0x69, 0x76, 0x65, 0x64, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2d, 0x02, 0x2c, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2d, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2d, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2d, 0x17, 0x18, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x2e, 0x02, 0x19, 0x22, 0x35, 0x20, 0x46, 0x6c, 0x61, 0x67, 0x20, 0x73, 0x70,
    0x65, 0x63, 0x69, 0x66, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67,
    0x20, 0x72, 0x65, 0x64, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x04, 0x2e, 0x02, 0x2d, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03, 0x2e, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x2e, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x2e, 0x17, 0x18, 0x0a, 0x22, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x30, 0x02, 0x1a, 0x22, 0x15, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x49,
    0x45, 0x45, 0x45, 0x20, 0x43, 0x52, 0x43, 0x33, 0x32, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x04, 0x12, 0x04, 0x30, 0x02, 0x2e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x06, 0x05, 0x12, 0x03, 0x30, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x30, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x30, 0x17, 0x19, 0x0a, 0x3a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x34, 0x00, 0x37, 0x01, 0x1a,
    0x2e, 0x20, 0x41, 0x63, 0x6b, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x64, 0x65, 0x6c, 0x69, 0x76,
    0x65, 0x72, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x63, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20,
    0x64, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x65, 0x64, 0x20, 0x6d, 0x73, 0x67, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x34, 0x08, 0x0b, 0x0a, 0x16, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x35, 0x02, 0x16, 0x22, 0x09, 0x20, 0x53, 0x75, 0x62, 0x6a, 0x65,
    0x63, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x35, 0x02,
    0x34, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x35, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x09, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x35, 0x14, 0x15, 0x0a, 0x26, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x36, 0x02, 0x16, 0x22, 0x19, 0x20, 0x53, 0x65, 0x71, 0x75,
    0x65, 0x6e, 0x63, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x63, 0x6b, 0x6e, 0x6f, 0x77, 0x6c, 0x65,
    0x64, 0x67, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x36,
    0x02, 0x35, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x36, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x36, 0x09, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x36, 0x14, 0x15, 0x0a, 0x20, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x3a, 0x00, 0x3d, 0x01, 0x1a, 0x14, 0x20, 0x43, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x16, 0x0a, 0x26, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x00, 0x12, 0x03, 0x3b, 0x02, 0x1c, 0x22, 0x19, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x2f, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x3b, 0x02,
    0x3a, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3b, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x09, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x1a, 0x1b, 0x0a, 0x35, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x02, 0x1c, 0x22, 0x28, 0x20, 0x49, 0x6e, 0x62, 0x6f,
    0x78, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x69, 0x6e, 0x69,
    0x74, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74,
    0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x3c, 0x02,
    0x3b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3c, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3c, 0x09, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3c, 0x1a, 0x1b, 0x0a, 0x2a, 0x0a, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x40, 0x00, 0x49, 0x01, 0x1a, 0x1e, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20,
    0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x40, 0x08, 0x17, 0x0a, 0x41, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x41, 0x02,
    0x1e, 0x22, 0x34, 0x20, 0x50, 0x72, 0x65, 0x66, 0x69, 0x78, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73,
    0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x69, 0x6e,
    0x67, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x53, 0x54, 0x41, 0x4e, 0x20, 0x63,
    0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x41, 0x02, 0x40, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x41, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x41,
    0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x41, 0x1c, 0x1d,
    0x0a, 0x37, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x42, 0x02, 0x1e, 0x22, 0x2a, 0x20,
    0x53, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x42, 0x02, 0x41, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x42, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x42, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x42,
    0x1c, 0x1d, 0x0a, 0x36, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x43, 0x02, 0x1e, 0x22,
    0x29, 0x20, 0x53, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x75, 0x6e, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x43, 0x02, 0x42, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x43, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x43, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x43, 0x1c, 0x1d, 0x0a, 0x36, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x44, 0x02, 0x1e,
    0x22, 0x29, 0x20, 0x53, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x63,
    0x6c, 0x6f, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x6e, 0x20,
    0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x03, 0x04, 0x12, 0x04, 0x44, 0x02, 0x43, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x44, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x44, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x44, 0x1c, 0x1d, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x45, 0x02,
    0x1e, 0x22, 0x27, 0x20, 0x65, 0x72, 0x72, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x2c, 0x20,
    0x65, 0x6d, 0x70, 0x74, 0x79, 0x2f, 0x6f, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x69, 0x66,
    0x20, 0x6e, 0x6f, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x45, 0x02, 0x44, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x45, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x45, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x45, 0x1c, 0x1d, 0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x05, 0x12, 0x03, 0x46, 0x02, 0x1e,
    0x22, 0x30, 0x20, 0x53, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73,
    0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x04, 0x12, 0x04, 0x46, 0x02, 0x45,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x03, 0x46, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x46, 0x09, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x05, 0x03, 0x12, 0x03, 0x46, 0x1c, 0x1d, 0x0a, 0x2f, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x06, 0x12, 0x03, 0x48, 0x02, 0x1d, 0x22, 0x22, 0x20, 0x50, 0x6f, 0x73, 0x73, 0x69,
    0x62, 0x6c, 0x79, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x69, 0x67, 0x6e,
    0x20, 0x61, 0x63, 0x6b, 0x73, 0x2c, 0x20, 0x65, 0x74, 0x63, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x06, 0x04, 0x12, 0x04, 0x48, 0x02, 0x46, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x06, 0x05, 0x12, 0x03, 0x48, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x48, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x48, 0x19, 0x1c, 0x0a, 0x2b, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x4c, 0x00, 0x52,
    0x03, 0x1a, 0x1f, 0x20, 0x45, 0x6e, 0x75, 0x6d, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x79, 0x70, 0x65,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x4c, 0x05, 0x12, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x4d, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4d, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x4d, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x4e, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4e,
    0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x4e, 0x15, 0x16,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x4f, 0x04, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x4f, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x50, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x50, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x50,
    0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x51, 0x04, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x51, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x51, 0x15, 0x16, 0x0a, 0x30, 0x0a, 0x02, 0x04,
    0x06, 0x12, 0x04, 0x55, 0x00, 0x60, 0x01, 0x1a, 0x24, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20,
    0x74, 0x6f, 0x20, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x55, 0x08, 0x1b, 0x0a, 0x17, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x00, 0x12, 0x03, 0x56, 0x02, 0x23, 0x22, 0x0a, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49,
    0x44, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x56, 0x02, 0x55,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x56, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x56, 0x10, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x56, 0x21, 0x22, 0x0a, 0x3b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x01, 0x12, 0x03, 0x57, 0x02, 0x23, 0x22, 0x2e, 0x20, 0x46, 0x6f, 0x72, 0x6d, 0x61,
    0x6c, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x75, 0x62,
    0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x20, 0x74, 0x6f, 0x2c, 0x20, 0x65, 0x2e, 0x67, 0x2e, 0x20,
    0x66, 0x6f, 0x6f, 0x2e, 0x62, 0x61, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x57, 0x02, 0x56, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x57, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x57, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x57, 0x21,
    0x22, 0x0a, 0x23, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x58, 0x02, 0x23, 0x22, 0x16,
    0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x71, 0x75, 0x65, 0x75, 0x65, 0x20,
    0x67, 0x72, 0x6f, 0x75, 0x70, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x58, 0x02, 0x57, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x58, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x58, 0x10,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x58, 0x21, 0x22, 0x0a,
    0x33, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x59, 0x02, 0x23, 0x22, 0x26, 0x20, 0x49,
    0x6e, 0x62, 0x6f, 0x78, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x64, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x20, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x04, 0x59,
    0x02, 0x58, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x59, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x59, 0x10, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x59, 0x21, 0x22, 0x0a, 0x3f, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x5a, 0x02, 0x23, 0x22, 0x32, 0x20, 0x4d, 0x61, 0x78,
    0x69, 0x6d, 0x75, 0x6d, 0x20, 0x69, 0x6e, 0x66, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74, 0x20, 0x61,
    0x6e, 0x20, 0x61, 0x63, 0x6b, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x64, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x04, 0x5a, 0x02, 0x59, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x5a, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x5a, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x5a, 0x21, 0x22, 0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05, 0x12,
    0x03, 0x5b, 0x02, 0x23, 0x22, 0x2e, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x20,
    0x61, 0x63, 0x6b, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x04, 0x12, 0x04, 0x5b,
    0x02, 0x5a, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x05, 0x12, 0x03, 0x5b, 0x02,
    0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x5b, 0x10, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x5b, 0x21, 0x22, 0x0a, 0x43, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x5c, 0x02, 0x23, 0x22, 0x36, 0x20, 0x4f, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x64, 0x75, 0x72, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6e, 0x61,
    0x6d, 0x65, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x73, 0x75, 0x72, 0x76, 0x69, 0x76, 0x65,
    0x73, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74,
    0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12, 0x04, 0x5c, 0x02, 0x5b,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x05, 0x12, 0x03, 0x5c, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x01, 0x12, 0x03, 0x5c, 0x10, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x06, 0x03, 0x12, 0x03, 0x5c, 0x21, 0x22, 0x0a, 0x1d, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x07, 0x12, 0x03, 0x5d, 0x02, 0x24, 0x22, 0x10, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74,
    0x20, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x07, 0x04, 0x12, 0x04, 0x5d, 0x02, 0x5c, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x07, 0x06, 0x12, 0x03, 0x5d, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x5d, 0x10, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x5d, 0x21, 0x23, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x08, 0x12, 0x03, 0x5e, 0x02, 0x24,
    0x22, 0x20, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x20, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65,
    0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x04, 0x12, 0x04, 0x5e, 0x02, 0x5d,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x05, 0x12, 0x03, 0x5e, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x01, 0x12, 0x03, 0x5e, 0x10, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x08, 0x03, 0x12, 0x03, 0x5e, 0x21, 0x23, 0x0a, 0x22, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x09, 0x12, 0x03, 0x5f, 0x02, 0x24, 0x22, 0x15, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x04, 0x12, 0x04, 0x5f, 0x02, 0x5e, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x05, 0x12, 0x03, 0x5f, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x09, 0x01, 0x12, 0x03, 0x5f, 0x10, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x09, 0x03, 0x12, 0x03, 0x5f, 0x21, 0x23, 0x0a, 0x46, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04,
    0x63, 0x00, 0x66, 0x01, 0x1a, 0x3a, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x55, 0x6e, 0x73, 0x75,
    0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x63, 0x08, 0x1c, 0x0a, 0x28, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x64, 0x02, 0x16, 0x22, 0x1b, 0x20, 0x61, 0x63, 0x6b, 0x49,
    0x6e, 0x62, 0x6f, 0x78, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x20, 0x61, 0x63, 0x6b, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x64, 0x02, 0x63, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x64, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x09,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x64, 0x14, 0x15, 0x0a,
    0x34, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x65, 0x02, 0x16, 0x22, 0x27, 0x20, 0x65,
    0x72, 0x72, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x2c, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79,
    0x2f, 0x6f, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x20, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x65, 0x02, 0x64, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x65,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x65, 0x09, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x65, 0x14, 0x15, 0x0a, 0x57,
    0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x69, 0x00, 0x6e, 0x01, 0x1a, 0x4b, 0x20, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x6e, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x2e, 0x20, 0x57, 0x69, 0x6c, 0x6c, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20,
    0x61, 0x20, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03,
    0x69, 0x08, 0x1a, 0x0a, 0x17, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x02, 0x19,
    0x22, 0x0a, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x44, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x6a, 0x02, 0x69, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x6a, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x6a, 0x17, 0x18, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x6b,
    0x02, 0x19, 0x22, 0x1e, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0x6b, 0x02, 0x6a,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6b, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6b, 0x09, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6b, 0x17, 0x18, 0x0a, 0x35, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x02, 0x12, 0x03, 0x6c, 0x02, 0x19, 0x22, 0x28, 0x20, 0x49, 0x6e, 0x62, 0x6f, 0x78,
    0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x66, 0x79, 0x20, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x04, 0x6c, 0x02, 0x6b,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6c, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6c, 0x09, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6c, 0x17, 0x18, 0x0a, 0x43, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x03, 0x12, 0x03, 0x6d, 0x02, 0x19, 0x22, 0x36, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x20, 0x64, 0x75, 0x72, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x73, 0x75, 0x72, 0x76, 0x69, 0x76, 0x65, 0x73, 0x20,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x73, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x04, 0x6d, 0x02, 0x6c, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x05, 0x12, 0x03, 0x6d, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6d, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x03, 0x03, 0x12, 0x03, 0x6d, 0x17, 0x18, 0x0a, 0x39, 0x0a, 0x02, 0x04, 0x09, 0x12,
    0x04, 0x71, 0x00, 0x73, 0x01, 0x1a, 0x2d, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f,
    0x20, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x71, 0x08, 0x14,
    0x0a, 0x39, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x72, 0x02, 0x16, 0x22, 0x2c, 0x20,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x76,
    0x69, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x28,
    0x29, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x72, 0x02, 0x71, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x72, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x72, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x72, 0x14, 0x15, 0x0a, 0x27, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x76, 0x00, 0x78, 0x01,
    0x1a, 0x1b, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x43, 0x6c, 0x6f, 0x73, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x76, 0x08, 0x15, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x0a, 0x02,
    0x00, 0x12, 0x03, 0x77, 0x02, 0x13, 0x22, 0x27, 0x20, 0x65, 0x72, 0x72, 0x20, 0x73, 0x74, 0x72,
    0x69, 0x6e, 0x67, 0x2c, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2f, 0x6f, 0x6d, 0x69, 0x74, 0x74,
    0x65, 0x64, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x77, 0x02, 0x76, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x77, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x77, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x77, 0x11, 0x12, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
