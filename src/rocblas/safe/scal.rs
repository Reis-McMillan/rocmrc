use super::{result, result::RocblasError, sys};
use crate::hip::DevicePtrMut;
use crate::rocblas::RocBlas;

#[derive(Debug, Copy, Clone)]
pub struct ScalConfig<T> {
    pub n: sys::rocblas_int,
    pub alpha: T,
    pub incx: sys::rocblas_int,
}

pub trait Scal<T> {
    /// `x := alpha*x`.
    fn scal<X: DevicePtrMut<T>>(
        &self,
        cfg: ScalConfig<T>,
        x: &mut X,
    ) -> Result<(), RocblasError>;
}

impl Scal<f32> for RocBlas {
    fn scal<X: DevicePtrMut<f32>>(
        &self,
        cfg: ScalConfig<f32>,
        x: &mut X,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr_mut(&self.stream);
        unsafe {
            result::sscal(self.handle, cfg.n, (&cfg.alpha) as *const _, x as *mut _, cfg.incx)
        }
    }
}

impl Scal<f64> for RocBlas {
    fn scal<X: DevicePtrMut<f64>>(
        &self,
        cfg: ScalConfig<f64>,
        x: &mut X,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr_mut(&self.stream);
        unsafe {
            result::dscal(self.handle, cfg.n, (&cfg.alpha) as *const _, x as *mut _, cfg.incx)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hip::HipContext;

    #[test]
    fn test_sscal() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        let alpha = -1.75f32;
        let x: [f32; 5] = [-0.5944882, 1.8055636, 0.52204555, -0.00397902, 0.39125723];
        let truth: Vec<f32> = x.iter().map(|v| alpha * v).collect();

        let mut x_dev = stream.clone_htod(&x).unwrap();
        blas.scal(ScalConfig { n: 5, alpha, incx: 1 }, &mut x_dev)
            .unwrap();

        let found = stream.clone_dtoh(&x_dev).unwrap();
        for i in 0..5 {
            assert!((found[i] - truth[i]).abs() <= 1e-5);
        }
    }

    #[test]
    fn test_dscal() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        let alpha = -1.75f64;
        let x: [f64; 5] = [-0.5944882, 1.8055636, 0.52204555, -0.00397902, 0.39125723];
        let truth: Vec<f64> = x.iter().map(|v| alpha * v).collect();

        let mut x_dev = stream.clone_htod(&x).unwrap();
        blas.scal(ScalConfig { n: 5, alpha, incx: 1 }, &mut x_dev)
            .unwrap();

        let found = stream.clone_dtoh(&x_dev).unwrap();
        for i in 0..5 {
            assert!((found[i] - truth[i]).abs() <= 1e-12);
        }
    }
}
