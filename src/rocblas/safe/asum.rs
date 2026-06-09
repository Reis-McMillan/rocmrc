use super::{result, result::RocblasError, sys};
use crate::hip::DevicePtr;
use crate::rocblas::RocBlas;

pub struct AsumConfig {
    pub n: sys::rocblas_int,
    pub incx: sys::rocblas_int,
}

pub trait Asum<T> {
    fn asum<X: DevicePtr<T>>(
        &self,
        cfg: AsumConfig,
        x: &X,
        result: &mut T,
    ) -> Result<(), RocblasError>;
}

impl Asum<f32> for RocBlas {
    fn asum<X: DevicePtr<f32>>(
        &self,
        cfg: AsumConfig,
        x: &X,
        result: &mut f32,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr(&self.stream);
        unsafe {
            result::sasum(
                self.handle,
                cfg.n,
                x as *const _,
                cfg.incx,
                result as *mut _,
            )
        }
    }
}

impl Asum<f64> for RocBlas {
    fn asum<X: DevicePtr<f64>>(
        &self,
        cfg: AsumConfig,
        x: &X,
        result: &mut f64,
    ) -> Result<(), RocblasError> {
        let (x, _record_x) = x.device_ptr(&self.stream);
        unsafe {
            result::dasum(
                self.handle,
                cfg.n,
                x as *const _,
                cfg.incx,
                result as *mut _,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_range_loop)]

    use super::*;
    use crate::hip::HipContext;
    use std::vec;

    fn dasum_truth(a: &[f64], c: &mut f64, n: usize, incx: usize) {
        *c = 0.0;
        for x in a.iter().step_by(incx).take(n) {
            *c += x.abs();
        }
    }
    fn sasum_truth(a: &[f32], c: &mut f32, n: usize, incx: usize) {
        *c = 0.0;
        for x in a.iter().step_by(incx).take(n) {
            *c += x.abs();
        }
    }

    #[test]
    fn test_sasum() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        #[rustfmt::skip]
        let samples = [
            (vec![-0.5944882f32, 1.8055636, 0.52204555, -0.00397902], 0.0f32, 4usize, 1usize),
            (vec![-0.5944882, 1.8055636, 0.52204555, -0.00397902], -1.0, 4, 1),
            (vec![-0.5944882, 1.8055636, 0.52204555, -0.00397902], 3.0, 4, 1),
            (vec![-0.38346434, -0.38013917, 0.4198623], 0.0, 3, 1),
            (vec![-1.6661372, -0.4568837, -0.9043474, 0.39125723, 0.39125723], 0.0, 5, 1)
        ];
        for ref out @ (ref x, result, n, incx) in samples.into_iter() {
            let mut actual = result;
            let mut expected = result;
            sasum_truth(x, &mut actual, n, incx);

            let x = stream.clone_htod(x).unwrap();
            blas.asum(
                AsumConfig {
                    n: n as i32,
                    incx: incx as i32,
                },
                &x,
                &mut expected,
            )
            .unwrap();
            let delta = (actual - expected).abs();
            let epsilon = 2.0 * f32::EPSILON;
            assert!(
                delta <= epsilon,
                "({actual} - {expected}).abs() -> {delta:+e} <= {epsilon:+e}: {out:?}"
            );
        }
    }

    #[test]
    fn test_dasum() {
        let ctx = HipContext::new(0).unwrap();
        let stream = ctx.default_stream();
        let blas = RocBlas::new(stream.clone()).unwrap();

        #[rustfmt::skip]
        let samples = [
            (vec![-0.5944882f64, 1.8055636, 0.52204555, -0.00397902], 0.0f64, 4usize, 1usize),
            (vec![-0.5944882, 1.8055636, 0.52204555, -0.00397902], -1.0, 4, 1),
            (vec![-0.5944882, 1.8055636, 0.52204555, -0.00397902], 3.0, 4, 1),
            (vec![-0.38346434, -0.38013917, 0.4198623], 0.0, 3, 1),
            (vec![-1.6661372, -0.4568837, -0.9043474, 0.39125723, 0.39125723], 0.0, 5, 1)
        ];
        for ref out @ (ref x, result, n, incx) in samples.into_iter() {
            let mut actual = result;
            let mut expected = result;
            dasum_truth(x, &mut actual, n, incx);

            let x = stream.clone_htod(x).unwrap();
            blas.asum(
                AsumConfig {
                    n: n as i32,
                    incx: incx as i32,
                },
                &x,
                &mut expected,
            )
            .unwrap();
            let delta = (actual - expected).abs();
            let epsilon = 2.0 * f64::EPSILON;
            assert!(
                delta <= epsilon,
                "({actual} - {expected}).abs() -> {delta:+e} <= {epsilon:+e}: {out:?}"
            );
        }
    }
}
