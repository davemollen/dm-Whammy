pub struct BilinearTransform {
  s: [f32; 3],
}

impl BilinearTransform {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: [t / 2., t * t / 4., t * t * t / 8.],
    }
  }

  pub fn process(&self, (mut b, mut a): ([f32; 4], [f32; 4])) -> ([f32; 4], [f32; 4]) {
    b[1] *= self.s[0];
    b[2] *= self.s[1];
    b[3] *= self.s[2];

    let b0 = b[0] + b[1] + b[2] + b[3];
    let b1 = -3. * b[0] - b[1] + b[2] + 3. * b[3];
    let b2 = 3. * b[0] - b[1] - b[2] + 3. * b[3];
    let b3 = -b[0] + b[1] - b[2] + b[3];

    a[1] *= self.s[0];
    a[2] *= self.s[1];
    a[3] *= self.s[2];

    let a0 = a[0] + a[1] + a[2] + a[3];
    let a1 = -3. * a[0] - a[1] + a[2] + 3. * a[3];
    let a2 = 3. * a[0] - a[1] - a[2] + 3. * a[3];
    let a3 = -a[0] + a[1] - a[2] + a[3];

    (
      [b0 / a0, b1 / a0, b2 / a0, b3 / a0],
      [1., a1 / a0, a2 / a0, a3 / a0],
    )
  }
}

#[cfg(test)]
mod tests {
  use super::BilinearTransform;

  #[test]
  fn bilinear_transform_should_be_correct() {
    let bilinear_transform = BilinearTransform::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [
        5.515663999999999e-11,
        1.0102048e-06,
        0.0025079999999999994,
        0.0,
      ],
      [
        9.498015999999998e-11,
        1.2471711999999998e-06,
        0.0030491999999999993,
        1.0,
      ],
    );
    assert_eq!(
      bilinear_transform.process(coeffs),
      (
        [0.61117939, -1.61259145, 1.40342029, -0.40200823],
        [1., -2.72736657, 2.46920943, -0.74173639],
      )
    );
  }
}
