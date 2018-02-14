// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

//! Lase.rs is a library for controlling laser projector DACs. It is designed as
//! an abstraction layer so that the DAC used may be swapped out as needed.

#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_qualifications)]

extern crate etherdream;
extern crate point as pointlib;

mod dac;
mod error;

pub mod tools;

pub use dac::Dac;
pub use error::LaseError;

#[deprecated]
pub use etherdream::protocol::Point;

/// Module for common point types.
pub mod point {
  pub use pointlib::PipelinePoint;
  pub use pointlib::SimplePoint;
}
