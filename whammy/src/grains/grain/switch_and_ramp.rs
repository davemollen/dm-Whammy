use std::f32::consts::PI;

use crate::shared::float_ext::FloatExt;

#[derive(Clone)]
pub struct SwitchAndRamp {
  trigger: bool,
  last_input: f32,
  env: f32,
  coefficient: f32
}

impl SwitchAndRamp {
  pub fn new(sample_rate: f32, freq: f32) -> Self {
    Self {
      trigger: false,
      last_input: 0.,
      env: 0.,
      coefficient: (freq * 2. * PI / sample_rate).fast_sin().clamp(0., 1.)
    }
  }

  pub fn start(&mut self) {
    self.trigger = true;
  }

  // TODO: check if can prevent a trigger when (input - self.last_input) is small
  pub fn process(&mut self, input: f32) -> f32 {
    self.env = if self.trigger {
      self.trigger = false;
      self.env - (input - self.last_input)
    } else if self.env.is_subnormal() {
      0.
    } else { 
      self.env * self.coefficient
    };
    
    self.last_input = input;
    input + self.env
  }
}