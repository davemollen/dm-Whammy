
pub struct FourthOrderIIRFilter {
  z: [f32; 4],
}

impl FourthOrderIIRFilter {
  pub fn new() -> Self {
    Self { z: [0.0; 4] }
  }

  pub fn process(&mut self, x: f32, (b, a): ([f32; 5], [f32; 5])) -> f32 {
    let y = x * b[0] + self.z[0];
    self.z[0] = x * b[1] - y * a[1] + self.z[1];
    self.z[1] = x * b[2] - y * a[2] + self.z[2];
    self.z[2] = x * b[3] - y * a[3] + self.z[3];
    self.z[3] = x * b[4] - y * a[4];

    y
  }
}
