use crate::shared::op_amp::OpAmp;

pub struct OpAmp1 {
  op_amp: OpAmp,
}

impl OpAmp1 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: OpAmp::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, distortion: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients(distortion);
    self.op_amp.process(input, s_domain_coefficients)
  }

  fn get_s_domain_coefficients(&self, gain: f32) -> (f32, [f32; 3]) {
    let r1 = 3300.;
    let c1 = 4.7e-8;
    let r2 = gain * 100000.;
    let c2 = 1e-10;

    let b1 = r2 * c1;
    let z2_a = r1 * c1;
    let z2_b = c2 * r2;

    let a0 = z2_a * z2_b;
    let a1 = z2_a + z2_b;

    // TODO: check if normalizing is needed, because it will be normalized at a later stage too
    (-b1 / a0, [1., a1 / a0, 1. / a0])
  }
}

#[cfg(test)]
mod tests {
  use super::OpAmp1;

  #[test]
  fn s_domain_coefficients_should_be_correct_for_gain_at_one() {
    let op_amp_with_gain = OpAmp1::new(44100.);

    assert_eq!(
      op_amp_with_gain.get_s_domain_coefficients(1.),
      (-3030303.030303, [1., 106447.45325596, 644745325.59639])
    );
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_gain_at_one_tenth() {
    let op_amp_with_gain = OpAmp1::new(44100.);

    assert_eq!(
      op_amp_with_gain.get_s_domain_coefficients(0.1),
      (3030303.030303, [1., 1006447.453256, 6447453255.9639])
    );
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_gain_at_one_hundredth() {
    let op_amp_with_gain = OpAmp1::new(44100.);

    assert_eq!(
      op_amp_with_gain.get_s_domain_coefficients(0.01),
      (-3030303.030303, [1., 10006447.453256, 64474532559.639])
    );
  }
}
