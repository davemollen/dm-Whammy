mod ramp;
mod switch_and_ramp;
use std::f32::consts::TAU;

use {
  ramp::Ramp,
  switch_and_ramp::SwitchAndRamp
};
use crate::shared::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
};

#[derive(Clone)]
pub struct Grain {
  freq: f32,
  window_size: f32,
  time_ramp: Ramp,
  has_discontinuity: bool,
  switch_and_ramp: SwitchAndRamp
}

impl Grain {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      freq: 0.,
      window_size: 0.,
      time_ramp: Ramp::new(sample_rate),
      has_discontinuity: false,
      switch_and_ramp: SwitchAndRamp::new(sample_rate, 13000.)
    }
  }

  pub fn set_parameters(&mut self, freq: f32) {
    self.freq = freq;
    self.window_size = 1000. / freq;
    self.has_discontinuity = !self.time_ramp.is_finished();
    self.time_ramp.start();
  }

  pub fn process(&mut self, grain_delay_line: &mut DelayLine, speed: f32) -> f32 {
    let ramp = self.time_ramp.process(self.freq * speed);
    let time = ramp * self.window_size;
    let window = 0.5 - 0.5 * (ramp * TAU).fast_cos();
    
    let grains_out = grain_delay_line.read(time, Interpolation::Spline) * window;
    let output = self.switch_and_ramp.process(grains_out, self.has_discontinuity);
    self.has_discontinuity = false;
    output
  }
}