mod pitch_detector;
use pitch_detector::PitchDetector;
pub mod shared {
  pub mod delay_line;
  pub mod float_ext;
}
mod grains;
use grains::Grains;

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

  pub fn process(&mut self, input: f32, pitch: f32) -> f32 {
    let freq = self.pitch_detector.get_frequency(input);
    let trigger = self.pitch_detector.get_trigger();
    self.grains.process(input, pitch, freq, trigger)
  }
}
