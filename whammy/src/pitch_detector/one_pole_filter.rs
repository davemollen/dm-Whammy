use std::f32::consts::TAU;

pub struct OnePoleFilter {
  b1: f32,
  z: f32,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f32, cutoff_freq: f32) -> Self {
    Self {
      b1: (-TAU * cutoff_freq * sample_rate.recip()).exp(),
      z: 0.,
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let a0 = 1.0 - self.b1;
    self.z = input * a0 + self.z * self.b1;
    self.z
  }
}
