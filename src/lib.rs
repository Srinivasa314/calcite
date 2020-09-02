//! # Calcite
//!
//! Calcite is a library to create deno plugins.
//!
//! Refer the [docs](https://github.com/Srinivasa314/calcite/tree/master/docs)
//!

pub use calcite_proc_macros::*;
pub use futures;

pub trait FromZeroCopyBuf<'a> {
    fn from_zero_copy_buf(buff: &'a deno_core::plugin_api::ZeroCopyBuf) -> Self;
}

/// A mutable arraybuffer that can contain primitives like i32, u64, f32, etc.
pub struct ArrayBuffer<'a, T>(&'a mut [T]);

impl<'a, T> ArrayBuffer<'a, T> {
    /// Get a mutable slice of the array
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.0
    }
    /// Get an immutable slice of the array
    pub fn as_slice(&self) -> &[T] {
        self.0
    }
}

impl<'a, T> FromZeroCopyBuf<'a> for ArrayBuffer<'a, T> {
    fn from_zero_copy_buf(buff: &deno_core::plugin_api::ZeroCopyBuf) -> Self {
        unsafe {
            Self(std::slice::from_raw_parts_mut(
                buff[..].as_ptr() as *mut T,
                buff.len() / std::mem::size_of::<T>(),
            ))
        }
    }
}

impl<'a, T: serde::Deserialize<'a>> FromZeroCopyBuf<'a> for T {
    fn from_zero_copy_buf(buff: &'a deno_core::plugin_api::ZeroCopyBuf) -> Self {
        serde_json::from_slice(buff).expect("Wrong argument type")
    }
}

pub fn to_argument_type<'a, T: FromZeroCopyBuf<'a>>(buff: &'a deno_core::plugin_api::ZeroCopyBuf) -> T {
    T::from_zero_copy_buf(buff)
}

#[macro_use]
extern crate serde_derive;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncResult<T, E> {
    pub command_id: usize,
    pub result: Result<T, E>,
}

/// A struct used when returning a raw unserialized buffer
pub struct ReturnBuffer(Box<[u8]>);

impl<T: serde::Serialize> From<T> for ReturnBuffer {
    fn from(t: T) -> Self {
        Self(serde_json::to_vec(&t).unwrap().into_boxed_slice())
    }
}

impl ReturnBuffer {
    pub fn inner(self) -> Box<[u8]> {
        self.0
    }
    /// Construct a ReturnBuffer from bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes.into_boxed_slice())
    }
}
