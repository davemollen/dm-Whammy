use crate::shared::ramp_smooth::RampSmooth;

const PITCH_RAMP_FREQ: f32 = 100.;
const GAIN_RAMP_TIME: f32 = 20.;

pub struct SmoothParameters {
  filters: [RampSmooth; 3],
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      filters: [
        RampSmooth::new(sample_rate, PITCH_RAMP_FREQ),
        RampSmooth::new(sample_rate, GAIN_RAMP_TIME),
        RampSmooth::new(sample_rate, GAIN_RAMP_TIME),
      ],
    }
  }

  pub fn process(&mut self, pitch: f32, dry_level: f32, wet_level: f32) -> (f32, f32, f32) {
    (
      self.filters[0].process(pitch),
      self.filters[1].process(dry_level),
      self.filters[2].process(wet_level),
    )
  }
}
