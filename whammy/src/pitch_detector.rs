mod one_pole_filter;
use one_pole_filter::{Mode, OnePoleFilter};

use crate::shared::delta::Delta;

pub const MIN_FREQ: f32 = 50.;
const MAX_FREQ: f32 = 1500.;

// TODO: replace this with a better pitch detector
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

    let filtered = self.filter.process(input, 20., Mode::Hertz);
    let zero_cross = self.delta.process(if filtered > 0. { 1. } else { 0. }) > 0.;
    if zero_cross {
      let frequency = self.sample_rate / self.counter;
      if frequency > MIN_FREQ && frequency < MAX_FREQ {
        self.frequency = Some(frequency);
      }
      self.counter = 0.;
    }

    self.frequency
  }
}
