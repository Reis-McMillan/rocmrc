use super::{result, result::RocblasError, sys};
use crate::hip::HipStream;
use std::sync::Arc;

mod asum;
mod axpy;
mod copy;
mod dot;
mod gemm;
mod gemv;
mod nrm2;
mod scal;

pub use asum::*;
pub use axpy::*;
pub use copy::*;
pub use dot::*;
pub use gemm::*;
pub use gemv::*;
pub use nrm2::*;
pub use scal::*;

pub struct RocBlas {
    pub(crate) handle: sys::rocblas_handle,
    pub(crate) stream: Arc<HipStream>,
}

unsafe impl Send for RocBlas {}
unsafe impl Sync for RocBlas {}

impl RocBlas {
    pub fn new(stream: Arc<HipStream>) -> Result<Self, RocblasError> {
        let ctx = stream.context();
        ctx.record_err(ctx.bind_to_thread());
        let handle = result::create_handle()?;
        unsafe { result::set_stream(handle, stream.hip_stream() as _) }?;
        let blas = Self { handle, stream };
        Ok(blas)
    }

    pub fn handle(&self) -> &sys::rocblas_handle {
        &self.handle
    }

    pub fn set_stream(&mut self, stream: Arc<HipStream>) -> Result<(), RocblasError> {
        self.stream = stream;
        unsafe { result::set_stream(self.handle, self.stream.hip_stream() as _) }
    }

    pub fn set_pointer_mode(
        &self,
        pointer_mode: sys::rocblas_pointer_mode,
    ) -> Result<(), RocblasError> {
        unsafe {
            sys::rocblas_set_pointer_mode(self.handle, pointer_mode).result()
        }
    }

    pub fn get_pointer_mode(&self) -> Result<sys::rocblas_pointer_mode, RocblasError> {
        unsafe {
            let mut mode = ::core::mem::MaybeUninit::uninit();
            sys::rocblas_get_pointer_mode(self.handle, mode.as_mut_ptr()).result()?;
            Ok(mode.assume_init())
        }
    }
}

impl Drop for RocBlas {
    fn drop(&mut self) {
        let handle = std::mem::replace(&mut self.handle, std::ptr::null_mut());
        if !handle.is_null() {
            unsafe { result::destroy_handle(handle) }.unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hip::HipContext;

    #[test]
    fn rocblas_pointer_mode() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();
        assert_eq!(
            blas.get_pointer_mode().unwrap(),
            sys::rocblas_pointer_mode::rocblas_pointer_mode_host,
            "The default pointer mode uses host pointers"
        );

        blas.set_pointer_mode(sys::rocblas_pointer_mode::rocblas_pointer_mode_device)
            .unwrap();
        assert_eq!(
            blas.get_pointer_mode().unwrap(),
            sys::rocblas_pointer_mode::rocblas_pointer_mode_device,
            "We have set the mode to use device pointers"
        );
    }
}