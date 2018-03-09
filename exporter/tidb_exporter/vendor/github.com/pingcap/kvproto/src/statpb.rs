// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Timespec {
    // message fields
    sec: ::std::option::Option<i64>,
    nsec: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Timespec {}

impl Timespec {
    pub fn new() -> Timespec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Timespec {
        static mut instance: ::protobuf::lazy::Lazy<Timespec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Timespec,
        };
        unsafe {
            instance.get(|| {
                Timespec {
                    sec: ::std::option::Option::None,
                    nsec: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 sec = 1;

    pub fn clear_sec(&mut self) {
        self.sec = ::std::option::Option::None;
    }

    pub fn has_sec(&self) -> bool {
        self.sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sec(&mut self, v: i64) {
        self.sec = ::std::option::Option::Some(v);
    }

    pub fn get_sec(&self) -> i64 {
        self.sec.unwrap_or(0)
    }

    // optional int32 nsec = 2;

    pub fn clear_nsec(&mut self) {
        self.nsec = ::std::option::Option::None;
    }

    pub fn has_nsec(&self) -> bool {
        self.nsec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nsec(&mut self, v: i32) {
        self.nsec = ::std::option::Option::Some(v);
    }

    pub fn get_nsec(&self) -> i32 {
        self.nsec.unwrap_or(0)
    }
}

impl ::protobuf::Message for Timespec {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.sec = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.nsec = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.sec {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.nsec {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sec {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.nsec {
            try!(os.write_int32(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Timespec>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Timespec {
    fn new() -> Timespec {
        Timespec::new()
    }

    fn descriptor_static(_: ::std::option::Option<Timespec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sec",
                    Timespec::has_sec,
                    Timespec::get_sec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "nsec",
                    Timespec::has_nsec,
                    Timespec::get_nsec,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Timespec>(
                    "Timespec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Timespec {
    fn clear(&mut self) {
        self.clear_sec();
        self.clear_nsec();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Timespec {
    fn eq(&self, other: &Timespec) -> bool {
        self.sec == other.sec &&
        self.nsec == other.nsec &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Timespec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PingReq {
    // message fields
    ping_ts: ::protobuf::SingularPtrField<Timespec>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingReq {}

impl PingReq {
    pub fn new() -> PingReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingReq {
        static mut instance: ::protobuf::lazy::Lazy<PingReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingReq,
        };
        unsafe {
            instance.get(|| {
                PingReq {
                    ping_ts: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .statpb.Timespec ping_ts = 1;

    pub fn clear_ping_ts(&mut self) {
        self.ping_ts.clear();
    }

    pub fn has_ping_ts(&self) -> bool {
        self.ping_ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ts(&mut self, v: Timespec) {
        self.ping_ts = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_ts(&mut self) -> &mut Timespec {
        if self.ping_ts.is_none() {
            self.ping_ts.set_default();
        };
        self.ping_ts.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_ts(&mut self) -> Timespec {
        self.ping_ts.take().unwrap_or_else(|| Timespec::new())
    }

    pub fn get_ping_ts(&self) -> &Timespec {
        self.ping_ts.as_ref().unwrap_or_else(|| Timespec::default_instance())
    }
}

impl ::protobuf::Message for PingReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_ts));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.ping_ts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ping_ts.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PingReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PingReq {
    fn new() -> PingReq {
        PingReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ping_ts",
                    PingReq::has_ping_ts,
                    PingReq::get_ping_ts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingReq>(
                    "PingReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingReq {
    fn clear(&mut self) {
        self.clear_ping_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PingReq {
    fn eq(&self, other: &PingReq) -> bool {
        self.ping_ts == other.ping_ts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PingReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PingResp {
    // message fields
    ping_ts: ::protobuf::SingularPtrField<Timespec>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingResp {}

impl PingResp {
    pub fn new() -> PingResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingResp {
        static mut instance: ::protobuf::lazy::Lazy<PingResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingResp,
        };
        unsafe {
            instance.get(|| {
                PingResp {
                    ping_ts: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .statpb.Timespec ping_ts = 1;

    pub fn clear_ping_ts(&mut self) {
        self.ping_ts.clear();
    }

    pub fn has_ping_ts(&self) -> bool {
        self.ping_ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ts(&mut self, v: Timespec) {
        self.ping_ts = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_ts(&mut self) -> &mut Timespec {
        if self.ping_ts.is_none() {
            self.ping_ts.set_default();
        };
        self.ping_ts.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_ts(&mut self) -> Timespec {
        self.ping_ts.take().unwrap_or_else(|| Timespec::new())
    }

    pub fn get_ping_ts(&self) -> &Timespec {
        self.ping_ts.as_ref().unwrap_or_else(|| Timespec::default_instance())
    }
}

impl ::protobuf::Message for PingResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_ts));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.ping_ts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ping_ts.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PingResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PingResp {
    fn new() -> PingResp {
        PingResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ping_ts",
                    PingResp::has_ping_ts,
                    PingResp::get_ping_ts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingResp>(
                    "PingResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingResp {
    fn clear(&mut self) {
        self.clear_ping_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PingResp {
    fn eq(&self, other: &PingResp) -> bool {
        self.ping_ts == other.ping_ts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PingResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RequestHeader {
    // message fields
    id: ::std::option::Option<u64>,
    from_store_id: ::std::option::Option<u64>,
    to_store_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestHeader {}

impl RequestHeader {
    pub fn new() -> RequestHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestHeader {
        static mut instance: ::protobuf::lazy::Lazy<RequestHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestHeader,
        };
        unsafe {
            instance.get(|| {
                RequestHeader {
                    id: ::std::option::Option::None,
                    from_store_id: ::std::option::Option::None,
                    to_store_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    // optional uint64 from_store_id = 2;

    pub fn clear_from_store_id(&mut self) {
        self.from_store_id = ::std::option::Option::None;
    }

    pub fn has_from_store_id(&self) -> bool {
        self.from_store_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_store_id(&mut self, v: u64) {
        self.from_store_id = ::std::option::Option::Some(v);
    }

    pub fn get_from_store_id(&self) -> u64 {
        self.from_store_id.unwrap_or(0)
    }

    // optional uint64 to_store_id = 3;

    pub fn clear_to_store_id(&mut self) {
        self.to_store_id = ::std::option::Option::None;
    }

    pub fn has_to_store_id(&self) -> bool {
        self.to_store_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_store_id(&mut self, v: u64) {
        self.to_store_id = ::std::option::Option::Some(v);
    }

    pub fn get_to_store_id(&self) -> u64 {
        self.to_store_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for RequestHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.from_store_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.to_store_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.from_store_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.to_store_id {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.from_store_id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.to_store_id {
            try!(os.write_uint64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RequestHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestHeader {
    fn new() -> RequestHeader {
        RequestHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "id",
                    RequestHeader::has_id,
                    RequestHeader::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "from_store_id",
                    RequestHeader::has_from_store_id,
                    RequestHeader::get_from_store_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "to_store_id",
                    RequestHeader::has_to_store_id,
                    RequestHeader::get_to_store_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestHeader>(
                    "RequestHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestHeader {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_from_store_id();
        self.clear_to_store_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestHeader {
    fn eq(&self, other: &RequestHeader) -> bool {
        self.id == other.id &&
        self.from_store_id == other.from_store_id &&
        self.to_store_id == other.to_store_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RequestHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ResponseHeader {
    // message fields
    id: ::std::option::Option<u64>,
    from_store_id: ::std::option::Option<u64>,
    to_store_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseHeader {}

impl ResponseHeader {
    pub fn new() -> ResponseHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseHeader {
        static mut instance: ::protobuf::lazy::Lazy<ResponseHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseHeader,
        };
        unsafe {
            instance.get(|| {
                ResponseHeader {
                    id: ::std::option::Option::None,
                    from_store_id: ::std::option::Option::None,
                    to_store_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    // optional uint64 from_store_id = 2;

    pub fn clear_from_store_id(&mut self) {
        self.from_store_id = ::std::option::Option::None;
    }

    pub fn has_from_store_id(&self) -> bool {
        self.from_store_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_store_id(&mut self, v: u64) {
        self.from_store_id = ::std::option::Option::Some(v);
    }

    pub fn get_from_store_id(&self) -> u64 {
        self.from_store_id.unwrap_or(0)
    }

    // optional uint64 to_store_id = 3;

    pub fn clear_to_store_id(&mut self) {
        self.to_store_id = ::std::option::Option::None;
    }

    pub fn has_to_store_id(&self) -> bool {
        self.to_store_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_store_id(&mut self, v: u64) {
        self.to_store_id = ::std::option::Option::Some(v);
    }

    pub fn get_to_store_id(&self) -> u64 {
        self.to_store_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for ResponseHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.from_store_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.to_store_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.from_store_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.to_store_id {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.from_store_id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.to_store_id {
            try!(os.write_uint64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ResponseHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseHeader {
    fn new() -> ResponseHeader {
        ResponseHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "id",
                    ResponseHeader::has_id,
                    ResponseHeader::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "from_store_id",
                    ResponseHeader::has_from_store_id,
                    ResponseHeader::get_from_store_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "to_store_id",
                    ResponseHeader::has_to_store_id,
                    ResponseHeader::get_to_store_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseHeader>(
                    "ResponseHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseHeader {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_from_store_id();
        self.clear_to_store_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ResponseHeader {
    fn eq(&self, other: &ResponseHeader) -> bool {
        self.id == other.id &&
        self.from_store_id == other.from_store_id &&
        self.to_store_id == other.to_store_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ResponseHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Request {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    msg_type: ::std::option::Option<MessageType>,
    ping_req: ::protobuf::SingularPtrField<PingReq>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(|| {
                Request {
                    header: ::protobuf::SingularPtrField::none(),
                    msg_type: ::std::option::Option::None,
                    ping_req: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .statpb.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header(&self) -> &RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }

    // optional .statpb.MessageType msg_type = 2;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: MessageType) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> MessageType {
        self.msg_type.unwrap_or(MessageType::Ping)
    }

    // optional .statpb.PingReq ping_req = 3;

    pub fn clear_ping_req(&mut self) {
        self.ping_req.clear();
    }

    pub fn has_ping_req(&self) -> bool {
        self.ping_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_req(&mut self, v: PingReq) {
        self.ping_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_req(&mut self) -> &mut PingReq {
        if self.ping_req.is_none() {
            self.ping_req.set_default();
        };
        self.ping_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_req(&mut self) -> PingReq {
        self.ping_req.take().unwrap_or_else(|| PingReq::new())
    }

    pub fn get_ping_req(&self) -> &PingReq {
        self.ping_req.as_ref().unwrap_or_else(|| PingReq::default_instance())
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.msg_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_req));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.header {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.msg_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.ping_req {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.msg_type {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.ping_req.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Request>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    Request::has_header,
                    Request::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "msg_type",
                    Request::has_msg_type,
                    Request::get_msg_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ping_req",
                    Request::has_ping_req,
                    Request::get_ping_req,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_msg_type();
        self.clear_ping_req();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.header == other.header &&
        self.msg_type == other.msg_type &&
        self.ping_req == other.ping_req &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Response {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    msg_type: ::std::option::Option<MessageType>,
    ping_resp: ::protobuf::SingularPtrField<PingResp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(|| {
                Response {
                    header: ::protobuf::SingularPtrField::none(),
                    msg_type: ::std::option::Option::None,
                    ping_resp: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .statpb.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    // optional .statpb.MessageType msg_type = 2;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: MessageType) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> MessageType {
        self.msg_type.unwrap_or(MessageType::Ping)
    }

    // optional .statpb.PingResp ping_resp = 3;

    pub fn clear_ping_resp(&mut self) {
        self.ping_resp.clear();
    }

    pub fn has_ping_resp(&self) -> bool {
        self.ping_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_resp(&mut self, v: PingResp) {
        self.ping_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_resp(&mut self) -> &mut PingResp {
        if self.ping_resp.is_none() {
            self.ping_resp.set_default();
        };
        self.ping_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_resp(&mut self) -> PingResp {
        self.ping_resp.take().unwrap_or_else(|| PingResp::new())
    }

    pub fn get_ping_resp(&self) -> &PingResp {
        self.ping_resp.as_ref().unwrap_or_else(|| PingResp::default_instance())
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.msg_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_resp));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.header {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.msg_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.ping_resp {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.msg_type {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.ping_resp.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Response>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    Response::has_header,
                    Response::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "msg_type",
                    Response::has_msg_type,
                    Response::get_msg_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ping_resp",
                    Response::has_ping_resp,
                    Response::get_ping_resp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_msg_type();
        self.clear_ping_resp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.header == other.header &&
        self.msg_type == other.msg_type &&
        self.ping_resp == other.ping_resp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageType {
    Ping = 0,
}

impl ::protobuf::ProtobufEnum for MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageType> {
        match value {
            0 => ::std::option::Option::Some(MessageType::Ping),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageType] = &[
            MessageType::Ping,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MessageType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x73, 0x74, 0x61, 0x74, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06,
    0x73, 0x74, 0x61, 0x74, 0x70, 0x62, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3c, 0x0a, 0x08,
    0x54, 0x69, 0x6d, 0x65, 0x73, 0x70, 0x65, 0x63, 0x12, 0x16, 0x0a, 0x03, 0x73, 0x65, 0x63, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x03, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x03, 0x73, 0x65, 0x63,
    0x12, 0x18, 0x0a, 0x04, 0x6e, 0x73, 0x65, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x42, 0x04,
    0xc8, 0xde, 0x1f, 0x00, 0x52, 0x04, 0x6e, 0x73, 0x65, 0x63, 0x22, 0x34, 0x0a, 0x07, 0x50, 0x69,
    0x6e, 0x67, 0x52, 0x65, 0x71, 0x12, 0x29, 0x0a, 0x07, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x73,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x73, 0x74, 0x61, 0x74, 0x70, 0x62, 0x2e,
    0x54, 0x69, 0x6d, 0x65, 0x73, 0x70, 0x65, 0x63, 0x52, 0x06, 0x70, 0x69, 0x6e, 0x67, 0x54, 0x73,
    0x22, 0x35, 0x0a, 0x08, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x12, 0x29, 0x0a, 0x07,
    0x70, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x73, 0x74, 0x61, 0x74, 0x70, 0x62, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x70, 0x65, 0x63, 0x52,
    0x06, 0x70, 0x69, 0x6e, 0x67, 0x54, 0x73, 0x22, 0x63, 0x0a, 0x0d, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x22, 0x0a, 0x0d, 0x66, 0x72, 0x6f, 0x6d,
    0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0b, 0x66, 0x72, 0x6f, 0x6d, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x64, 0x12, 0x1e, 0x0a, 0x0b,
    0x74, 0x6f, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x09, 0x74, 0x6f, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x64, 0x22, 0x64, 0x0a, 0x0e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x0e,
    0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x22,
    0x0a, 0x0d, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x66, 0x72, 0x6f, 0x6d, 0x53, 0x74, 0x6f, 0x72, 0x65,
    0x49, 0x64, 0x12, 0x1e, 0x0a, 0x0b, 0x74, 0x6f, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x74, 0x6f, 0x53, 0x74, 0x6f, 0x72, 0x65,
    0x49, 0x64, 0x22, 0x9a, 0x01, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2d,
    0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15,
    0x2e, 0x73, 0x74, 0x61, 0x74, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x34, 0x0a,
    0x08, 0x6d, 0x73, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x13, 0x2e, 0x73, 0x74, 0x61, 0x74, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x54, 0x79, 0x70, 0x65, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x07, 0x6d, 0x73, 0x67, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x2a, 0x0a, 0x08, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x72, 0x65, 0x71, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x73, 0x74, 0x61, 0x74, 0x70, 0x62, 0x2e, 0x50,
    0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x52, 0x07, 0x70, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x22,
    0x9f, 0x01, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2e, 0x0a, 0x06,
    0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x73,
    0x74, 0x61, 0x74, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x34, 0x0a, 0x08,
    0x6d, 0x73, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x13,
    0x2e, 0x73, 0x74, 0x61, 0x74, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54,
    0x79, 0x70, 0x65, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x07, 0x6d, 0x73, 0x67, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x2d, 0x0a, 0x09, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x73, 0x74, 0x61, 0x74, 0x70, 0x62, 0x2e, 0x50,
    0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x52, 0x08, 0x70, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73,
    0x70, 0x2a, 0x17, 0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x08, 0x0a, 0x04, 0x50, 0x69, 0x6e, 0x67, 0x10, 0x00, 0x4a, 0x88, 0x0e, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x2c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x03, 0x07, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x05, 0x00, 0x07, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x05, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x06, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x0c,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x10, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x04, 0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x0a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0a, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a,
    0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x0a, 0x2a, 0x48,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x0a, 0x2b,
    0x47, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x0a, 0x2b, 0x3f, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x0a, 0x2b, 0x3f, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x2c, 0x3e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x42, 0x47, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x04, 0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x0b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0b, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x28,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x2a, 0x48, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x0b, 0x2b, 0x47,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x0b,
    0x2b, 0x3f, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x0b, 0x2b, 0x3f, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x2c, 0x3e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x42, 0x47, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x0e, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x0e, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x04, 0x2a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0f, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04,
    0x12, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x12, 0x08, 0x10,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x13, 0x04, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x13, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x13, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x13, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x13, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x16, 0x00, 0x1a,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x16, 0x08, 0x15, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x17, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x17, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x17, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x17, 0x18, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17,
    0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x2e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x18, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x18, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x18, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12,
    0x03, 0x19, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x19,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x19, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x18, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x19, 0x2c, 0x2d, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x1c, 0x00, 0x20, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x1c, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x08,
    0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1d, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x18, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x1e, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1e,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x18, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x2b, 0x2c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x1f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x1f, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x1f, 0x2c, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x22, 0x00, 0x26, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x22, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x00, 0x12, 0x03, 0x23, 0x08, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x23, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x23, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23,
    0x1f, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x42, 0x43,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x24, 0x08, 0x63, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x24, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x24, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x24, 0x1d, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x24, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x08, 0x12, 0x03,
    0x24, 0x44, 0x62, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x24, 0x45, 0x61, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x24, 0x45, 0x59, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x24, 0x45, 0x59, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x05, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x46, 0x58, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x24, 0x5c, 0x61, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x25, 0x08, 0x44, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x25, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x25, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x25, 0x19, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x25, 0x42, 0x43, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x28, 0x00, 0x2c, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x28, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x29, 0x08, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x29, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x29, 0x11, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x29, 0x20, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x42,
    0x43, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x63, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2a, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x1d, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x2a, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x08, 0x12,
    0x03, 0x2a, 0x44, 0x62, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x06, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x2a, 0x45, 0x61, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x06, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x2a, 0x45, 0x59, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x06, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2a, 0x45, 0x59, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x06,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x46, 0x58, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x06, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x2a, 0x5c, 0x61,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x08, 0x44, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2b, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x2b, 0x1a, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x2b, 0x42, 0x43,
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
