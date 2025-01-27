mod grain;
mod phasor;
use {grain::Grain, phasor::Phasor};

use crate::shared::delay_line::DelayLine;

// VOICES needs to be a power of 2
pub const VOICES: usize = 4;
const TARGET_FREQUENCY: f32 = 13.;

pub struct Grains {
  delay_line: DelayLine,
  grains: Vec<Grain>,
  phasor: Phasor,
  gain_correction: f32,
}

impl Grains {
  pub fn new(sample_rate: f32) -> Self {
    let grains = (0..VOICES).map(|i| Grain::new(sample_rate, i)).collect();

    Self {
      delay_line: DelayLine::new((sample_rate * 0.2) as usize, sample_rate),
      grains,
      phasor: Phasor::new(sample_rate),
      gain_correction: (VOICES as f32 / 2.).recip(),
    }
  }

  pub fn process(&mut self, input: f32, speed: f32, detected_freq: f32) -> f32 {
    let grain_freq = Self::get_grain_freq(detected_freq);
    let phasor = self.phasor.process(grain_freq * speed);

    let grains_out = self
      .grains
      .iter_mut()
      .map(|grain| grain.process(&self.delay_line, phasor, grain_freq, speed))
      .sum::<f32>()
      * self.gain_correction;

    self.delay_line.write(input);

    grains_out
  }

  fn get_grain_freq(freq: f32) -> f32 {
    let divider = ((freq / TARGET_FREQUENCY / 4.).trunc() * 4.).max(4.);
    freq / divider
  }
}
