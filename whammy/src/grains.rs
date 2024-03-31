mod grain;
use grain::Grain;
mod phasor;
use phasor::Phasor;

use crate::shared::{delay_line::DelayLine, delta::Delta};

const TARGET_FREQUENCY: f32 = 20.;
const VOICES: usize = 4;

pub struct Grains {
  grain_delay_line: DelayLine,
  grains: Vec<Grain>,
  index: usize,
  phasor: Phasor,
  delta: Delta,
}

impl Grains {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      grain_delay_line: DelayLine::new((sample_rate * 5.) as usize, sample_rate),
      grains: vec![Grain::new(sample_rate); VOICES],
      index: 0,
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
    }
  }

  pub fn process(&mut self, input: f32, pitch: f32, freq: f32, pitch_trigger: bool) -> f32 {
    // get a subdivision that's a power of two or a whole number, but also fits in a specified range (like between 5 and 20Hz)

    let offset = 1000. / freq;
    let grain_freq = freq / ((freq / TARGET_FREQUENCY).trunc());
    let phasor = self.phasor.process(grain_freq * VOICES as f32);
    let trigger = self.delta.process(phasor) < 0.;
    if pitch_trigger {
      self.phasor.reset();
    }

    if trigger {
      self.set_grain_parameters(grain_freq, pitch, offset);
    }

    let grain_delay_line = &mut self.grain_delay_line;
    let output = self
      .grains
      .iter_mut()
      .filter(|grain| !grain.is_free())
      .map(|grain| grain.process(grain_delay_line, pitch))
      .sum();

    self.grain_delay_line.write(input);

    output
  }

  fn set_grain_parameters(&mut self, freq: f32, pitch: f32, start_position: f32) {
    let window_size = 1000. / freq;

    let (start, end) = self.grains.split_at(self.index);
    let index = start.iter().chain(end).position(|grain| grain.is_free());
    match index {
      Some(i) => {
        self.grains[i].set_parameters(freq, window_size, pitch, start_position);
        self.index = i;
      }
      None => {}
    }
  }
}
