use std::f32::consts::PI;

use crate::shared::float_ext::FloatExt;

#[derive(Clone)]
pub struct SwitchAndRamp {
  last_input: f32,
  env: f32,
  coefficient: f32
}

impl SwitchAndRamp {
  pub fn new(sample_rate: f32, freq: f32) -> Self {
    Self {
      last_input: 0.,
      env: 0.,
      coefficient: (freq * 2. * PI / sample_rate).fast_sin().clamp(0., 1.)
    }
  }

  pub fn process(&mut self, input: f32, trigger: bool) -> f32 {
    if self.env.is_subnormal() && !trigger {
      input
    } else {  
      self.env = if trigger {
        self.env - (input - self.last_input)
      } else { 
        self.env * self.coefficient
      };
      
      self.last_input = input;
      input + self.env
    }

  }
}