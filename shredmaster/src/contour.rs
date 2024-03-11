use crate::shared::{
  bilinear_transform::BilinearTransform, third_order_iir_filter::ThirdOrderIIRFilter,
};

pub struct Contour {
  filter: ThirdOrderIIRFilter,
  bilinear_transform: BilinearTransform,
}

impl Contour {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      bilinear_transform: BilinearTransform::new(sample_rate),
      filter: ThirdOrderIIRFilter::new(),
    }
  }

  pub fn process(&mut self, input: f32, contour: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients(contour);
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.filter.process(input, z_domain_coefficients)
  }

  fn get_s_domain_coefficients(&self, contour: f32) -> ([f32; 4], [f32; 4]) {
    let r1 = 100.;
    let r2 = 33000.;
    let r3 = 33000.;
    let r4 = 100000.;
    let r4_a = contour * r4;
    let r4_b = (1. - contour) * r4;
    let c1 = 1e-9;
    let c2 = 1e-7;
    let c3 = 4.7e-8;

    let c1c2 = c1 * c2;
    let c1c3 = c1 * c3;
    let c2c3 = c2 * c3;
    let c1c3r1 = c1c3 * r1;
    let c1c2c3 = c1c2 * c3;
    let c1c3r2 = c1c3 * r2;
    let c2c3r1 = c2c3 * r1;
    let c1c2r1 = c1c2 * r1;
    let c1c2c3r1 = c1c2c3 * r1;
    let c1c2c3r1r3 = c1c2c3r1 * r3;
    let c1c2c3r1r2 = c1c2c3r1 * r2;
    let r3r4_a = r3 * r4_a;
    let r2r4_a = r2 * r4_a;

    let b0 = r4_b * c1c2c3 * r3 * r4_a + r4_b * c1c2c3 * r2r4_a + c1c2c3 * r2r4_a * r3;
    let b1 = r4_b * c2c3 * r4_a
      + r4_b * c1c3 * r3
      + c1c2 * r3r4_a
      + r4_b * c1c3r2
      + c1c2 * r2r4_a
      + c1c3r2 * r3;
    let b2 = r4_b * c3 + c2 * r4_a + c1 * r3 + c1 * r2;

    let a0 = b0
      + r4_b * c1c2c3r1r3
      + c1c2c3r1 * r3r4_a
      + r4_b * c1c2c3r1r2
      + c1c2c3r1r2 * r4_a
      + c1c2c3r1r2 * r3;
    let a1 = b1
      + c2c3 * r2r4_a
      + r4_b * c2c3r1
      + c2c3r1 * r4_a
      + c1c2r1 * r3
      + c1c3r1 * r3
      + c1c2r1 * r2
      + c1c3r1 * r2
      + c2c3r1 * r2;
    let a2 = b2 + c3 * r2 + c2 * r1 + c3 * r1;

    ([b0, b1, b2, 1.], [a0, a1, a2, 1.])
  }
}

#[cfg(test)]
mod tests {
  use super::Contour;

  #[test]
  fn s_domain_coefficients_should_be_correct_for_contour_at_zero() {
    let contour = Contour::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [0.0, 3.61383e-07, 0.004765999999999999, 1.],
      [
        3.61383e-12,
        4.2486319999999997e-07,
        0.006331699999999999,
        1.0,
      ],
    );
    assert_eq!(contour.get_s_domain_coefficients(1.), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_contour_at_a_half() {
    let contour = Contour::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [1.031415e-09, 1.2286283000000001e-05, 0.007416, 1.],
      [
        1.0350288300000003e-09,
        2.01047632e-05,
        0.008981699999999999,
        1.,
      ],
    );
    assert_eq!(contour.get_s_domain_coefficients(0.5), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_contour_at_one() {
    let contour = Contour::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [5.1183e-10, 7.11183e-07, 0.010066, 1.],
      [5.154438299999999e-10, 1.62846632e-05, 0.0116317, 1.],
    );
    assert_eq!(contour.get_s_domain_coefficients(0.001), coeffs)
  }
}
