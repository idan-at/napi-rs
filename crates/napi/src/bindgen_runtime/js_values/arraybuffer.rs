use std::ffi::c_void;
use std::ptr;

pub use crate::js_values::TypedArrayType;
use crate::{check_status, sys, Error, Result, Status};

use super::{FromNapiValue, ToNapiValue, TypeName};

pub struct Int8Array {
  data: *mut c_void,
}

impl FromNapiValue for Int8Array {
  unsafe fn from_napi_value(
    env: napi_sys::napi_env,
    napi_val: napi_sys::napi_value,
  ) -> Result<Self> {
    let mut typed_array_type = 0;
    let mut len = 0;
    let mut data = ptr::null_mut();
    let mut array_buffer = ptr::null_mut();
    let mut byte_offset = 0;
    check_status!(
      sys::napi_get_typedarray_info(
        env,
        napi_val,
        &mut typed_array_type,
        &mut len,
        &mut data,
        &mut array_buffer,
        &mut byte_offset
      ),
      "Get TypedArray info failed"
    )?;
    if typed_array_type != TypedArrayType::Int8 as i32 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Expected Int8Array, got {}", typed_array_type),
      ));
    }
    Ok(Int8Array { data })
  }
}
