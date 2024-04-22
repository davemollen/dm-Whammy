use std::f32::consts::PI;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Interpolation {
  Step,
  Linear,
  Cosine,
  Cubic,
  Spline,
}

#[derive(Clone)]
pub struct DelayLine {
  buffer: Vec<f32>,
  write_pointer: usize,
  sample_rate: f32,
}

impl DelayLine {
  pub fn new(length: usize, sample_rate: f32) -> Self {
    Self {
      buffer: vec![0.0; length],
      write_pointer: 0,
      sample_rate,
    }
  }

  pub fn read(&mut self, time: f32, interp: Interpolation) -> f32 {
    let read_pointer = (self.write_pointer + self.buffer.len() - 1) as f32 - self.mstosamps(time);
    let rounded_read_pointer = read_pointer.trunc();
    let mix = read_pointer - rounded_read_pointer;
    let index = rounded_read_pointer as usize;

    match interp {
      Interpolation::Step => self.step_interp(index),
      Interpolation::Linear => self.linear_interp(index, mix),
      Interpolation::Cosine => self.cosine_interp(index, mix),
      Interpolation::Cubic => self.cubic_interp(index, mix),
      Interpolation::Spline => self.spline_interp(index, mix),
    }
  }

  pub fn write(&mut self, value: f32) {
    self.buffer[self.write_pointer] = value;
    self.write_pointer = self.wrap(self.write_pointer + 1);
  }

  fn mstosamps(&self, time: f32) -> f32 {
    time * 0.001 * self.sample_rate
  }

  fn wrap(&self, index: usize) -> usize {
    let buffer_len = self.buffer.len();
    if index >= buffer_len {
      index - buffer_len
    } else {
      index
    }
  }

  fn step_interp(&self, index: usize) -> f32 {
    self.buffer[self.wrap(index)]
  }

  fn linear_interp(&self, index: usize, mix: f32) -> f32 {
    let x = self.buffer[self.wrap(index)];
    let y = self.buffer[self.wrap(index + 1)];
    x * (1. - mix) + y * mix
  }

  fn cosine_interp(&self, index: usize, mix: f32) -> f32 {
    let cosine_mix = (1. - (mix * PI).cos()) / 2.;
    let x = self.buffer[self.wrap(index)];
    let y = self.buffer[self.wrap(index + 1)];
    x * (1. - cosine_mix) + y * cosine_mix
  }

  fn cubic_interp(&self, index: usize, mix: f32) -> f32 {
    let w = self.buffer[self.wrap(index.checked_sub(1).unwrap_or(index + self.buffer.len() - 1))];
    let x = self.buffer[self.wrap(index)];
    let y = self.buffer[self.wrap(index + 1)];
    let z = self.buffer[self.wrap(index + 2)];

    let a1 = 1. + mix;
    let aa = mix * a1;
    let b = 1. - mix;
    let b1 = 2. - mix;
    let bb = b * b1;
    let fw = -0.1666667 * bb * mix;
    let fx = 0.5 * bb * a1;
    let fy = 0.5 * aa * b1;
    let fz = -0.1666667 * aa * b;
    w * fw + x * fx + y * fy + z * fz
  }

  fn spline_interp(&self, index: usize, mix: f32) -> f32 {
    let w = self.buffer[self.wrap(index.checked_sub(1).unwrap_or(index + self.buffer.len() - 1))];
    let x = self.buffer[self.wrap(index)];
    let y = self.buffer[self.wrap(index + 1)];
    let z = self.buffer[self.wrap(index + 2)];

    let c0 = x;
    let c1 = (0.5) * (y - w);
    let c2 = w - (2.5) * x + y + y - (0.5) * z;
    let c3 = (0.5) * (z - w) + (1.5) * (x - y);
    ((c3 * mix + c2) * mix + c1) * mix + c0
  }
}

#[cfg(test)]
mod tests {
  use super::{DelayLine, Interpolation};

  #[test]
  fn step_interp() {
    let mut delay_line = DelayLine::new(2, 1.);
    delay_line.buffer = vec![1., 0.];

    assert_eq!(delay_line.step_interp(0), 1.);
    assert_eq!(delay_line.step_interp(1), 0.);
    assert_eq!(delay_line.step_interp(2), 1.);
    assert_eq!(delay_line.step_interp(3), 0.);
  }

  #[test]
  fn linear_interp() {
    let mut delay_line = DelayLine::new(2, 1.);
    delay_line.buffer = vec![1., 0.];

    assert_eq!(delay_line.linear_interp(0, 0.), 1.);
    assert_eq!(delay_line.linear_interp(0, 0.5), 0.5);
    assert_eq!(delay_line.linear_interp(0, 1.), 0.);
    assert_eq!(delay_line.linear_interp(1, 0.), 0.);
    assert_eq!(delay_line.linear_interp(1, 0.5), 0.5);
    assert_eq!(delay_line.linear_interp(1, 1.), 1.);
  }

  #[test]
  fn cubic_interp() {
    let mut delay_line = DelayLine::new(4, 1.);
    delay_line.buffer = vec![1., 0.75, 0.5, 0.25];

    assert_eq!(delay_line.cubic_interp(0, 0.), 1.);
    assert_eq!(delay_line.cubic_interp(0, 0.5), 0.9375);
    assert_eq!(delay_line.cubic_interp(0, 1.), 0.75);
    assert_eq!(delay_line.cubic_interp(2, 0.), 0.5);
    assert_eq!(delay_line.cubic_interp(2, 1.), 0.25);
    assert_eq!(delay_line.cubic_interp(3, 0.), 0.25);
    assert_eq!(delay_line.cubic_interp(3, 1.), 1.);
    assert_eq!(delay_line.cubic_interp(4, 0.), 1.);
    assert_eq!(delay_line.cubic_interp(4, 1.), 0.75);
  }

  #[test]
  fn read() {
    let mut delay_line = DelayLine::new(4, 1000.);
    delay_line.write(0.1);
    delay_line.write(0.2);
    delay_line.write(0.3);
    delay_line.write(0.4);

    assert_eq!(delay_line.read(0., Interpolation::Linear), 0.4);
    assert_eq!(delay_line.read(1., Interpolation::Linear), 0.3);
    assert_eq!(delay_line.read(2., Interpolation::Linear), 0.2);
    assert_eq!(delay_line.read(3., Interpolation::Linear), 0.1);
  }

  #[test]
  fn cubic_read() {
    let mut delay_line = DelayLine::new(8, 1000.);
    delay_line.write(0.1);
    delay_line.write(0.2);
    delay_line.write(0.3);
    delay_line.write(0.4);
    delay_line.write(0.5);
    delay_line.write(0.6);
    delay_line.write(0.7);
    delay_line.write(0.8);

    assert_eq!(delay_line.read(0., Interpolation::Cubic), 0.8);
    assert_eq!(delay_line.read(1., Interpolation::Cubic), 0.7);
    assert_eq!(delay_line.read(4., Interpolation::Cubic), 0.4);
    assert_eq!(delay_line.read(4.5, Interpolation::Cubic), 0.35);
    assert_eq!(delay_line.read(5.0, Interpolation::Cubic), 0.3);
  }
}
