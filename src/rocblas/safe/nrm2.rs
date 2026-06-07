use super::{result, result::RocblasError, sys};
use crate::hip::DevicePtr;
use crate::rocblas::RocBlas;

#[derive(Debug, Copy, Clone)]
pub struct Nrm2Config {
    pub n: sys::rocblas_int,
    pub incx: sys::rocblas_int,
}

pub trait Nrm2<T> {
    fn nrm2<X: DevicePtr<T>>(
        &self,
        cfg: Nrm2Config,
        x: &X,
        result: &mut T,
    ) -> Result<(), RocblasError>;
}

impl Nrm2<f32> for RocBlas {
    fn nrm2<X: DevicePtr<f32>>(
        &self,
        cfg: Nrm2Config,
        x: &X,
        result: &mut f32,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr(&self.stream);
        unsafe { result::snrm2(self.handle, cfg.n, x as *const _, cfg.incx, result as *mut _) }
    }
}

impl Nrm2<f64> for RocBlas {
    fn nrm2<X: DevicePtr<f64>>(
        &self,
        cfg: Nrm2Config,
        x: &X,
        result: &mut f64,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr(&self.stream);
        unsafe { result::dnrm2(self.handle, cfg.n, x as *const _, cfg.incx, result as *mut _) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hip::HipContext;

    #[test]
    fn test_snrm2() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        let x: [f32; 5] = [-0.5944882, 1.8055636, 0.52204555, -0.00397902, 0.39125723];
        let truth = x.iter().map(|v| v * v).sum::<f32>().sqrt();

        let x_dev = stream.clone_htod(&x).unwrap();
        let mut found = 0.0f32;
        blas.nrm2(Nrm2Config { n: 5, incx: 1 }, &x_dev, &mut found)
            .unwrap();
        assert!((found - truth).abs() <= 1e-5, "found={found}, truth={truth}");
    }

    #[test]
    fn test_dnrm2() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        let x: [f64; 5] = [-0.5944882, 1.8055636, 0.52204555, -0.00397902, 0.39125723];
        let truth = x.iter().map(|v| v * v).sum::<f64>().sqrt();

        let x_dev = stream.clone_htod(&x).unwrap();
        let mut found = 0.0f64;
        blas.nrm2(Nrm2Config { n: 5, incx: 1 }, &x_dev, &mut found)
            .unwrap();
        assert!((found - truth).abs() <= 1e-12, "found={found}, truth={truth}");
    }
}
