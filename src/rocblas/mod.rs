pub mod result;
pub mod safe;
#[allow(warnings)]
#[rustfmt::skip]
pub mod sys;

pub use result::RocblasError;
pub use safe::*;
pub use sys::rocblas_operation as Operation;