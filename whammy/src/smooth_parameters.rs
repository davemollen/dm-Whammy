use crate::shared::ramp_smooth::RampSmooth;

const SPEED_RAMP_FREQ: f32 = 50.;
const GAIN_RAMP_TIME: f32 = 20.;

pub struct SmoothParameters {
  filters: [RampSmooth; 3],
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      filters: [
        RampSmooth::new(sample_rate, SPEED_RAMP_FREQ),
        RampSmooth::new(sample_rate, GAIN_RAMP_TIME),
        RampSmooth::new(sample_rate, GAIN_RAMP_TIME),
      ],
    }
  }

  pub fn initialize(&mut self, (speed, dry_level, wet_level): (f32, f32, f32)) {
    self.filters[0].initialize(speed);
    self.filters[1].initialize(dry_level);
    self.filters[2].initialize(wet_level);
  }

  pub fn process(&mut self, speed: f32, dry_level: f32, wet_level: f32) -> (f32, f32, f32) {
    (
      self.filters[0].process(speed),
      self.filters[1].process(dry_level),
      self.filters[2].process(wet_level),
    )
  }
}
