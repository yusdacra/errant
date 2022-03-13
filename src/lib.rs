#![feature(
    never_type,
    try_trait_v2,
    termination_trait_lib,
    process_exitcode_placeholder
)]

pub mod backtrace;
pub mod error;
pub mod result;

pub mod prelude {
    pub use super::result::{err as Err, TrackedResult::Ok};

    pub type Result<T, E> = super::result::TrackedResult<T, super::Backtrace<E>>;
}

use crate::{backtrace::Backtrace, error::Error};
pub use core::{
    fmt,
    ops::{ControlFlow, FromResidual, Range, Try},
    panic::Location,
};
