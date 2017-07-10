// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

//! Lase.rs is a library for controlling laser projector DACs. It is designed as
//! an abstraction layer so that the DAC used may be swapped out as needed.

#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_qualifications)]

extern crate etherdream;

mod dac;
mod error;

pub use dac::Dac;
pub use error::LaseError;
pub use etherdream::protocol::Point; // TODO: Move this into the 'lasers' lib.
