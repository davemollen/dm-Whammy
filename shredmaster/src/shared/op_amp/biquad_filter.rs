pub struct BiquadFilter {
  z: [f32; 2],
}

impl BiquadFilter {
  pub fn new() -> Self {
    Self { z: [0.0; 2] }
  }

  pub fn process(&mut self, x: f32, (b, a): ([f32; 3], [f32; 3])) -> f32 {
    let y = x * b[0] + self.z[0];
    self.z[0] = x * b[1] - y * a[1] + self.z[1];
    self.z[1] = x * b[2] - y * a[2];

    y
  }
}
