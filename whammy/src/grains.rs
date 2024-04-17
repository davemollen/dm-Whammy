mod grain;
use grain::Grain;
mod phasor;
use phasor::Phasor;
mod bit_floor;
use bit_floor::BitFloor;

use crate::{pitch_detector::MIN_FREQ, shared::{delay_line::DelayLine, delta::Delta}, MIN_PITCH};

const VOICES: usize = 4;

pub struct Grains {
  grain_delay_line: DelayLine,
  grains: Vec<Grain>,
  phasor: Phasor,
  delta: Delta,
}

impl Grains {
  pub fn new(sample_rate: f32) -> Self {
    let min_grain_freq = Self::get_grain_freq(MIN_FREQ, MIN_PITCH);

    Self {
      grain_delay_line: DelayLine::new((sample_rate * min_grain_freq.recip().ceil()) as usize, sample_rate),
      grains: vec![Grain::new(sample_rate); VOICES * 2],
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
    }
  }

  pub fn process(&mut self, input: f32, pitch: f32, freq: Option<f32>) -> f32 {
    match freq {
      Some(freq) => {
        let grain_freq = Self::get_grain_freq(freq, pitch);
        let phasor = self.phasor.process(grain_freq * VOICES as f32);
        let trigger = self.delta.process(phasor) < 0.;

        if trigger {
          self.set_grain_parameters(grain_freq, pitch);
        }
      }
      None => (),
    };

    let grain_delay_line = &mut self.grain_delay_line;
    let output = self
      .grains
      .iter_mut()
      .filter(|grain| !grain.is_free())
      .map(|grain| grain.process(grain_delay_line, pitch))
      .sum::<f32>() * 0.5;

    self.grain_delay_line.write(input);

    output
  }
  
  fn set_grain_parameters(&mut self, freq: f32, pitch: f32) {
    let window_size = 1000. / freq;

    let grain = self.grains.iter_mut().find(|grain | grain.is_free());
    match grain {
      Some(grain) => {
        grain.set_parameters(freq, window_size, pitch);
      }
      None => {}
    }
  }

  fn get_grain_freq(freq: f32, pitch: f32) -> f32 {
    let division = (freq / Self::get_target_grain_freq(pitch)).trunc() as u32;
    freq / division.bit_floor() as f32
  }

  fn get_target_grain_freq(pitch: f32) -> f32 {
    2_f32.powf(pitch / 36.) * 14.
  }
}
