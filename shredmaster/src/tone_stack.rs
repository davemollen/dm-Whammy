mod bilinear_transform;
use bilinear_transform::BilinearTransform;
mod third_order_iir_filter;
use third_order_iir_filter::ThirdOrderIIRFilter;

pub struct ToneStack {
  filter: ThirdOrderIIRFilter,
  bilinear_transform: BilinearTransform,
}

impl ToneStack {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      filter: ThirdOrderIIRFilter::new(),
      bilinear_transform: BilinearTransform::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, bass: f32, treble: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients(bass, treble);
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.filter.process(input, z_domain_coefficients)
  }

  fn get_s_domain_coefficients(&self, bass: f32, treble: f32) -> ([f32; 4], [f32; 4]) {
    let r1 = 22000.;
    let r2 = 100000. * bass;
    let r3 = 1000.;
    let r4 = 6800.;

    let c1 = 2.2e-8;
    let c2 = 2.2e-7;
    let c3 = 2.2e-8;
    let c1c2 = c1 * c2;
    let c1c3 = c1 * c3;
    let c2c3 = c2 * c3;
    let c1c2c3 = c1c2 * c3;

    let r1_a = (1. - treble) * r1;
    let r1_b = treble * r1;

    let b0 = c1c2c3 * r2 * r3 * r4
      + r1_a * c1c2c3 * r2 * r4
      + r1_a * c1c2c3 * r2 * r3
      + c1c2c3 * r2 * r3 * r1_b;
    let b1 = c1c3 * r2 * r4
      + c1c2 * r2 * r4
      + c2c3 * r2 * r3
      + c1c3 * r2 * r3
      + c1c3 * r3 * r4
      + c1c2 * r3 * r4
      + r1_a * c1c3 * r4
      + r1_a * c1c2 * r2
      + r1_a * c1c2 * r4
      + r1_a * c1c3 * r3
      + r1_a * c1c2 * r3
      + c1c2 * r2 * r1_b
      + c1c3 * r3 * r1_b
      + c1c2 * r3 * r1_b;
    let b2 = c2 * r2 + c1 * r2 + c3 * r3 + c2 * r3 + c1 * r3 + r1_a * c1;
    let a0 = b0 + c1c2c3 * r2 * r4 * r1_b;
    let a1 = b1 + c2c3 * r2 * r4 + c1c3 * r4 * r1_b + c1c2 * r4 * r1_b;
    let a2 =
      c3 * r4 + c2 * r2 + c1 * r2 + c2 * r4 + c3 * r3 + c2 * r3 + c1 * r3 + r1_a * c1 + c1 * r1_b;

    ([b0, b1, b2, 0.], [a0, a1, a2, 1.])
  }
}

#[cfg(test)]
mod tests {
  use super::ToneStack;

  #[test]
  fn s_domain_coefficients_should_be_correct_for_contour_at_one() {
    let tone_stack = ToneStack::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [5.515663999999999e-10, 7.9519264e-06, 0.012606, 0.0],
      [
        9.498015999999999e-10,
        9.9957616e-06,
        0.014493599999999999,
        1.0,
      ],
    );
    assert_eq!(tone_stack.get_s_domain_coefficients(0.5, 0.5), coeffs)
  }
}
