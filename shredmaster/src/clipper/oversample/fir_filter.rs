use super::{simd_type::SimdType, Coefficients};

pub trait SimdFir {
  type Float;

  fn new(length: usize) -> Self;
  fn write(&mut self, input: Self::Float);
  fn convolve(&self) -> Self::Float;
  fn process(&mut self, input: Self::Float) -> Self::Float;
}

pub struct FirFilter<T> {
  buffer: Vec<T>,
  coefficients: Vec<T>,
}

impl<T: SimdType> SimdFir for FirFilter<T>
where
  Vec<T>: Coefficients,
{
  type Float = T;

  fn new(length: usize) -> Self {
    Self {
      buffer: vec![T::splat(0.); length],
      coefficients: Coefficients::new(),
    }
  }

  fn process(&mut self, input: Self::Float) -> T {
    self.write(input);
    self.convolve()
  }

  fn write(&mut self, input: Self::Float) {
    self.buffer.insert(0, input);
    self.buffer.pop();
  }

  fn convolve(&self) -> T {
    let coefficients = &self.coefficients;

    self
      .buffer
      .iter()
      .zip(coefficients)
      .map(|(input, coeff)| *input * *coeff)
      .sum::<T>()
  }
}
