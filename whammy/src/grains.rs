mod grain;
use grain::Grain;
mod phasor;
use phasor::Phasor;

use crate::shared::{delay_line::DelayLine, delta::Delta};

const VOICES: usize = 4;

pub struct Grains {
  grain_delay_line: DelayLine,
  grains: Vec<Grain>,
  phasor: Phasor,
  delta: Delta,
}

impl Grains {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      grain_delay_line: DelayLine::new((sample_rate * 1.) as usize, sample_rate),
      grains: vec![Grain::new(sample_rate); VOICES * 2],
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
    }
  }

  pub fn process(&mut self, input: f32, pitch: f32, freq: Option<f32>) -> f32 {
    let speed = Self::pitch_to_speed(pitch);
    match freq {
      Some(freq) => {
        let grain_freq = Self::get_grain_freq(freq, speed);
        let phasor = self.phasor.process(grain_freq * VOICES as f32);
        let trigger = self.delta.process(phasor) < 0.;

        if trigger {
          self.set_grain_parameters(grain_freq, speed);
        }
      }
      None => (),
    };

    let grain_delay_line = &mut self.grain_delay_line;
    let output = self
      .grains
      .iter_mut()
      .filter(|grain| !grain.is_free())
      .map(|grain| grain.process(grain_delay_line, speed))
      .sum::<f32>() * (VOICES as f32 / 2.).recip();

    self.grain_delay_line.write(input);

    output
  }
  
  fn set_grain_parameters(&mut self, freq: f32, speed: f32) {
    let window_size = 1000. / freq;

    let grain = self.grains.iter_mut().find(|grain | grain.is_free());
    match grain {
      Some(grain) => {
        grain.set_parameters(freq, window_size, speed);
      }
      None => {}
    }
  }

  fn pitch_to_speed(pitch: f32) -> f32 {
    1. - 2_f32.powf(pitch / 12.)
  }

  fn get_grain_freq(freq: f32, speed: f32) -> f32 {
    let division = if freq < 160. {
      freq / 2.
    } else if freq < 320. {
      freq / 4.
    } else if freq < 640. {
      freq / 8.
    } else {
      freq / 16.
    }.trunc();
    let grain_freq = freq * speed.abs() / division;

    if grain_freq < 7. {
      grain_freq * 2.
    } else {
      grain_freq
    }
  }
}
