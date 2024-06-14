mod pitch_detector;
pub mod shared {
  pub mod delay_line;
  pub mod delta;
  pub mod float_ext;
  pub mod ramp_smooth;
}
mod grains;
mod smooth_parameters;
use {
  grains::Grains, pitch_detector::PitchDetector, shared::float_ext::FloatExt,
  smooth_parameters::SmoothParameters,
};

pub const MIN_PITCH: f32 = -24.;
pub const MAX_PITCH: f32 = 24.;

pub struct Whammy {
  pitch_detector: PitchDetector,
  smooth_parameters: SmoothParameters,
  grains: Grains,
}

impl Whammy {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      pitch_detector: PitchDetector::new(sample_rate),
      smooth_parameters: SmoothParameters::new(sample_rate),
      grains: Grains::new(sample_rate),
    }
  }

  pub fn get_params(&self, pitch: f32, dry: f32, wet: f32) -> (f32, f32, f32) {
    (
      1. - 2_f32.powf(pitch / 12.),
      Self::dbtoa(dry),
      Self::dbtoa(wet),
    )
  }

  pub fn initialize_params(&mut self, params: (f32, f32, f32)) {
    self.smooth_parameters.initialize(params);
    self.grains.initialize(params.0);
  }

  pub fn process(&mut self, input: f32, speed: f32, dry_level: f32, wet_level: f32) -> f32 {
    let (speed, dry_gain, wet_gain) = self.smooth_parameters.process(speed, dry_level, wet_level);
    let freq = self.pitch_detector.get_frequency(input);
    let grains_out = self.grains.process(input, speed, freq);

    input * dry_gain + grains_out * wet_gain
  }

  fn dbtoa(level: f32) -> f32 {
    if level <= -70. {
      0.
    } else {
      level.dbtoa()
    }
  }
}
