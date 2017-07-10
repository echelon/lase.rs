// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

use error::LaseError;
use etherdream::protocol::Point;
use etherdream;
use std::net::IpAddr;

/// A hardware DAC used to communicate with a laser projector.
pub enum Dac {
  /// An Etherdream DAC.
  Etherdream {
    /// The wrapped 'etherdream.rs' DAC.
    dac: etherdream::dac::Dac
  },
}

impl Dac {
  /// Construct a new Etherdream DAC.
  pub fn etherdream(ip_addr: IpAddr) -> Dac {
    Dac::Etherdream { dac: etherdream::dac::Dac::new(ip_addr) }
  }

  /// Get the IP address, if the DAC communication is IP-based.
  pub fn get_ip_address(&self) -> Option<&IpAddr> {
    match self {
      &Dac::Etherdream { ref dac } => Some(&dac.get_ip_address()),
    }
  }

  /// Play a stream of points on the hardware.
  pub fn play_function<F>(&mut self, make_points: F)
      -> Result<(), LaseError> where F: FnMut(u16) -> Vec<Point> {

    match self {
      &mut Dac::Etherdream { ref mut dac } => {
        dac.play_function(make_points).map_err(|e| e.into())
      },
    }
  }
}