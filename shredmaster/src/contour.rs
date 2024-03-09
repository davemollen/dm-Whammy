mod bilinear_transform;
use bilinear_transform::BilinearTransform;
mod fourth_order_iir_filter;
use fourth_order_iir_filter::FourthOrderIIRFilter;

pub struct Contour {
  filter: FourthOrderIIRFilter,
  bilinear_transform: BilinearTransform,
}

impl Contour {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      bilinear_transform: BilinearTransform::new(sample_rate),
      filter: FourthOrderIIRFilter::new(),
    }
  }

  pub fn process(&mut self, input: f32, contour: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients(contour);
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.filter.process(input, z_domain_coefficients)
  }

  /// First tuple element returns b1, b2 & b3. It skips b0 & b4 because these equal zero.
  fn get_s_domain_coefficients(&self, contour: f32) -> ([f32; 5], [f32; 5]) {
    let r1 = 100.;
    let r2 = 33000.;
    let r3 = 33000.;
    let r4 = 100000.;
    let r5 = contour.min(0.98) * 100000.;
    let r6 = 100000.;

    let c1 = 1e-9;
    let c2 = 2.2e-7;
    let c3 = 1e-7;
    let c4 = 4.7e-8;
    let c5 = 1e-9;

    let a4 = c3 + c4;
    let var_a = r1 + r5; // assert_eq!(var_a, 200.);
    let var_b = (var_a + r2 + r1) * c3 * c4; // assert_eq!(var_b, 1.5651e-10);
    let var_c = r1 + r2;
    let var_d = var_c * var_a * c4 * c3 + r3 * var_b; // assert_eq!(var_d, 5.19594e-6);
    let var_e = var_c * a4 + r3 * a4; // assert_eq!(var_e, 0.0097167);
    let var_f = c1 * r1 * var_b + c1 * var_d; // assert_eq!(var_f, 5.21159e-15);
    let var_g = c3 * a4 + var_e * c1 + var_b; // assert_eq!(var_g, 1.66241e-10);
    let var_h = c2 * r4;
    let var_i = var_f * var_h + var_d * c3 * c2; // assert_eq!(var_i, 1.14769e-16);
    let var_j = var_g * var_h + (var_e * c3 + var_d) * c2 + var_f; // assert_eq!(var_j, 4.80583e-12);
    let var_k = a4 * var_h + var_e * c2 + var_g; // assert_eq!(var_k, 5.53792E-9);
    let var_l = r6 * c5;

    let a0 = var_i * var_l;
    let a1 = var_j * var_l + var_i;
    let a2 = var_k * var_l + var_j;
    let a3 = a4 * var_l + var_k;

    let b1 = -var_f * var_h;
    let b2 = -var_g * var_h;
    let b3 = -a4 * var_h;

    ([0., b1, b2, b3, 0.], [a0, a1, a2, a3, a4])
  }
}

#[cfg(test)]
mod tests {
  use super::Contour;

  #[test]
  fn s_domain_coefficients_should_be_correct_for_contour_at_one() {
    let contour = Contour::new(44100.);

    let coeffs: ([f32; 5], [f32; 5]) = (
      [0., -7.98479E-16, -1.3987E-11, -3.234E-9, 0.],
      [
        7.99276E-20,
        2.99873E-15,
        2.25952E-11,
        6.02215E-9,
        0.000000147,
      ],
    );
    assert_eq!(contour.get_s_domain_coefficients(1.), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_contour_at_one_hundredth() {
    let contour = Contour::new(44100.);

    let coeffs: ([f32; 5], [f32; 5]) = (
      [0., -1.20816E-16, -3.75036E-12, -3.234E-9, 0.],
      [
        1.20936E-20,
        6.17006E-16,
        5.51492E-12,
        5.55685E-9,
        0.000000147,
      ],
    );
    assert_eq!(contour.get_s_domain_coefficients(0.01), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_contour_at_one_thousandth() {
    let contour = Contour::new(44100.);

    let coeffs: ([f32; 5], [f32; 5]) = (
      [0., -1.14655E-16, -3.6573E-12, -3.234E-9, 0.],
      [
        1.14769E-20,
        5.95353E-16,
        5.35963E-12,
        5.55262E-9,
        0.000000147,
      ],
    );
    assert_eq!(contour.get_s_domain_coefficients(0.001), coeffs)
  }
}
