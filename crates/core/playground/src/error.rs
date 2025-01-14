use std::convert::Infallible;
use std::num::{ParseFloatError, ParseIntError};
pub use thiserror_impl::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {}
