use std::{
  iter::Sum,
  ops::Mul,
  simd::{f32x16, f32x2, f32x4, f32x8, num::SimdFloat},
};

pub trait SimdType:
  Clone + Copy + Mul<Self> + Sum<<Self as Mul>::Output> + SimdFloat<Scalar = f32>
{
  fn oversample_factor() -> usize;
  fn splat(input: f32) -> Self;
}

impl SimdType for f32x2 {
  fn oversample_factor() -> usize {
    2
  }

  fn splat(input: f32) -> Self {
    f32x2::splat(input)
  }
}

impl SimdType for f32x4 {
  fn oversample_factor() -> usize {
    4
  }

  fn splat(input: f32) -> Self {
    f32x4::splat(input)
  }
}

impl SimdType for f32x8 {
  fn oversample_factor() -> usize {
    8
  }

  fn splat(input: f32) -> Self {
    f32x8::splat(input)
  }
}

impl SimdType for f32x16 {
  fn oversample_factor() -> usize {
    16
  }

  fn splat(input: f32) -> Self {
    f32x16::splat(input)
  }
}
