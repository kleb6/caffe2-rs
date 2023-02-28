// This file is generated by rust-protobuf 2.24.1. Do not edit
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
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `pytorch/caffe2/proto/predictor_consts.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_24_1;

#[derive(PartialEq,Clone,Default)]
pub struct PredictorConsts {
    // message fields
    META_NET_DEF: ::protobuf::SingularField<::std::string::String>,
    PREDICTOR_DBREADER: ::protobuf::SingularField<::std::string::String>,
    PARAMETERS_BLOB_TYPE: ::protobuf::SingularField<::std::string::String>,
    INPUTS_BLOB_TYPE: ::protobuf::SingularField<::std::string::String>,
    OUTPUTS_BLOB_TYPE: ::protobuf::SingularField<::std::string::String>,
    GLOBAL_INIT_NET_TYPE: ::protobuf::SingularField<::std::string::String>,
    PREDICT_INIT_NET_TYPE: ::protobuf::SingularField<::std::string::String>,
    PREDICT_NET_TYPE: ::protobuf::SingularField<::std::string::String>,
    SINGLE_PREDICTOR: ::protobuf::SingularField<::std::string::String>,
    MULTI_PREDICTOR: ::protobuf::SingularField<::std::string::String>,
    TRAIN_INIT_PLAN_TYPE: ::protobuf::SingularField<::std::string::String>,
    TRAIN_PLAN_TYPE: ::protobuf::SingularField<::std::string::String>,
    SHAPE_INFO_BLOB: ::protobuf::SingularField<::std::string::String>,
    DEFERRED_BLOB_READER: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PredictorConsts {
    fn default() -> &'a PredictorConsts {
        <PredictorConsts as ::protobuf::Message>::default_instance()
    }
}

impl PredictorConsts {
    pub fn new() -> PredictorConsts {
        ::std::default::Default::default()
    }

    // optional string META_NET_DEF = 1;


    pub fn get_META_NET_DEF(&self) -> &str {
        match self.META_NET_DEF.as_ref() {
            Some(v) => &v,
            None => "!!META_NET_DEF",
        }
    }
    pub fn clear_META_NET_DEF(&mut self) {
        self.META_NET_DEF.clear();
    }

    pub fn has_META_NET_DEF(&self) -> bool {
        self.META_NET_DEF.is_some()
    }

    // Param is passed by value, moved
    pub fn set_META_NET_DEF(&mut self, v: ::std::string::String) {
        self.META_NET_DEF = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_META_NET_DEF(&mut self) -> &mut ::std::string::String {
        if self.META_NET_DEF.is_none() {
            self.META_NET_DEF.set_default();
        }
        self.META_NET_DEF.as_mut().unwrap()
    }

    // Take field
    pub fn take_META_NET_DEF(&mut self) -> ::std::string::String {
        self.META_NET_DEF.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string PREDICTOR_DBREADER = 2;


    pub fn get_PREDICTOR_DBREADER(&self) -> &str {
        match self.PREDICTOR_DBREADER.as_ref() {
            Some(v) => &v,
            None => "!!PREDICTOR_DBREADER",
        }
    }
    pub fn clear_PREDICTOR_DBREADER(&mut self) {
        self.PREDICTOR_DBREADER.clear();
    }

    pub fn has_PREDICTOR_DBREADER(&self) -> bool {
        self.PREDICTOR_DBREADER.is_some()
    }

    // Param is passed by value, moved
    pub fn set_PREDICTOR_DBREADER(&mut self, v: ::std::string::String) {
        self.PREDICTOR_DBREADER = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_PREDICTOR_DBREADER(&mut self) -> &mut ::std::string::String {
        if self.PREDICTOR_DBREADER.is_none() {
            self.PREDICTOR_DBREADER.set_default();
        }
        self.PREDICTOR_DBREADER.as_mut().unwrap()
    }

    // Take field
    pub fn take_PREDICTOR_DBREADER(&mut self) -> ::std::string::String {
        self.PREDICTOR_DBREADER.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string PARAMETERS_BLOB_TYPE = 3;


    pub fn get_PARAMETERS_BLOB_TYPE(&self) -> &str {
        match self.PARAMETERS_BLOB_TYPE.as_ref() {
            Some(v) => &v,
            None => "PARAMETERS_BLOB_TYPE",
        }
    }
    pub fn clear_PARAMETERS_BLOB_TYPE(&mut self) {
        self.PARAMETERS_BLOB_TYPE.clear();
    }

    pub fn has_PARAMETERS_BLOB_TYPE(&self) -> bool {
        self.PARAMETERS_BLOB_TYPE.is_some()
    }

    // Param is passed by value, moved
    pub fn set_PARAMETERS_BLOB_TYPE(&mut self, v: ::std::string::String) {
        self.PARAMETERS_BLOB_TYPE = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_PARAMETERS_BLOB_TYPE(&mut self) -> &mut ::std::string::String {
        if self.PARAMETERS_BLOB_TYPE.is_none() {
            self.PARAMETERS_BLOB_TYPE.set_default();
        }
        self.PARAMETERS_BLOB_TYPE.as_mut().unwrap()
    }

    // Take field
    pub fn take_PARAMETERS_BLOB_TYPE(&mut self) -> ::std::string::String {
        self.PARAMETERS_BLOB_TYPE.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string INPUTS_BLOB_TYPE = 4;


    pub fn get_INPUTS_BLOB_TYPE(&self) -> &str {
        match self.INPUTS_BLOB_TYPE.as_ref() {
            Some(v) => &v,
            None => "INPUTS_BLOB_TYPE",
        }
    }
    pub fn clear_INPUTS_BLOB_TYPE(&mut self) {
        self.INPUTS_BLOB_TYPE.clear();
    }

    pub fn has_INPUTS_BLOB_TYPE(&self) -> bool {
        self.INPUTS_BLOB_TYPE.is_some()
    }

    // Param is passed by value, moved
    pub fn set_INPUTS_BLOB_TYPE(&mut self, v: ::std::string::String) {
        self.INPUTS_BLOB_TYPE = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_INPUTS_BLOB_TYPE(&mut self) -> &mut ::std::string::String {
        if self.INPUTS_BLOB_TYPE.is_none() {
            self.INPUTS_BLOB_TYPE.set_default();
        }
        self.INPUTS_BLOB_TYPE.as_mut().unwrap()
    }

    // Take field
    pub fn take_INPUTS_BLOB_TYPE(&mut self) -> ::std::string::String {
        self.INPUTS_BLOB_TYPE.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string OUTPUTS_BLOB_TYPE = 5;


    pub fn get_OUTPUTS_BLOB_TYPE(&self) -> &str {
        match self.OUTPUTS_BLOB_TYPE.as_ref() {
            Some(v) => &v,
            None => "OUTPUTS_BLOB_TYPE",
        }
    }
    pub fn clear_OUTPUTS_BLOB_TYPE(&mut self) {
        self.OUTPUTS_BLOB_TYPE.clear();
    }

    pub fn has_OUTPUTS_BLOB_TYPE(&self) -> bool {
        self.OUTPUTS_BLOB_TYPE.is_some()
    }

    // Param is passed by value, moved
    pub fn set_OUTPUTS_BLOB_TYPE(&mut self, v: ::std::string::String) {
        self.OUTPUTS_BLOB_TYPE = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_OUTPUTS_BLOB_TYPE(&mut self) -> &mut ::std::string::String {
        if self.OUTPUTS_BLOB_TYPE.is_none() {
            self.OUTPUTS_BLOB_TYPE.set_default();
        }
        self.OUTPUTS_BLOB_TYPE.as_mut().unwrap()
    }

    // Take field
    pub fn take_OUTPUTS_BLOB_TYPE(&mut self) -> ::std::string::String {
        self.OUTPUTS_BLOB_TYPE.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string GLOBAL_INIT_NET_TYPE = 6;


    pub fn get_GLOBAL_INIT_NET_TYPE(&self) -> &str {
        match self.GLOBAL_INIT_NET_TYPE.as_ref() {
            Some(v) => &v,
            None => "GLOBAL_INIT_NET_TYPE",
        }
    }
    pub fn clear_GLOBAL_INIT_NET_TYPE(&mut self) {
        self.GLOBAL_INIT_NET_TYPE.clear();
    }

    pub fn has_GLOBAL_INIT_NET_TYPE(&self) -> bool {
        self.GLOBAL_INIT_NET_TYPE.is_some()
    }

    // Param is passed by value, moved
    pub fn set_GLOBAL_INIT_NET_TYPE(&mut self, v: ::std::string::String) {
        self.GLOBAL_INIT_NET_TYPE = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_GLOBAL_INIT_NET_TYPE(&mut self) -> &mut ::std::string::String {
        if self.GLOBAL_INIT_NET_TYPE.is_none() {
            self.GLOBAL_INIT_NET_TYPE.set_default();
        }
        self.GLOBAL_INIT_NET_TYPE.as_mut().unwrap()
    }

    // Take field
    pub fn take_GLOBAL_INIT_NET_TYPE(&mut self) -> ::std::string::String {
        self.GLOBAL_INIT_NET_TYPE.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string PREDICT_INIT_NET_TYPE = 7;


    pub fn get_PREDICT_INIT_NET_TYPE(&self) -> &str {
        match self.PREDICT_INIT_NET_TYPE.as_ref() {
            Some(v) => &v,
            None => "PREDICT_INIT_NET_TYPE",
        }
    }
    pub fn clear_PREDICT_INIT_NET_TYPE(&mut self) {
        self.PREDICT_INIT_NET_TYPE.clear();
    }

    pub fn has_PREDICT_INIT_NET_TYPE(&self) -> bool {
        self.PREDICT_INIT_NET_TYPE.is_some()
    }

    // Param is passed by value, moved
    pub fn set_PREDICT_INIT_NET_TYPE(&mut self, v: ::std::string::String) {
        self.PREDICT_INIT_NET_TYPE = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_PREDICT_INIT_NET_TYPE(&mut self) -> &mut ::std::string::String {
        if self.PREDICT_INIT_NET_TYPE.is_none() {
            self.PREDICT_INIT_NET_TYPE.set_default();
        }
        self.PREDICT_INIT_NET_TYPE.as_mut().unwrap()
    }

    // Take field
    pub fn take_PREDICT_INIT_NET_TYPE(&mut self) -> ::std::string::String {
        self.PREDICT_INIT_NET_TYPE.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string PREDICT_NET_TYPE = 8;


    pub fn get_PREDICT_NET_TYPE(&self) -> &str {
        match self.PREDICT_NET_TYPE.as_ref() {
            Some(v) => &v,
            None => "PREDICT_NET_TYPE",
        }
    }
    pub fn clear_PREDICT_NET_TYPE(&mut self) {
        self.PREDICT_NET_TYPE.clear();
    }

    pub fn has_PREDICT_NET_TYPE(&self) -> bool {
        self.PREDICT_NET_TYPE.is_some()
    }

    // Param is passed by value, moved
    pub fn set_PREDICT_NET_TYPE(&mut self, v: ::std::string::String) {
        self.PREDICT_NET_TYPE = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_PREDICT_NET_TYPE(&mut self) -> &mut ::std::string::String {
        if self.PREDICT_NET_TYPE.is_none() {
            self.PREDICT_NET_TYPE.set_default();
        }
        self.PREDICT_NET_TYPE.as_mut().unwrap()
    }

    // Take field
    pub fn take_PREDICT_NET_TYPE(&mut self) -> ::std::string::String {
        self.PREDICT_NET_TYPE.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string SINGLE_PREDICTOR = 9;


    pub fn get_SINGLE_PREDICTOR(&self) -> &str {
        match self.SINGLE_PREDICTOR.as_ref() {
            Some(v) => &v,
            None => "SINGLE_PREDICTOR",
        }
    }
    pub fn clear_SINGLE_PREDICTOR(&mut self) {
        self.SINGLE_PREDICTOR.clear();
    }

    pub fn has_SINGLE_PREDICTOR(&self) -> bool {
        self.SINGLE_PREDICTOR.is_some()
    }

    // Param is passed by value, moved
    pub fn set_SINGLE_PREDICTOR(&mut self, v: ::std::string::String) {
        self.SINGLE_PREDICTOR = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_SINGLE_PREDICTOR(&mut self) -> &mut ::std::string::String {
        if self.SINGLE_PREDICTOR.is_none() {
            self.SINGLE_PREDICTOR.set_default();
        }
        self.SINGLE_PREDICTOR.as_mut().unwrap()
    }

    // Take field
    pub fn take_SINGLE_PREDICTOR(&mut self) -> ::std::string::String {
        self.SINGLE_PREDICTOR.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string MULTI_PREDICTOR = 10;


    pub fn get_MULTI_PREDICTOR(&self) -> &str {
        match self.MULTI_PREDICTOR.as_ref() {
            Some(v) => &v,
            None => "MULTI_PREDICTOR",
        }
    }
    pub fn clear_MULTI_PREDICTOR(&mut self) {
        self.MULTI_PREDICTOR.clear();
    }

    pub fn has_MULTI_PREDICTOR(&self) -> bool {
        self.MULTI_PREDICTOR.is_some()
    }

    // Param is passed by value, moved
    pub fn set_MULTI_PREDICTOR(&mut self, v: ::std::string::String) {
        self.MULTI_PREDICTOR = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_MULTI_PREDICTOR(&mut self) -> &mut ::std::string::String {
        if self.MULTI_PREDICTOR.is_none() {
            self.MULTI_PREDICTOR.set_default();
        }
        self.MULTI_PREDICTOR.as_mut().unwrap()
    }

    // Take field
    pub fn take_MULTI_PREDICTOR(&mut self) -> ::std::string::String {
        self.MULTI_PREDICTOR.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string TRAIN_INIT_PLAN_TYPE = 11;


    pub fn get_TRAIN_INIT_PLAN_TYPE(&self) -> &str {
        match self.TRAIN_INIT_PLAN_TYPE.as_ref() {
            Some(v) => &v,
            None => "TRAIN_INIT_PLAN_TYPE",
        }
    }
    pub fn clear_TRAIN_INIT_PLAN_TYPE(&mut self) {
        self.TRAIN_INIT_PLAN_TYPE.clear();
    }

    pub fn has_TRAIN_INIT_PLAN_TYPE(&self) -> bool {
        self.TRAIN_INIT_PLAN_TYPE.is_some()
    }

    // Param is passed by value, moved
    pub fn set_TRAIN_INIT_PLAN_TYPE(&mut self, v: ::std::string::String) {
        self.TRAIN_INIT_PLAN_TYPE = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_TRAIN_INIT_PLAN_TYPE(&mut self) -> &mut ::std::string::String {
        if self.TRAIN_INIT_PLAN_TYPE.is_none() {
            self.TRAIN_INIT_PLAN_TYPE.set_default();
        }
        self.TRAIN_INIT_PLAN_TYPE.as_mut().unwrap()
    }

    // Take field
    pub fn take_TRAIN_INIT_PLAN_TYPE(&mut self) -> ::std::string::String {
        self.TRAIN_INIT_PLAN_TYPE.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string TRAIN_PLAN_TYPE = 12;


    pub fn get_TRAIN_PLAN_TYPE(&self) -> &str {
        match self.TRAIN_PLAN_TYPE.as_ref() {
            Some(v) => &v,
            None => "TRAIN_PLAN_TYPE",
        }
    }
    pub fn clear_TRAIN_PLAN_TYPE(&mut self) {
        self.TRAIN_PLAN_TYPE.clear();
    }

    pub fn has_TRAIN_PLAN_TYPE(&self) -> bool {
        self.TRAIN_PLAN_TYPE.is_some()
    }

    // Param is passed by value, moved
    pub fn set_TRAIN_PLAN_TYPE(&mut self, v: ::std::string::String) {
        self.TRAIN_PLAN_TYPE = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_TRAIN_PLAN_TYPE(&mut self) -> &mut ::std::string::String {
        if self.TRAIN_PLAN_TYPE.is_none() {
            self.TRAIN_PLAN_TYPE.set_default();
        }
        self.TRAIN_PLAN_TYPE.as_mut().unwrap()
    }

    // Take field
    pub fn take_TRAIN_PLAN_TYPE(&mut self) -> ::std::string::String {
        self.TRAIN_PLAN_TYPE.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string SHAPE_INFO_BLOB = 13;


    pub fn get_SHAPE_INFO_BLOB(&self) -> &str {
        match self.SHAPE_INFO_BLOB.as_ref() {
            Some(v) => &v,
            None => "SHAPE_INFO_BLOB",
        }
    }
    pub fn clear_SHAPE_INFO_BLOB(&mut self) {
        self.SHAPE_INFO_BLOB.clear();
    }

    pub fn has_SHAPE_INFO_BLOB(&self) -> bool {
        self.SHAPE_INFO_BLOB.is_some()
    }

    // Param is passed by value, moved
    pub fn set_SHAPE_INFO_BLOB(&mut self, v: ::std::string::String) {
        self.SHAPE_INFO_BLOB = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_SHAPE_INFO_BLOB(&mut self) -> &mut ::std::string::String {
        if self.SHAPE_INFO_BLOB.is_none() {
            self.SHAPE_INFO_BLOB.set_default();
        }
        self.SHAPE_INFO_BLOB.as_mut().unwrap()
    }

    // Take field
    pub fn take_SHAPE_INFO_BLOB(&mut self) -> ::std::string::String {
        self.SHAPE_INFO_BLOB.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string DEFERRED_BLOB_READER = 14;


    pub fn get_DEFERRED_BLOB_READER(&self) -> &str {
        match self.DEFERRED_BLOB_READER.as_ref() {
            Some(v) => &v,
            None => "__DEFERRED_BLOB_READER__",
        }
    }
    pub fn clear_DEFERRED_BLOB_READER(&mut self) {
        self.DEFERRED_BLOB_READER.clear();
    }

    pub fn has_DEFERRED_BLOB_READER(&self) -> bool {
        self.DEFERRED_BLOB_READER.is_some()
    }

    // Param is passed by value, moved
    pub fn set_DEFERRED_BLOB_READER(&mut self, v: ::std::string::String) {
        self.DEFERRED_BLOB_READER = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_DEFERRED_BLOB_READER(&mut self) -> &mut ::std::string::String {
        if self.DEFERRED_BLOB_READER.is_none() {
            self.DEFERRED_BLOB_READER.set_default();
        }
        self.DEFERRED_BLOB_READER.as_mut().unwrap()
    }

    // Take field
    pub fn take_DEFERRED_BLOB_READER(&mut self) -> ::std::string::String {
        self.DEFERRED_BLOB_READER.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for PredictorConsts {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.META_NET_DEF)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.PREDICTOR_DBREADER)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.PARAMETERS_BLOB_TYPE)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.INPUTS_BLOB_TYPE)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.OUTPUTS_BLOB_TYPE)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.GLOBAL_INIT_NET_TYPE)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.PREDICT_INIT_NET_TYPE)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.PREDICT_NET_TYPE)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.SINGLE_PREDICTOR)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.MULTI_PREDICTOR)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.TRAIN_INIT_PLAN_TYPE)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.TRAIN_PLAN_TYPE)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.SHAPE_INFO_BLOB)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.DEFERRED_BLOB_READER)?;
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
        if let Some(ref v) = self.META_NET_DEF.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.PREDICTOR_DBREADER.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.PARAMETERS_BLOB_TYPE.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.INPUTS_BLOB_TYPE.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.OUTPUTS_BLOB_TYPE.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.GLOBAL_INIT_NET_TYPE.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.PREDICT_INIT_NET_TYPE.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(ref v) = self.PREDICT_NET_TYPE.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(ref v) = self.SINGLE_PREDICTOR.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        }
        if let Some(ref v) = self.MULTI_PREDICTOR.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(ref v) = self.TRAIN_INIT_PLAN_TYPE.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        }
        if let Some(ref v) = self.TRAIN_PLAN_TYPE.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        }
        if let Some(ref v) = self.SHAPE_INFO_BLOB.as_ref() {
            my_size += ::protobuf::rt::string_size(13, &v);
        }
        if let Some(ref v) = self.DEFERRED_BLOB_READER.as_ref() {
            my_size += ::protobuf::rt::string_size(14, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.META_NET_DEF.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.PREDICTOR_DBREADER.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.PARAMETERS_BLOB_TYPE.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.INPUTS_BLOB_TYPE.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.OUTPUTS_BLOB_TYPE.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.GLOBAL_INIT_NET_TYPE.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.PREDICT_INIT_NET_TYPE.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(ref v) = self.PREDICT_NET_TYPE.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(ref v) = self.SINGLE_PREDICTOR.as_ref() {
            os.write_string(9, &v)?;
        }
        if let Some(ref v) = self.MULTI_PREDICTOR.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(ref v) = self.TRAIN_INIT_PLAN_TYPE.as_ref() {
            os.write_string(11, &v)?;
        }
        if let Some(ref v) = self.TRAIN_PLAN_TYPE.as_ref() {
            os.write_string(12, &v)?;
        }
        if let Some(ref v) = self.SHAPE_INFO_BLOB.as_ref() {
            os.write_string(13, &v)?;
        }
        if let Some(ref v) = self.DEFERRED_BLOB_READER.as_ref() {
            os.write_string(14, &v)?;
        }
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PredictorConsts {
        PredictorConsts::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "META_NET_DEF",
                |m: &PredictorConsts| { &m.META_NET_DEF },
                |m: &mut PredictorConsts| { &mut m.META_NET_DEF },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "PREDICTOR_DBREADER",
                |m: &PredictorConsts| { &m.PREDICTOR_DBREADER },
                |m: &mut PredictorConsts| { &mut m.PREDICTOR_DBREADER },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "PARAMETERS_BLOB_TYPE",
                |m: &PredictorConsts| { &m.PARAMETERS_BLOB_TYPE },
                |m: &mut PredictorConsts| { &mut m.PARAMETERS_BLOB_TYPE },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "INPUTS_BLOB_TYPE",
                |m: &PredictorConsts| { &m.INPUTS_BLOB_TYPE },
                |m: &mut PredictorConsts| { &mut m.INPUTS_BLOB_TYPE },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "OUTPUTS_BLOB_TYPE",
                |m: &PredictorConsts| { &m.OUTPUTS_BLOB_TYPE },
                |m: &mut PredictorConsts| { &mut m.OUTPUTS_BLOB_TYPE },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "GLOBAL_INIT_NET_TYPE",
                |m: &PredictorConsts| { &m.GLOBAL_INIT_NET_TYPE },
                |m: &mut PredictorConsts| { &mut m.GLOBAL_INIT_NET_TYPE },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "PREDICT_INIT_NET_TYPE",
                |m: &PredictorConsts| { &m.PREDICT_INIT_NET_TYPE },
                |m: &mut PredictorConsts| { &mut m.PREDICT_INIT_NET_TYPE },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "PREDICT_NET_TYPE",
                |m: &PredictorConsts| { &m.PREDICT_NET_TYPE },
                |m: &mut PredictorConsts| { &mut m.PREDICT_NET_TYPE },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "SINGLE_PREDICTOR",
                |m: &PredictorConsts| { &m.SINGLE_PREDICTOR },
                |m: &mut PredictorConsts| { &mut m.SINGLE_PREDICTOR },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "MULTI_PREDICTOR",
                |m: &PredictorConsts| { &m.MULTI_PREDICTOR },
                |m: &mut PredictorConsts| { &mut m.MULTI_PREDICTOR },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "TRAIN_INIT_PLAN_TYPE",
                |m: &PredictorConsts| { &m.TRAIN_INIT_PLAN_TYPE },
                |m: &mut PredictorConsts| { &mut m.TRAIN_INIT_PLAN_TYPE },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "TRAIN_PLAN_TYPE",
                |m: &PredictorConsts| { &m.TRAIN_PLAN_TYPE },
                |m: &mut PredictorConsts| { &mut m.TRAIN_PLAN_TYPE },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "SHAPE_INFO_BLOB",
                |m: &PredictorConsts| { &m.SHAPE_INFO_BLOB },
                |m: &mut PredictorConsts| { &mut m.SHAPE_INFO_BLOB },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "DEFERRED_BLOB_READER",
                |m: &PredictorConsts| { &m.DEFERRED_BLOB_READER },
                |m: &mut PredictorConsts| { &mut m.DEFERRED_BLOB_READER },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PredictorConsts>(
                "PredictorConsts",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PredictorConsts {
        static instance: ::protobuf::rt::LazyV2<PredictorConsts> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PredictorConsts::new)
    }
}

impl ::protobuf::Clear for PredictorConsts {
    fn clear(&mut self) {
        self.META_NET_DEF.clear();
        self.PREDICTOR_DBREADER.clear();
        self.PARAMETERS_BLOB_TYPE.clear();
        self.INPUTS_BLOB_TYPE.clear();
        self.OUTPUTS_BLOB_TYPE.clear();
        self.GLOBAL_INIT_NET_TYPE.clear();
        self.PREDICT_INIT_NET_TYPE.clear();
        self.PREDICT_NET_TYPE.clear();
        self.SINGLE_PREDICTOR.clear();
        self.MULTI_PREDICTOR.clear();
        self.TRAIN_INIT_PLAN_TYPE.clear();
        self.TRAIN_PLAN_TYPE.clear();
        self.SHAPE_INFO_BLOB.clear();
        self.DEFERRED_BLOB_READER.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PredictorConsts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PredictorConsts {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n+pytorch/caffe2/proto/predictor_consts.proto\x12\x06caffe2\"\x94\x07\n\
    \x0fPredictorConsts\x120\n\x0cMETA_NET_DEF\x18\x01\x20\x01(\t:\x0e!!META\
    _NET_DEFR\nMETANETDEF\x12C\n\x12PREDICTOR_DBREADER\x18\x02\x20\x01(\t:\
    \x14!!PREDICTOR_DBREADERR\x11PREDICTORDBREADER\x12F\n\x14PARAMETERS_BLOB\
    _TYPE\x18\x03\x20\x01(\t:\x14PARAMETERS_BLOB_TYPER\x12PARAMETERSBLOBTYPE\
    \x12:\n\x10INPUTS_BLOB_TYPE\x18\x04\x20\x01(\t:\x10INPUTS_BLOB_TYPER\x0e\
    INPUTSBLOBTYPE\x12=\n\x11OUTPUTS_BLOB_TYPE\x18\x05\x20\x01(\t:\x11OUTPUT\
    S_BLOB_TYPER\x0fOUTPUTSBLOBTYPE\x12E\n\x14GLOBAL_INIT_NET_TYPE\x18\x06\
    \x20\x01(\t:\x14GLOBAL_INIT_NET_TYPER\x11GLOBALINITNETTYPE\x12H\n\x15PRE\
    DICT_INIT_NET_TYPE\x18\x07\x20\x01(\t:\x15PREDICT_INIT_NET_TYPER\x12PRED\
    ICTINITNETTYPE\x12:\n\x10PREDICT_NET_TYPE\x18\x08\x20\x01(\t:\x10PREDICT\
    _NET_TYPER\x0ePREDICTNETTYPE\x12;\n\x10SINGLE_PREDICTOR\x18\t\x20\x01(\t\
    :\x10SINGLE_PREDICTORR\x0fSINGLEPREDICTOR\x128\n\x0fMULTI_PREDICTOR\x18\
    \n\x20\x01(\t:\x0fMULTI_PREDICTORR\x0eMULTIPREDICTOR\x12E\n\x14TRAIN_INI\
    T_PLAN_TYPE\x18\x0b\x20\x01(\t:\x14TRAIN_INIT_PLAN_TYPER\x11TRAININITPLA\
    NTYPE\x127\n\x0fTRAIN_PLAN_TYPE\x18\x0c\x20\x01(\t:\x0fTRAIN_PLAN_TYPER\
    \rTRAINPLANTYPE\x127\n\x0fSHAPE_INFO_BLOB\x18\r\x20\x01(\t:\x0fSHAPE_INF\
    O_BLOBR\rSHAPEINFOBLOB\x12J\n\x14DEFERRED_BLOB_READER\x18\x0e\x20\x01(\t\
    :\x18__DEFERRED_BLOB_READER__R\x12DEFERREDBLOBREADERJ\xfb\r\n\x06\x12\
    \x04\0\0\"\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\
    \x02\0\x0f\n\n\n\x02\x04\0\x12\x04\x04\0\"\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\x04\x08\x17\n\x8f\x01\n\x04\x04\0\x02\0\x12\x03\x07\x02B\x1a\x81\
    \x01\x20Important\x20-\x20to\x20ensure\x20ordered\x20traversal\x20of\x20\
    the\x20DB,\x20these\x20must\x20be\n\x20set\x20in\x20the\x20given\x20(lex\
    icographic)\x20order\x20in\x20the\x20input\x20DBReader.\n\n\x0c\n\x05\
    \x04\0\x02\0\x04\x12\x03\x07\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x07\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x07\x12\x1e\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x07!\"\n\x0c\n\x05\x04\0\x02\0\x08\x12\x03\x07#\
    A\n\x0c\n\x05\x04\0\x02\0\x07\x12\x03\x07/?\nz\n\x04\x04\0\x02\x01\x12\
    \x03\x0c\x02N2m\x20The\x20key\x20the\x20Predictor\x20sets\x20in\x20the\
    \x20global\x20workspace\x20for\x20DBReader\n\x20consumed\x20by\x20the\
    \x20LoadOp\x20in\x20GLOBAL_INIT_NET.\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\
    \x03\x0c\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0c\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\x0c\x12$\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x0c'(\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x03\x0c)M\n\x0c\n\x05\
    \x04\0\x02\x01\x07\x12\x03\x0c5K\n2\n\x04\x04\0\x02\x02\x12\x03\x0f\x02P\
    \x1a%\x20Blob\x20types\x20used\x20in\x20MetaNetDef\x20blobs\n\n\x0c\n\
    \x05\x04\0\x02\x02\x04\x12\x03\x0f\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\
    \x12\x03\x0f\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0f\x12&\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0f)*\n\x0c\n\x05\x04\0\x02\x02\x08\
    \x12\x03\x0f+O\n\x0c\n\x05\x04\0\x02\x02\x07\x12\x03\x0f7M\n\x0b\n\x04\
    \x04\0\x02\x03\x12\x03\x10\x02H\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\
    \x10\x02\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x10\x0b\x11\n\x0c\n\x05\
    \x04\0\x02\x03\x01\x12\x03\x10\x12\"\n\x0c\n\x05\x04\0\x02\x03\x03\x12\
    \x03\x10%&\n\x0c\n\x05\x04\0\x02\x03\x08\x12\x03\x10'G\n\x0c\n\x05\x04\0\
    \x02\x03\x07\x12\x03\x103E\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x11\x02J\n\
    \x0c\n\x05\x04\0\x02\x04\x04\x12\x03\x11\x02\n\n\x0c\n\x05\x04\0\x02\x04\
    \x05\x12\x03\x11\x0b\x11\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x11\x12#\
    \n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x11&'\n\x0c\n\x05\x04\0\x02\x04\
    \x08\x12\x03\x11(I\n\x0c\n\x05\x04\0\x02\x04\x07\x12\x03\x114G\n0\n\x04\
    \x04\0\x02\x05\x12\x03\x14\x02P\x1a#\x20Net\x20types\x20used\x20in\x20Me\
    taNetDef\x20nets\n\n\x0c\n\x05\x04\0\x02\x05\x04\x12\x03\x14\x02\n\n\x0c\
    \n\x05\x04\0\x02\x05\x05\x12\x03\x14\x0b\x11\n\x0c\n\x05\x04\0\x02\x05\
    \x01\x12\x03\x14\x12&\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x14)*\n\x0c\
    \n\x05\x04\0\x02\x05\x08\x12\x03\x14+O\n\x0c\n\x05\x04\0\x02\x05\x07\x12\
    \x03\x147M\n\x0c\n\x04\x04\0\x02\x06\x12\x04\x15\x02\x16,\n\x0c\n\x05\
    \x04\0\x02\x06\x04\x12\x03\x15\x02\n\n\x0c\n\x05\x04\0\x02\x06\x05\x12\
    \x03\x15\x0b\x11\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x15\x12'\n\x0c\n\
    \x05\x04\0\x02\x06\x03\x12\x03\x15*+\n\x0c\n\x05\x04\0\x02\x06\x08\x12\
    \x03\x16\x06+\n\x0c\n\x05\x04\0\x02\x06\x07\x12\x03\x16\x12)\n\x0b\n\x04\
    \x04\0\x02\x07\x12\x03\x17\x02H\n\x0c\n\x05\x04\0\x02\x07\x04\x12\x03\
    \x17\x02\n\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x03\x17\x0b\x11\n\x0c\n\x05\
    \x04\0\x02\x07\x01\x12\x03\x17\x12\"\n\x0c\n\x05\x04\0\x02\x07\x03\x12\
    \x03\x17%&\n\x0c\n\x05\x04\0\x02\x07\x08\x12\x03\x17'G\n\x0c\n\x05\x04\0\
    \x02\x07\x07\x12\x03\x173E\n\x0b\n\x04\x04\0\x02\x08\x12\x03\x18\x02H\n\
    \x0c\n\x05\x04\0\x02\x08\x04\x12\x03\x18\x02\n\n\x0c\n\x05\x04\0\x02\x08\
    \x05\x12\x03\x18\x0b\x11\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x18\x12\"\
    \n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03\x18%&\n\x0c\n\x05\x04\0\x02\x08\
    \x08\x12\x03\x18'G\n\x0c\n\x05\x04\0\x02\x08\x07\x12\x03\x183E\n\x0b\n\
    \x04\x04\0\x02\t\x12\x03\x19\x02G\n\x0c\n\x05\x04\0\x02\t\x04\x12\x03\
    \x19\x02\n\n\x0c\n\x05\x04\0\x02\t\x05\x12\x03\x19\x0b\x11\n\x0c\n\x05\
    \x04\0\x02\t\x01\x12\x03\x19\x12!\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03\
    \x19$&\n\x0c\n\x05\x04\0\x02\t\x08\x12\x03\x19'F\n\x0c\n\x05\x04\0\x02\t\
    \x07\x12\x03\x193D\n\x0c\n\x04\x04\0\x02\n\x12\x04\x1a\x02\x1b+\n\x0c\n\
    \x05\x04\0\x02\n\x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\0\x02\n\x05\x12\
    \x03\x1a\x0b\x11\n\x0c\n\x05\x04\0\x02\n\x01\x12\x03\x1a\x12&\n\x0c\n\
    \x05\x04\0\x02\n\x03\x12\x03\x1a)+\n\x0c\n\x05\x04\0\x02\n\x08\x12\x03\
    \x1b\x06*\n\x0c\n\x05\x04\0\x02\n\x07\x12\x03\x1b\x12(\n\x0b\n\x04\x04\0\
    \x02\x0b\x12\x03\x1c\x02G\n\x0c\n\x05\x04\0\x02\x0b\x04\x12\x03\x1c\x02\
    \n\n\x0c\n\x05\x04\0\x02\x0b\x05\x12\x03\x1c\x0b\x11\n\x0c\n\x05\x04\0\
    \x02\x0b\x01\x12\x03\x1c\x12!\n\x0c\n\x05\x04\0\x02\x0b\x03\x12\x03\x1c$\
    &\n\x0c\n\x05\x04\0\x02\x0b\x08\x12\x03\x1c'F\n\x0c\n\x05\x04\0\x02\x0b\
    \x07\x12\x03\x1c3D\n#\n\x04\x04\0\x02\x0c\x12\x03\x1f\x02G\x1a\x16\x20Sh\
    ape\x20info\x20blob\x20name\n\n\x0c\n\x05\x04\0\x02\x0c\x04\x12\x03\x1f\
    \x02\n\n\x0c\n\x05\x04\0\x02\x0c\x05\x12\x03\x1f\x0b\x11\n\x0c\n\x05\x04\
    \0\x02\x0c\x01\x12\x03\x1f\x12!\n\x0c\n\x05\x04\0\x02\x0c\x03\x12\x03\
    \x1f$&\n\x0c\n\x05\x04\0\x02\x0c\x08\x12\x03\x1f'F\n\x0c\n\x05\x04\0\x02\
    \x0c\x07\x12\x03\x1f3D\n*\n\x04\x04\0\x02\r\x12\x03!\x02U\x1a\x1d\x20Seq\
    uential\x20blob\x20reader\x20name\n\n\x0c\n\x05\x04\0\x02\r\x04\x12\x03!\
    \x02\n\n\x0c\n\x05\x04\0\x02\r\x05\x12\x03!\x0b\x11\n\x0c\n\x05\x04\0\
    \x02\r\x01\x12\x03!\x12&\n\x0c\n\x05\x04\0\x02\r\x03\x12\x03!)+\n\x0c\n\
    \x05\x04\0\x02\r\x08\x12\x03!,T\n\x0c\n\x05\x04\0\x02\r\x07\x12\x03!8R\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
