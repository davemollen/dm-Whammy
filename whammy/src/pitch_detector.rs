mod one_pole_filter;
use one_pole_filter::{Mode, OnePoleFilter};

use crate::shared::delta::Delta;

// TODO: improve the stability of this simple pitch detection algorithm
pub struct PitchDetector {
  sample_rate: f32,
  filter: OnePoleFilter,
  delta: Delta,
  counter: f32,
  frequency: Option<f32>,
}

impl PitchDetector {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      filter: OnePoleFilter::new(sample_rate),
      delta: Delta::new(),
      counter: 0.,
      frequency: None,
    }
  }

  pub fn get_frequency(&mut self, input: f32) -> Option<f32> {
    self.counter += 1.;

    let filtered = self.filter.process(input, 400., Mode::Hertz);
    let zero_cross = self.delta.process(if filtered > 0. { 1. } else { 0. }) > 0.;
    if zero_cross {
      let frequency = self.sample_rate / self.counter;
      if frequency > 30. && frequency < 3000. {
        self.frequency = Some(frequency);
      }
      self.counter = 0.;
    }

    self.frequency
  }
}
