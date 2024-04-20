#[derive(Clone)]
pub struct Ramp {
  sample_rate: f32,
  x: f32,
  trigger: bool
}

impl Ramp {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      x: 0.,
      trigger: false,
    }
  }

  pub fn start(&mut self) {
    self.trigger = true;
  }

  pub fn process(&mut self, speed: f32) -> f32 {
    let step_size = self.sample_rate.recip() * speed;

    if self.trigger {
      if speed > 0. {
        self.x = 0.;
      } else {
        self.x = 1.;
      }
      self.trigger = false;
    }

    if (speed > 0. && self.x < 1.) || (speed < 0. && self.x > 0.) {
      self.x += step_size;
    }

    self.x
  }
}