pub struct ThirdOrderIIRFilter {
  z: [f32; 3],
}

impl ThirdOrderIIRFilter {
  pub fn new() -> Self {
    Self { z: [0.0; 3] }
  }

  pub fn process(&mut self, x: f32, (b, a): ([f32; 4], [f32; 4])) -> f32 {
    let y = x * b[0] + self.z[0];
    self.z[0] = x * b[1] - y * a[1] + self.z[1];
    self.z[1] = x * b[2] - y * a[2] + self.z[2];
    self.z[2] = x * b[3] - y * a[3];

    y
  }
}
