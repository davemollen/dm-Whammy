mod grain;
use grain::Grain;
mod phasor;
use phasor::Phasor;

use crate::shared::{delay_line::DelayLine, delta::Delta};

// VOICES needs to be a power of 2
const VOICES: usize = 4;
const TARGET_FREQUENCY: f32 = 12.;

pub struct Grains {
  grain_delay_line: DelayLine,
  grains: Vec<Grain>,
  phasor: Phasor,
  delta: Delta,
  voice_index: usize
}

impl Grains {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      grain_delay_line: DelayLine::new((sample_rate * 0.2) as usize, sample_rate),
      grains: vec![Grain::new(sample_rate); VOICES],
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
      voice_index: 0
    }
  }

  pub fn process(&mut self, input: f32, pitch: f32, freq: Option<f32>) -> f32 {
    let speed = Self::pitch_to_speed(pitch);
    match freq {
      Some(freq) => {
        let grain_freq = Self::get_grain_freq(freq);
        let phasor = self.phasor.process(grain_freq * speed * VOICES as f32);
        let trigger = self.delta.process(phasor).abs() > 0.5;

        if trigger {
          self.set_grain_parameters(grain_freq);
        }
      }
      None => (),
    };

    let grain_delay_line = &mut self.grain_delay_line;
    let output = self
      .grains
      .iter_mut()
      .map(|grain| grain.process(grain_delay_line, speed))
      .sum::<f32>() * (VOICES as f32 / 2.).recip();

    self.grain_delay_line.write(input);

    output
  }
  
  fn set_grain_parameters(&mut self, freq: f32) {
    self.grains[self.voice_index].set_parameters(freq);
    // increment from 0 to VOICES
    self.voice_index = self.voice_index + 1 & VOICES - 1;
  }

  fn pitch_to_speed(pitch: f32) -> f32 {
    1. - 2_f32.powf(pitch / 12.)
  }

  fn get_grain_freq(freq: f32) -> f32 {
    let divider = ((freq / TARGET_FREQUENCY / 4.).trunc() * 4.).max(4.);
    freq / divider
  }
}
