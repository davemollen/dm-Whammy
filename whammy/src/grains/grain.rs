mod ramp;
use ramp::Ramp;
use std::f32::consts::PI;
use super::VOICES;
use crate::shared::{
  delay_line::{DelayLine, Interpolation}, delta::Delta, float_ext::FloatExt
};

#[derive(Clone)]
pub struct Grain {
  freq: f32,
  window_size: f32,
  time_ramp: Ramp,
  phase_offset: f32,
  delta: Delta
}

impl Grain {
  pub fn new(sample_rate: f32, index: usize) -> Self {
    Self {
      freq: 0.,
      window_size: 0.,
      time_ramp: Ramp::new(sample_rate),
      phase_offset: (VOICES as f32).recip() * index as f32,
      delta: Delta::new()
    }
  }

  pub fn process(&mut self, grain_delay_line: &mut DelayLine, phasor: f32, freq: f32, speed: f32) -> f32 {
    let phase = (phasor + self.phase_offset) % 1.;
    let trigger = self.delta.process(phase).abs() > 0.5;
    if trigger {
      self.freq = freq;
      self.window_size = 1000. / freq;
      self.time_ramp.start();
    }
    
    let ramp = self.time_ramp.process(self.freq * speed);
    let time = ramp * self.window_size;
    let window = (ramp * PI).fast_sin() * (phase * PI).fast_sin();
    grain_delay_line.read(time, Interpolation::Spline) * window
  }
}