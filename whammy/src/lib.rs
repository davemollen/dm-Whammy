mod pitch_detector;
pub mod shared {
  pub mod delay_line;
  pub mod delta;
  pub mod float_ext;
}
mod grains;
mod params;
pub use params::Params;
use {grains::Grains, params::Smoother, pitch_detector::PitchDetector};

pub const MIN_PITCH: f32 = -24.;
pub const MAX_PITCH: f32 = 24.;

pub struct Whammy {
  pitch_detector: PitchDetector,
  grains: Grains,
}

impl Whammy {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      pitch_detector: PitchDetector::new(sample_rate),
      grains: Grains::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> f32 {
    let speed = params.speed.next();
    let dry = params.dry.next();
    let wet = params.wet.next();

    let freq = self.pitch_detector.get_frequency(input);
    let grains_out = self.grains.process(input, speed, freq);

    input * dry + grains_out * wet
  }
}
