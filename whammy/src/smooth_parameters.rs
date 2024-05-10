mod ramp_smooth;
use ramp_smooth::RampSmooth;

const PITCH_RAMP_TIME: f32 = 10.;
const GAIN_RAMP_TIME: f32 = 50.;

pub struct SmoothParameters {
  filters: [RampSmooth; 3],
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      filters: [RampSmooth::new(sample_rate); 3],
    }
  }

  pub fn process(&mut self, pitch: f32, dry_gain: f32, wet_gain: f32) -> (f32, f32, f32) {
    (
      self.filters[0].process(pitch, PITCH_RAMP_TIME),
      self.filters[1].process(dry_gain, GAIN_RAMP_TIME),
      self.filters[2].process(wet_gain, GAIN_RAMP_TIME),
    )
  }
}
