use super::{result, result::RocblasError, sys};
use crate::hip::{DevicePtr, DevicePtrMut};
use crate::rocblas::RocBlas;

#[derive(Debug, Copy, Clone)]
pub struct CopyConfig {
    pub n: sys::rocblas_int,
    pub incx: sys::rocblas_int,
    pub incy: sys::rocblas_int,
}

pub trait Copy<T> {
    /// `y := x`.
    fn copy<X: DevicePtr<T>, Y: DevicePtrMut<T>>(
        &self,
        cfg: CopyConfig,
        x: &X,
        y: &mut Y,
    ) -> Result<(), RocblasError>;
}

impl Copy<f32> for RocBlas {
    fn copy<X: DevicePtr<f32>, Y: DevicePtrMut<f32>>(
        &self,
        cfg: CopyConfig,
        x: &X,
        y: &mut Y,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr(&self.stream);
        let (y, _record_y) = y.device_ptr_mut(&self.stream);
        unsafe {
            result::scopy(
                self.handle,
                cfg.n,
                x as *const _,
                cfg.incx,
                y as *mut _,
                cfg.incy,
            )
        }
    }
}

impl Copy<f64> for RocBlas {
    fn copy<X: DevicePtr<f64>, Y: DevicePtrMut<f64>>(
        &self,
        cfg: CopyConfig,
        x: &X,
        y: &mut Y,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr(&self.stream);
        let (y, _record_y) = y.device_ptr_mut(&self.stream);
        unsafe {
            result::dcopy(
                self.handle,
                cfg.n,
                x as *const _,
                cfg.incx,
                y as *mut _,
                cfg.incy,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hip::HipContext;

    #[test]
    fn test_scopy() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        let x: [f32; 5] = [-0.5944882, 1.8055636, 0.52204555, -0.00397902, 0.39125723];
        let x_dev = stream.clone_htod(&x).unwrap();
        let mut y_dev = stream.alloc_zeros::<f32>(5).unwrap();
        blas.copy(
            CopyConfig {
                n: 5,
                incx: 1,
                incy: 1,
            },
            &x_dev,
            &mut y_dev,
        )
        .unwrap();

        let found = stream.clone_dtoh(&y_dev).unwrap();
        assert_eq!(found, x.to_vec());
    }

    #[test]
    fn test_dcopy() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        let x: [f64; 5] = [-0.5944882, 1.8055636, 0.52204555, -0.00397902, 0.39125723];
        let x_dev = stream.clone_htod(&x).unwrap();
        let mut y_dev = stream.alloc_zeros::<f64>(5).unwrap();
        blas.copy(
            CopyConfig {
                n: 5,
                incx: 1,
                incy: 1,
            },
            &x_dev,
            &mut y_dev,
        )
        .unwrap();

        let found = stream.clone_dtoh(&y_dev).unwrap();
        assert_eq!(found, x.to_vec());
    }
}
