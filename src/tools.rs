// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

//! Various DAC-specific tools.

use dac::Dac;
use error::LaseError;
use etherdream::network::find_first_dac;

/// Find the first Etherdream DAC on the network.
pub fn find_first_etherdream_dac() -> Result<Dac, LaseError> {
  let result = find_first_dac()?;
  Ok(Dac::etherdream(result.ip_address))
}
