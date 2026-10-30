#[derive(Clone)]
pub struct Delta {
  z: f32,
}

impl Delta {
  pub fn new(value: f32) -> Self {
    Self { z: value }
  }

  pub fn reset(&mut self, value: f32) {
    self.z = value;
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let output = input - self.z;
    self.z = input;
    output
  }
}
