mod ramp_smooth;
use ramp_smooth::RampSmooth;
use std::convert::TryInto;

const RAMPTIME: f32 = 50.;

pub struct SmoothParameters<const T: usize> {
  filters: [RampSmooth; T],
}

impl<const T: usize> SmoothParameters<T> {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      filters: [RampSmooth::new(sample_rate); T],
    }
  }

  pub fn process(&mut self, params: [f32; T]) -> [f32; T] {
    self
      .filters
      .iter_mut()
      .zip(params)
      .map(|(filter, param)| filter.process(param, RAMPTIME))
      .collect::<Vec<f32>>()
      .try_into()
      .unwrap()
  }
}
