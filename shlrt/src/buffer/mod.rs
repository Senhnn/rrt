mod io_buf;
pub use io_buf::{IoBuf, IoBufMut};

mod io_vec_buf;
pub use io_vec_buf::{IoVecBuf, IoVecBufMut, VecBuf};

mod slice;
pub use slice::{IoVecWrapper, IoVecWrapperMut, Slice, SliceMut};

mod raw_buf;
pub use raw_buf::{RawBuf, RawBufVectored};

mod vec_wrapper;
pub(crate) use vec_wrapper::{read_vec_meta, write_vec_meta};

pub(crate) fn deref(buf: &impl IoBuf) -> &[u8] {
    // Safety: the `IoBuf` trait is marked as unsafe and is expected to be
    // implemented correctly.
    unsafe { std::slice::from_raw_parts(buf.read_ptr(), buf.bytes_init()) }
}