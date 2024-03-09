pub struct BilinearTransform {
  s: [f32; 2],
}

impl BilinearTransform {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: [t / 2., t * t / 4.],
    }
  }

  /// First tuple element represents b1, because b0 & b2 are expected to equal zero.
  pub fn process(&self, (mut b1, mut a): (f32, [f32; 3])) -> ([f32; 3], [f32; 3]) {
    b1 *= self.s[0];
    a[1] *= self.s[0];
    a[2] *= self.s[1];

    let a0 = a[0] + a[1] + a[2];
    let a1 = -2. * a[0] + 2. * a[2];
    let a2 = a[0] - a[1] + a[2];

    ([b1 / a0, 0., -b1 / a0], [1., a1 / a0, a2 / a0])
  }
}

#[cfg(test)]
mod tests {
  use super::BilinearTransform;

  #[test]
  fn bilinear_transform_should_be_correct() {
    let bilinear_transform = BilinearTransform::new(44100.);

    let coeffs: (f32, [f32; 3]) = (2594706.7981318, [1., 33082.511676181, 56113901.343681]);
    assert_eq!(
      bilinear_transform.process(coeffs),
      (
        [21.28226674, 0., -21.28226674],
        [1., -1.43642888, 0.4573022],
      )
    );
  }
}
