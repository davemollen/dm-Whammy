mod ramp;
use ramp::Ramp;
use std::f32::consts::PI;

use crate::shared::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
};

#[derive(Clone)]
pub struct Grain {
  freq: f32,
  start_position: f32,
  window_size: f32,
  time_ramp: Ramp,
  time_ramp_max: f32,
}

impl Grain {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      freq: 0.,
      start_position: 0.,
      window_size: 0.,
      time_ramp: Ramp::new(sample_rate),
      time_ramp_max: 1.,
    }
  }

  pub fn is_free(&self) -> bool {
    self.time_ramp.is_finished()
  }

  pub fn set_parameters(&mut self, freq: f32, window_size: f32, pitch: f32, offset: f32) {
    self.freq = freq;
    self.window_size = window_size;
    self.start_position = offset;
    self.time_ramp.start(None);

    let speed = 2_f32.powf(pitch / 12.);
    self.time_ramp_max = ((1. - speed) * freq).abs() / freq;
  }

  pub fn process(&mut self, grain_delay_line: &mut DelayLine, pitch: f32) -> f32 {
    let speed = 2_f32.powf(pitch / 12.);
    let time = self.get_time(speed);
    let window = self.get_window();

    let grains_out = grain_delay_line.read(time + self.start_position, Interpolation::Linear);
    grains_out * window * window
  }

  fn get_time(&mut self, speed: f32) -> f32 {
    if self.time_ramp_max == 0. {
      self.time_ramp.run(self.freq, 0., 1.);
      0.
    } else {
      let ramp_freq = (1. - speed) * self.freq;
      self.time_ramp.run(ramp_freq, 0., self.time_ramp_max) * self.window_size
    }
  }

  fn get_window(&mut self) -> f32 {
    (self.time_ramp.get_progress() * PI).fast_sin()
  }
}
