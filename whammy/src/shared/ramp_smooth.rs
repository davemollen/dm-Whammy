use crate::shared::float_ext::FloatExt;

pub struct RampSmooth {
  prev: f32,
  index: usize,
  step_size: f32,
  z: f32,
  ramp_time: usize,
  ramp_factor: f32,
}

impl RampSmooth {
  pub fn new(sample_rate: f32, freq: f32) -> Self {
    let ramp_time = (freq.recip() * 1000.).mstosamps(sample_rate);

    Self {
      prev: 0.,
      index: 0,
      step_size: 0.,
      z: 0.,
      ramp_time: ramp_time as usize,
      ramp_factor: ramp_time.recip(),
    }
  }

  pub fn initialize(&mut self, value: f32) {
    self.z = value;
  }

  pub fn process(&mut self, input: f32) -> f32 {
    if input.is_equal_to(self.z) {
      input
    } else {
      let difference = input - self.z;
      self.ramp(input, difference)
    }
  }

  fn ramp(&mut self, input: f32, difference: f32) -> f32 {
    if input != self.prev {
      self.index = self.ramp_time;
      self.step_size = difference * self.ramp_factor;
      self.prev = input;
    }

    if self.index > 0 {
      self.index -= 1;
      self.z += self.step_size;
    }
    self.z
  }
}

#[cfg(test)]
mod tests {
  use super::RampSmooth;

  #[test]
  fn should_ramp_up_and_down_in_time() {
    let ramp_time_in_samples = 8.0;
    let mut smoother = RampSmooth::new(1000., 1000. / ramp_time_in_samples);

    assert_eq!(smoother.process(1.), 0.125);
    assert_eq!(smoother.process(1.), 0.25);
    assert_eq!(smoother.process(1.), 0.375);
    assert_eq!(smoother.process(1.), 0.5);
    assert_eq!(smoother.process(1.), 0.625);
    assert_eq!(smoother.process(1.), 0.75);
    assert_eq!(smoother.process(1.), 0.875);
    assert_eq!(smoother.process(1.), 1.0);
    assert_eq!(smoother.process(1.), 1.0);
    assert_eq!(smoother.process(0.), 0.875);
    assert_eq!(smoother.process(0.), 0.75);
    assert_eq!(smoother.process(0.), 0.625);
    assert_eq!(smoother.process(0.), 0.5);
    assert_eq!(smoother.process(1.), 0.5625);
  }
}
