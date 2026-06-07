use super::{result, result::RocblasError, sys};
use crate::hip::{DevicePtr, DevicePtrMut};
use crate::rocblas::RocBlas;

#[derive(Debug, Copy, Clone)]
pub struct AxpyConfig<T> {
    pub n: sys::rocblas_int,
    pub alpha: T,
    pub incx: sys::rocblas_int,
    pub incy: sys::rocblas_int,
}

pub trait Axpy<T> {
    /// `y := alpha*x + y`.
    fn axpy<X: DevicePtr<T>, Y: DevicePtrMut<T>>(
        &self,
        cfg: AxpyConfig<T>,
        x: &X,
        y: &mut Y,
    ) -> Result<(), RocblasError>;
}

impl Axpy<f32> for RocBlas {
    fn axpy<X: DevicePtr<f32>, Y: DevicePtrMut<f32>>(
        &self,
        cfg: AxpyConfig<f32>,
        x: &X,
        y: &mut Y,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr(&self.stream);
        let (y, _record_y) = y.device_ptr_mut(&self.stream);
        unsafe {
            result::saxpy(
                self.handle,
                cfg.n,
                (&cfg.alpha) as *const _,
                x as *const _,
                cfg.incx,
                y as *mut _,
                cfg.incy,
            )
        }
    }
}

impl Axpy<f64> for RocBlas {
    fn axpy<X: DevicePtr<f64>, Y: DevicePtrMut<f64>>(
        &self,
        cfg: AxpyConfig<f64>,
        x: &X,
        y: &mut Y,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr(&self.stream);
        let (y, _record_y) = y.device_ptr_mut(&self.stream);
        unsafe {
            result::daxpy(
                self.handle,
                cfg.n,
                (&cfg.alpha) as *const _,
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
    fn test_saxpy() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        let alpha = 2.5f32;
        let x: [f32; 5] = [-0.5944882, 1.8055636, 0.52204555, -0.00397902, 0.39125723];
        let y: [f32; 5] = [1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938];
        let truth: Vec<f32> = x.iter().zip(y.iter()).map(|(a, b)| alpha * a + b).collect();

        let x_dev = stream.clone_htod(&x).unwrap();
        let mut y_dev = stream.clone_htod(&y).unwrap();
        blas.axpy(
            AxpyConfig {
                n: 5,
                alpha,
                incx: 1,
                incy: 1,
            },
            &x_dev,
            &mut y_dev,
        )
        .unwrap();

        let found = stream.clone_dtoh(&y_dev).unwrap();
        for i in 0..5 {
            assert!((found[i] - truth[i]).abs() <= 1e-5);
        }
    }

    #[test]
    fn test_daxpy() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        let alpha = 2.5f64;
        let x: [f64; 5] = [-0.5944882, 1.8055636, 0.52204555, -0.00397902, 0.39125723];
        let y: [f64; 5] = [1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938];
        let truth: Vec<f64> = x.iter().zip(y.iter()).map(|(a, b)| alpha * a + b).collect();

        let x_dev = stream.clone_htod(&x).unwrap();
        let mut y_dev = stream.clone_htod(&y).unwrap();
        blas.axpy(
            AxpyConfig {
                n: 5,
                alpha,
                incx: 1,
                incy: 1,
            },
            &x_dev,
            &mut y_dev,
        )
        .unwrap();

        let found = stream.clone_dtoh(&y_dev).unwrap();
        for i in 0..5 {
            assert!((found[i] - truth[i]).abs() <= 1e-12);
        }
    }
}
