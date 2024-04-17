mod pitch_detector;
use pitch_detector::PitchDetector;
pub mod shared {
  pub mod delay_line;
  pub mod delta;
  pub mod float_ext;
}
mod grains;
use grains::Grains;
mod ramp_smooth;
use ramp_smooth::RampSmooth;

pub const MIN_PITCH: f32 = -24.;
pub const MAX_PITCH: f32 = 24.;

pub struct Whammy {
  pitch_detector: PitchDetector,
  smooth_pitch: RampSmooth,
  grains: Grains,
}

impl Whammy {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      pitch_detector: PitchDetector::new(sample_rate),
      smooth_pitch: RampSmooth::new(sample_rate),
      grains: Grains::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, pitch: f32) -> f32 {
    let smooth_pitch = self.smooth_pitch.process(pitch, 50.);
    let freq = self.pitch_detector.get_frequency(input);
    self.grains.process(input, smooth_pitch, freq)
  }
}
