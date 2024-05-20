mod grain;
mod phasor;
use {crate::shared::ramp_smooth::RampSmooth, grain::Grain, phasor::Phasor};

use crate::shared::delay_line::DelayLine;

// VOICES needs to be a power of 2
pub const VOICES: usize = 4;
const TARGET_FREQUENCY: f32 = 13.;

pub struct Grains {
  xfade: RampSmooth,
  grain_delay_line: DelayLine,
  grains: Vec<Grain>,
  phasor: Phasor,
  gain_correction: f32,
}

impl Grains {
  pub fn new(sample_rate: f32) -> Self {
    let grains = (0..VOICES).map(|i| Grain::new(sample_rate, i)).collect();

    Self {
      xfade: RampSmooth::new(sample_rate, 100.),
      grain_delay_line: DelayLine::new((sample_rate * 0.2) as usize, sample_rate),
      grains,
      phasor: Phasor::new(sample_rate),
      gain_correction: (VOICES as f32 / 2.).recip(),
    }
  }

  pub fn process(&mut self, input: f32, speed: f32, detected_freq: f32) -> f32 {
    let fade_a = self.xfade.process(if speed == 0. { 1. } else { 0. });
    let fade_b = 1. - fade_a;

    if fade_a == 1. {
      input
    } else {
      let grain_freq = Self::get_grain_freq(detected_freq);
      let phasor = self.phasor.process(grain_freq * speed);

      let grain_delay_line = &mut self.grain_delay_line;
      let output = self
        .grains
        .iter_mut()
        .map(|grain| grain.process(grain_delay_line, phasor, grain_freq, speed))
        .sum::<f32>()
        * self.gain_correction;

      self.grain_delay_line.write(input);

      input * fade_a + output * fade_b
    }
  }

  fn get_grain_freq(freq: f32) -> f32 {
    let divider = ((freq / TARGET_FREQUENCY / 4.).trunc() * 4.).max(4.);
    freq / divider
  }
}
