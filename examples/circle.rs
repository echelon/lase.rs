// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

extern crate lase;

use lase::Dac;
use lase::tools::find_first_etherdream_dac;
use lase::Point; // TODO: Move to 'lasers.rs' library.
use lase::tools::ETHERDREAM_X_MAX; // TODO: Move these.
use lase::tools::ETHERDREAM_Y_MAX;
use std::f64::consts::PI;
use std::f64;

static DIV : i32 = 200;

fn main() {
  println!("Searching for DAC...");

  let mut dac = find_first_etherdream_dac().expect("Couldn't find DAC.");

  static mut pos: i32 = 0;

  let _r = dac.play_function(|num_points: u16| {
    let mut points = Vec::new();
    for _i in 0 .. num_points {
      let f = unsafe {
        pos = (pos + 1) % DIV;
        pos
      };

      let j = (f as f64 / DIV as f64) * 2 as f64 * PI;
      let x = j.cos() * ETHERDREAM_X_MAX as f64;
      let y = j.sin() * ETHERDREAM_Y_MAX as f64;

      let x = x as i16;
      let y = y as i16;

      points.push(Point::xy_binary(x, y, true));
    }

    points
  });
}

