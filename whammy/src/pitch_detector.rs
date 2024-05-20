mod one_pole_filter;
use one_pole_filter::OnePoleFilter;

use crate::shared::delta::Delta;

pub const MIN_FREQ: f32 = 50.;
const MAX_FREQ: f32 = 1500.;

// TODO: replace this with a better pitch detector
pub struct PitchDetector {
  sample_rate: f32,
  filter: OnePoleFilter,
  delta: Delta,
  counter: f32,
  frequency: f32,
}

impl PitchDetector {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      filter: OnePoleFilter::new(sample_rate, 20.),
      delta: Delta::new(),
      counter: 0.,
      frequency: 0.,
    }
  }

  pub fn get_frequency(&mut self, input: f32) -> f32 {
    self.counter += 1.;

    let filtered = self.filter.process(input);
    let zero_cross = self.delta.process(if filtered > 0. { 1. } else { 0. }) > 0.;
    if zero_cross {
      let frequency = self.sample_rate * self.counter.recip();
      if frequency > MIN_FREQ && frequency < MAX_FREQ {
        self.frequency = frequency;
      }
      self.counter = 0.;
    }

    self.frequency
  }
}
