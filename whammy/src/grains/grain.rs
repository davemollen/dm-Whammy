mod ramp;
use ramp::Ramp;
use std::f32::consts::TAU;

use crate::shared::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
};

#[derive(Clone)]
pub struct Grain {
  freq: f32,
  window_size: f32,
  time_ramp: Ramp,
}

impl Grain {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      freq: 0.,
      window_size: 0.,
      time_ramp: Ramp::new(sample_rate),
    }
  }

  // pub fn is_free(&self) -> bool {
  //   self.time_ramp.is_finished()
  // }

  pub fn set_parameters(&mut self, freq: f32) {
    self.freq = freq;
    self.window_size = 1000. / freq;
    self.time_ramp.start();
  }

  pub fn process(&mut self, grain_delay_line: &mut DelayLine, speed: f32) -> f32 {
    let ramp = self.time_ramp.process(self.freq * speed);
    let time = ramp * self.window_size;
    let window = 0.5 - 0.5 * (ramp * TAU).fast_cos();

    let grains_out = grain_delay_line.read(time, Interpolation::Spline);
    grains_out * window
  }
}