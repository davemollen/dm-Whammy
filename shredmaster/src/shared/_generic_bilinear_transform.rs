use std::array;

pub trait BilinearTransformer {
  fn process(coefficients: Self) -> Self;
}

impl BilinearTransformer for [f32; 3] {
  fn process(x: [f32; 3]) -> [f32; 3] {
    [
      x[0] + x[1] + x[2],
      -2. * x[0] + 2. * x[2],
      x[0] - x[1] + x[2],
    ]
  }
}

impl BilinearTransformer for [f32; 4] {
  fn process(x: [f32; 4]) -> [f32; 4] {
    [
      x[0] + x[1] + x[2] + x[3],
      -3. * x[0] - x[1] + x[2] + 3. * x[3],
      3. * x[0] - x[1] - x[2] + 3. * x[3],
      -x[0] + x[1] - x[2] + x[3],
    ]
  }
}

impl BilinearTransformer for [f32; 5] {
  fn process(x: [f32; 5]) -> [f32; 5] {
    [
      x[0] + x[1] + x[2] + x[3] + x[4],
      -4. * x[0] - 2. * x[1] + 2. * x[3] + 4. * x[4],
      6. * x[0] - 2. * x[2] + 6. * x[4],
      -4. * x[0] + 2. * x[1] - 2. * x[3] + 4. * x[4],
      x[0] - x[1] + x[2] - x[3] + x[4],
    ]
  }
}

pub struct BilinearTransform<const T: usize> {
  s: [f32; T],
}

impl<const T: usize> BilinearTransform<T>
where
  [f32; T]: BilinearTransformer,
{
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: array::from_fn(|i| (i + 1) as f32).map(|i| t.powf(i) / 2_f32.powf(i)),
    }
  }

  pub fn process(&self, (b, a): ([f32; T], [f32; T])) -> ([f32; T], [f32; T]) {
    let b = BilinearTransformer::process(self.apply_power_to_and_sr(b));
    let a = BilinearTransformer::process(self.apply_power_to_and_sr(a));
    (b.map(|x| x / a[0]), a.map(|x| x / a[0]))
  }

  fn apply_power_to_and_sr(&self, x: [f32; T]) -> [f32; T] {
    let mut result = [0.; T];
    for i in 0..x.len() {
      result[i] = x[i] * self.s[i];
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::BilinearTransform;

  #[test]
  fn third_order_bilinear_transform_should_be_correct() {
    let bilinear_transform = BilinearTransform::new(44100.);

    let coeffs: ([f32; 3], [f32; 3]) = (
      [0., 2594706.7981318, 0.],
      [1., 33082.511676181, 56113901.343681],
    );
    assert_eq!(
      bilinear_transform.process(coeffs),
      (
        [21.28226674, 0., -21.28226674],
        [1., -1.43642888, 0.4573022],
      )
    );
  }

  #[test]
  fn fourth_order_bilinear_transform_should_be_correct() {
    let bilinear_transform = BilinearTransform::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [1., 2.27816380e+08, 2.54546370e+11, 3.67445774e+11],
      [1., 1.10051112e+05, 1.00878563e+09, 3.67445774e+11],
    );
    assert_eq!(
      bilinear_transform.process(coeffs),
      (
        [1100.38731292, -1073.70700939, -1098.70429603, 1072.02579416],
        [1., -1.73109163, 0.6830169, 0.0498764],
      )
    );
  }

  #[test]
  fn fifth_order_bilinear_transform_should_be_correct() {
    let bilinear_transform = BilinearTransform::new(44100.);

    let coeffs: ([f32; 5], [f32; 5]) = (
      [0., -1.13977E-16, -3.64707E-12, -3.234E-9, 0.],
      [
        1.14091E-20,
        5.92972E-16,
        5.34256E-12,
        5.55215E-9,
        0.000000147,
      ],
    );
    assert_eq!(
      bilinear_transform.process(coeffs),
      (
        [-0.09379031, 0.13677618, 0.04980302, -0.13677618, 0.04398728],
        [1., -3.13731712, 3.56302416, -1.71065436, 0.28494939],
      )
    );
  }
}
