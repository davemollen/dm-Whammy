mod grain;
use grain::Grain;
mod phasor;
use phasor::Phasor;
mod bit_floor;
use bit_floor::BitFloor;

use crate::shared::{delay_line::DelayLine, delta::Delta};

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
      grains: vec![Grain::new(sample_rate); VOICES * 2],
      index: 0,
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
    }
  }

  pub fn process(&mut self, input: f32, pitch: f32, freq: Option<f32>) -> f32 {
    match freq {
      Some(freq) => {
        // let offset = 1000. / freq;
        let offset = 0.;
        // let division = (freq / self.get_target_grain_freq(pitch) / 2.).trunc() * 2.;
        // let grain_freq = freq / division;
        let division = (freq / self.get_target_grain_freq(pitch)).trunc() as u32;
        let grain_freq = freq / division.bit_floor() as f32;
        let phasor = self.phasor.process(grain_freq * VOICES as f32);
        let trigger = self.delta.process(phasor) < 0.;

        if trigger {
          self.set_grain_parameters(grain_freq, pitch, offset);
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

  fn get_target_grain_freq(&self, pitch: f32) -> f32 {
    2_f32.powf(pitch / 36.) * 11.
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
