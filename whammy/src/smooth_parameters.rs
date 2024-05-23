use crate::shared::ramp_smooth::RampSmooth;

pub struct SmoothParameters {
  smooth_speed: RampSmooth,
  smooth_dry_level: RampSmooth,
  smooth_wet_level: RampSmooth,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_speed: RampSmooth::new(sample_rate, 50.),
      smooth_dry_level: RampSmooth::new(sample_rate, 20.),
      smooth_wet_level: RampSmooth::new(sample_rate, 20.),
    }
  }

  pub fn initialize(&mut self, (speed, dry_level, wet_level): (f32, f32, f32)) {
    self.smooth_speed.initialize(speed);
    self.smooth_dry_level.initialize(dry_level);
    self.smooth_wet_level.initialize(wet_level);
  }

  pub fn process(&mut self, speed: f32, dry_level: f32, wet_level: f32) -> (f32, f32, f32) {
    (
      self.smooth_speed.process(speed),
      self.smooth_dry_level.process(dry_level),
      self.smooth_wet_level.process(wet_level),
    )
  }
}
