// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

use etherdream::EtherdreamError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

/// Library errors. This can wrap the underlying DAC hardware/library errors.
#[derive(Debug)]
pub enum LaseError {
  /// Wraps Etherdream errors.
  EtherdreamError {
    /// Root cause of the error.
    cause: EtherdreamError
  },

  /// An operation unsupported by the DAC hardware was attempted.
  UnsupportedByDac,
}

impl Error for LaseError {
  fn description(&self) -> &str {
    match *self {
      LaseError::EtherdreamError { .. } => "EtherdreamError",
      LaseError::UnsupportedByDac => "UnsupportedByDac",
    }
  }
}

impl Display for LaseError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.description())
  }
}

impl From<EtherdreamError> for LaseError {
  fn from(error: EtherdreamError) -> Self {
    LaseError::EtherdreamError { cause: error }
  }
}
