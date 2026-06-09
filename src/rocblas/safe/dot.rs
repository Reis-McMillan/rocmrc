use super::{result, result::RocblasError, sys};
use crate::hip::DevicePtr;
use crate::rocblas::RocBlas;

#[derive(Debug, Copy, Clone)]
pub struct DotConfig {
    pub n: sys::rocblas_int,
    pub incx: sys::rocblas_int,
    pub incy: sys::rocblas_int,
}

pub trait Dot<T> {
    fn dot<X: DevicePtr<T>, Y: DevicePtr<T>>(
        &self,
        cfg: DotConfig,
        x: &X,
        y: &Y,
        result: &mut T,
    ) -> Result<(), RocblasError>;
}

impl Dot<f32> for RocBlas {
    fn dot<X: DevicePtr<f32>, Y: DevicePtr<f32>>(
        &self,
        cfg: DotConfig,
        x: &X,
        y: &Y,
        result: &mut f32,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr(&self.stream);
        let (y, _record_y) = y.device_ptr(&self.stream);
        unsafe {
            result::sdot(
                self.handle,
                cfg.n,
                x as *const _,
                cfg.incx,
                y as *const _,
                cfg.incy,
                result as *mut _,
            )
        }
    }
}

impl Dot<f64> for RocBlas {
    fn dot<X: DevicePtr<f64>, Y: DevicePtr<f64>>(
        &self,
        cfg: DotConfig,
        x: &X,
        y: &Y,
        result: &mut f64,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr(&self.stream);
        let (y, _record_y) = y.device_ptr(&self.stream);
        unsafe {
            result::ddot(
                self.handle,
                cfg.n,
                x as *const _,
                cfg.incx,
                y as *const _,
                cfg.incy,
                result as *mut _,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hip::HipContext;

    #[test]
    fn test_sdot() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        let x: [f32; 5] = [-0.5944882, 1.8055636, 0.52204555, -0.00397902, 0.39125723];
        let y: [f32; 5] = [1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938];
        let truth = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum::<f32>();

        let x_dev = stream.clone_htod(&x).unwrap();
        let y_dev = stream.clone_htod(&y).unwrap();
        let mut found = 0.0f32;
        blas.dot(
            DotConfig {
                n: 5,
                incx: 1,
                incy: 1,
            },
            &x_dev,
            &y_dev,
            &mut found,
        )
        .unwrap();
        assert!(
            (found - truth).abs() <= 1e-5,
            "found={found}, truth={truth}"
        );
    }

    #[test]
    fn test_ddot() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        let x: [f64; 5] = [-0.5944882, 1.8055636, 0.52204555, -0.00397902, 0.39125723];
        let y: [f64; 5] = [1.1292169, -0.13450263, 0.62789696, -0.5685516, 0.21946938];
        let truth = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum::<f64>();

        let x_dev = stream.clone_htod(&x).unwrap();
        let y_dev = stream.clone_htod(&y).unwrap();
        let mut found = 0.0f64;
        blas.dot(
            DotConfig {
                n: 5,
                incx: 1,
                incy: 1,
            },
            &x_dev,
            &y_dev,
            &mut found,
        )
        .unwrap();
        assert!(
            (found - truth).abs() <= 1e-12,
            "found={found}, truth={truth}"
        );
    }
}
