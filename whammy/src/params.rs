mod smooth;
use smooth::LinearSmooth;
pub use {crate::shared::float_ext::FloatExt, smooth::Smoother};

pub struct Params {
  pub speed: LinearSmooth,
  pub dry: LinearSmooth,
  pub wet: LinearSmooth,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      speed: LinearSmooth::new(sample_rate, 50.),
      dry: LinearSmooth::new(sample_rate, 20.),
      wet: LinearSmooth::new(sample_rate, 20.),
      is_initialized: false,
    }
  }

  pub fn set(&mut self, pitch: f32, dry: f32, wet: f32) {
    let speed = 1. - 2_f32.powf(pitch / 12.);
    let dry = Self::dbtoa(dry);
    let wet = Self::dbtoa(wet);

    if self.is_initialized {
      self.speed.set_target(speed);
      self.dry.set_target(dry);
      self.wet.set_target(wet);
    } else {
      self.speed.reset(speed);
      self.dry.reset(dry);
      self.wet.reset(wet);
      self.is_initialized = true;
    }
  }

  fn dbtoa(level: f32) -> f32 {
    if level <= -70. {
      0.
    } else {
      level.dbtoa()
    }
  }
}
