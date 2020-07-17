pub use calcite_proc_macros::*;
pub use futures;
pub use rmp_serde;

pub trait FromZeroCopyBuf<'a> {
    fn from_zero_copy_buf(buff: &'a deno_core::plugin_api::ZeroCopyBuf) -> Self;
}

pub struct ArrayBuffer<'a, T>(&'a mut [T]);

impl<'a, T> ArrayBuffer<'a, T> {
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.0
    }
    pub fn as_slice(&self) -> &[T] {
        self.0
    }
}

impl<'a, T> FromZeroCopyBuf<'a> for ArrayBuffer<'a, T> {
    fn from_zero_copy_buf(buff: &deno_core::ZeroCopyBuf) -> Self {
        unsafe {
            Self(std::slice::from_raw_parts_mut(
                buff[..].as_ptr() as *mut T,
                buff.len() / std::mem::size_of::<T>(),
            ))
        }
    }
}

impl<'a, T: serde::Deserialize<'a>> FromZeroCopyBuf<'a> for T {
    fn from_zero_copy_buf(buff: &'a deno_core::ZeroCopyBuf) -> Self {
        rmp_serde::from_read_ref(buff).expect("Wrong argument type")
    }
}

pub fn to_argument_type<'a, T: FromZeroCopyBuf<'a>>(buff: &'a deno_core::ZeroCopyBuf) -> T {
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
