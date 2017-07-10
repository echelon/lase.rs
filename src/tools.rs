// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

//! Various DAC-specific tools.

use dac::Dac;
use error::LaseError;
use etherdream::network::find_first_dac;
use etherdream::protocol;

// TODO: Move these.

/// Max color on Etherdream.
pub const ETHERDREAM_COLOR_MAX : u16 = protocol::COLOR_MAX;
/// Min color on Etherdream.
pub const ETHERDREAM_COLOR_MIN : u16 = protocol::COLOR_MIN;
/// Max X on Etherdream.
pub const ETHERDREAM_X_MAX : i16 = protocol::X_MAX;
/// Min X on Etherdream.
pub const ETHERDREAM_X_MIN : i16 = protocol::X_MIN;
/// Max Y on Etherdream.
pub const ETHERDREAM_Y_MAX : i16 = protocol::Y_MAX;
/// Min Y on Etherdream.
pub const ETHERDREAM_Y_MIN : i16 = protocol::Y_MIN;

/// Find the first Etherdream DAC on the network.
pub fn find_first_etherdream_dac() -> Result<Dac, LaseError> {
  let result = find_first_dac()?;
  Ok(Dac::etherdream(result.ip_address))
}
