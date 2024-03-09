pub struct BilinearTransform {
  s: [f32; 4],
}

impl BilinearTransform {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: [t / 2., t * t / 4., t * t * t / 8., t * t * t * t / 16.],
    }
  }

  pub fn process(&self, (mut b, mut a): ([f32; 5], [f32; 5])) -> ([f32; 5], [f32; 5]) {
    b[1] *= self.s[0];
    b[2] *= self.s[1];
    b[3] *= self.s[2];
    b[4] *= self.s[3];

    let b0 = b[0] + b[1] + b[2] + b[3] + b[4];
    let b1 = -4. * b[0] - 2. * b[1] + 2. * b[3] + 4. * b[4];
    let b2 = 6. * b[0] - 2. * b[2] + 6. * b[4];
    let b3 = -4. * b[0] + 2. * b[1] - 2. * b[3] + 4. * b[4];
    let b4 = b[0] - b[1] + b[2] - b[3] + b[4];

    a[1] *= self.s[0];
    a[2] *= self.s[1];
    a[3] *= self.s[2];
    a[4] *= self.s[3];

    let a0 = a[0] + a[1] + a[2] + a[3] + a[4];
    let a1 = -4. * a[0] - 2. * a[1] + 2. * a[3] + 4. * a[4];
    let a2 = 6. * a[0] - 2. * a[2] + 6. * a[4];
    let a3 = -4. * a[0] + 2. * a[1] - 2. * a[3] + 4. * a[4];
    let a4 = a[0] - a[1] + a[2] - a[3] + a[4];

    (
      [b0 / a0, b1 / a0, b2 / a0, b3 / a0, b4 / a0],
      [1., a1 / a0, a2 / a0, a3 / a0, a4 / a0],
    )
  }
}

#[cfg(test)]
mod tests {
  use super::BilinearTransform;

  #[test]
  fn bilinear_transform_should_be_correct() {
    let bilinear_transform = BilinearTransform::new(44100.);

    let coeffs: ([f32; 5], [f32; 5]) = (
      [
        0.,
        -7.984785820000001e-16,
        -1.39869708e-11,
        -3.2339999999999998e-09,
        0.,
      ],
      [
        7.992756832940001e-20,
        2.9987248821340003e-15,
        2.25952365284e-11,
        6.022145399999999e-09,
        1.4699999999999998e-07,
      ],
    );
    assert_eq!(
      bilinear_transform.process(coeffs),
      (
        [-0.09291117, 0.15488408, 0.03077689, -0.15488408, 0.06213427],
        [1., -3.31813449, 4.05474241, -2.15447971, 0.41787213]
      )
    );
  }
}
