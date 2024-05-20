mod pitch_detector;
use pitch_detector::PitchDetector;
pub mod shared {
  pub mod delay_line;
  pub mod delta;
  pub mod float_ext;
  pub mod ramp_smooth;
}
mod grains;
use grains::Grains;
mod smooth_parameters;
pub use shared::float_ext::FloatExt;
use smooth_parameters::SmoothParameters;

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

  pub fn process(&mut self, input: f32, speed: f32, dry_level: f32, wet_level: f32) -> f32 {
    let (speed, dry_gain, wet_gain) = self.smooth_parameters.process(speed, dry_level, wet_level);
    let freq = self.pitch_detector.get_frequency(input);
    let grains_out = self.grains.process(input, speed, freq);

    input * dry_gain + grains_out * wet_gain
  }
}
