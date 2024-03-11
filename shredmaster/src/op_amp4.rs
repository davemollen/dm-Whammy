use crate::shared::op_amp::OpAmp;

// Maybe this one can be replaced with a one-pole filter, because the highpass frequency is at 7.23Hz
pub struct OpAmp4 {
  op_amp: OpAmp,
}

impl OpAmp4 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: OpAmp::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let s_domain_coefficients = (-10000., [1., 10045.454545455, 454545.45454545]);
    self.op_amp.process(input, s_domain_coefficients)
  }
}
