// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

extern crate lase;

use lase::Dac;
use lase::Point; // TODO: Move to 'lasers.rs' library.
use lase::tools::ETHERDREAM_COLOR_MAX;
use lase::tools::ETHERDREAM_X_MAX; // TODO: Move these.
use lase::tools::ETHERDREAM_X_MIN;
use lase::tools::ETHERDREAM_Y_MAX;
use lase::tools::ETHERDREAM_Y_MIN;
use lase::tools::find_first_etherdream_dac;
use std::f64::consts::PI;
use std::f64;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

static DIV : i16 = 200;

/// A nifty circle.
struct Circle {
  // Color
  pub r: u16,
  pub g: u16,
  pub b: u16,
  // Size
  pub radius: i16,
  // Position
  pub x: i16,
  pub y: i16,
  // Velocity
  pub x_vel: i16,
  pub y_vel: i16,
}

impl Circle {
  pub fn new(radius: i16) -> Circle {
    Circle {
      r: ETHERDREAM_COLOR_MAX,
      g: ETHERDREAM_COLOR_MAX,
      b: 0,
      radius: radius,
      x: 0,
      y: 0,
      x_vel: 170,
      y_vel: 150,
    }
  }
}

impl Circle {
  pub fn get_points(&self, num_points: u16, position: &mut i16) -> Vec<Point> {
    let mut points = Vec::new();
    for _i in 0 .. num_points {
      *position = (*position + 1) % DIV;

      let j = (*position as f64 / DIV as f64) * 2 as f64 * PI;
      let x = j.cos() * self.radius as f64 + self.x as f64;
      let y = j.sin() * self.radius as f64 + self.y as f64;

      let x = x as i16;
      let y = y as i16;

      points.push(Point::xy_rgb(x, y, self.r, self.g, self.b))
    }
    points
  }

  pub fn animate(&mut self) {
    println!("x: {}, y: {}", self.x, self.y);

    let max = self.max_coordinate();
    let min = self.min_coordinate();

    let mut x = self.x.saturating_add(self.x_vel);

    if x > max {
      x = max;
      self.x_vel *= -1;
    } else if x < min {
      x = min;
      self.x_vel *= -1;
    }

    self.x = x;

    let mut y = self.y.saturating_add(self.y_vel);

    if y > max {
      y = max;
      self.y_vel *= -1;
    } else if y < min {
      y = min;
      self.y_vel *= -1;
    }

    self.y = y;
  }

  fn max_coordinate(&self) -> i16 {
    ETHERDREAM_X_MAX - self.radius
  }

  fn min_coordinate(&self) -> i16 {
    ETHERDREAM_X_MIN + self.radius
  }
}

fn main() {
  println!("Searching for DAC...");

  let mut dac = find_first_etherdream_dac().expect("Couldn't find DAC");

  let mut circle = Arc::new(RwLock::new(Circle::new(8000)));
  let mut circle2 = circle.clone();

  let mut pos = 0;

  thread::spawn(move || {
    loop {
      // FIXME: Locking critical section is bigger than it has to be.
      circle.write().unwrap().animate();
      thread::sleep_ms(10);
    }
  });

  dac.play_function(|num_points: u16| {
    circle2.read().unwrap().get_points(num_points, &mut pos)
  });
}
