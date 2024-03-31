mod grain;
use grain::Grain;

use crate::shared::delay_line::DelayLine;

const TARGET_FREQUENCY: f32 = 15.;
const VOICES: usize = 4;

pub struct Grains {
  grain_delay_line: DelayLine,
  grains: Vec<Grain>,
  index: usize,
}

impl Grains {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      grain_delay_line: DelayLine::new((sample_rate * 5.) as usize, sample_rate),
      grains: vec![Grain::new(sample_rate); VOICES],
      index: 0,
    }
  }

  pub fn process(&mut self, input: f32, pitch: f32, freq: f32, trigger: bool) -> f32 {
    if trigger {
      self.set_grain_parameters(freq, pitch);
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

  fn set_grain_parameters(&mut self, freq: f32, pitch: f32) {
    let offset = 1000. / freq;
    // TODO: get a subdivision that's a power of two or a whole number, but also fits in a specified range (like between 5 and 20Hz)
    // let grain_freq = (freq / TARGET_FREQUENCY).trunc() / freq;
    let grain_freq = freq / VOICES as f32;
    let window_size = 1000. / grain_freq;

    let (start, end) = self.grains.split_at(self.index);
    let index = start.iter().chain(end).position(|grain| grain.is_free());
    match index {
      Some(i) => {
        self.grains[i].set_parameters(grain_freq, window_size, pitch, offset);
        self.index = i;
      }
      None => {}
    }
  }
}
