mod ramp;
use super::{TARGET_FREQUENCY, VOICES};
use crate::shared::{
  delay_line::{DelayLine, Interpolation},
  delta::Delta,
  float_ext::FloatExt,
};
use ramp::Ramp;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Grain {
  freq: f32,
  window_size: f32,
  time_ramp: Ramp,
  phase_offset: f32,
  delta: Delta,
}

impl Grain {
  pub fn new(sample_rate: f32, index: usize) -> Self {
    let phase_offset = (VOICES as f32).recip() * index as f32;

    Self {
      freq: TARGET_FREQUENCY,
      window_size: TARGET_FREQUENCY.recip() * 1000.,
      time_ramp: Ramp::new(sample_rate, phase_offset),
      phase_offset,
      delta: Delta::new(),
    }
  }

  pub fn process(&mut self, delay_line: &DelayLine, phasor: f32, freq: f32, speed: f32) -> f32 {
    let phase = Self::wrap(phasor + self.phase_offset);
    let trigger = self.delta.process(phase).abs() > 0.5;
    if trigger {
      self.freq = freq;
      self.window_size = freq.recip() * 1000.;
      self.time_ramp.start();
    }

    let ramp = self.time_ramp.process(self.freq * speed);
    let time = ramp * self.window_size;
    let window = (ramp * PI).fast_sin() * (phase * PI).fast_sin();
    delay_line.read(time, Interpolation::Spline) * window
  }

  fn wrap(input: f32) -> f32 {
    if input >= 1. {
      input - 1.
    } else {
      input
    }
  }
}
