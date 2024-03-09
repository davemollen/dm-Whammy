mod oversample;
use oversample::Oversample;
use std::simd::{f32x8, StdFloat};

pub struct Clipper {
  oversample: Oversample<f32x8>,
}

impl Clipper {
  pub fn new() -> Self {
    Self {
      oversample: Oversample::new(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    self.oversample.process(input, |x| {
      let x2 = x * x;
      let x3 = x2 * x;
      let x5 = x3 * x2;
      let a = x + (f32x8::splat(0.16489087) * x3) + (f32x8::splat(0.00985468) * x5);
      a / (f32x8::splat(1.0) + (a * a)).sqrt()
    }) * 0.558838
  }
}
