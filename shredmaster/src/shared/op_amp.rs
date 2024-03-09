mod biquad_filter;
use biquad_filter::BiquadFilter;
mod bilinear_transform;
use bilinear_transform::BilinearTransform;

pub struct OpAmp {
  op_amp: BiquadFilter,
  bilinear_transform: BilinearTransform,
}

impl OpAmp {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: BiquadFilter::new(),
      bilinear_transform: BilinearTransform::new(sample_rate),
    }
  }

  /// First tuple element represents b1, because b0 & b2 are expected to equal zero.
  pub fn process(&mut self, input: f32, s_domain_coefficients: (f32, [f32; 3])) -> f32 {
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.op_amp.process(input, z_domain_coefficients)
  }
}
