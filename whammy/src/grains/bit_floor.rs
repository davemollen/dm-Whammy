pub trait BitFloor {
  fn bit_floor(self) -> Self;
}

impl BitFloor for u32 {
  fn bit_floor(mut self: u32) -> u32 {
    if !self.is_power_of_two() {
      self |= self >> 1;
      self |= self >> 2;
      self |= self >> 4;
      self |= self >> 8;
      self |= self >> 16;
      return self - (self >> 1);
    } else {
      self
    }
  }
}

#[cfg(test)]
mod tests {
  use super::BitFloor;

  #[test]
  fn floor() {
    assert_eq!(0_u32.bit_floor(), 0_u32);
    assert_eq!(1_u32.bit_floor(), 1_u32);
    assert_eq!(2_u32.bit_floor(), 2_u32);
    assert_eq!(3_u32.bit_floor(), 2_u32);
    assert_eq!(4_u32.bit_floor(), 4_u32);
    assert_eq!(5_u32.bit_floor(), 4_u32);
    assert_eq!(7_u32.bit_floor(), 4_u32);
    assert_eq!(8_u32.bit_floor(), 8_u32);
    assert_eq!(9_u32.bit_floor(), 8_u32);
    assert_eq!(15_u32.bit_floor(), 8_u32);
    assert_eq!(16_u32.bit_floor(), 16_u32);
    assert_eq!(17_u32.bit_floor(), 16_u32);
    assert_eq!(31_u32.bit_floor(), 16_u32);
    assert_eq!(32_u32.bit_floor(), 32_u32);
    assert_eq!(33_u32.bit_floor(), 32_u32);
    assert_eq!(63_u32.bit_floor(), 32_u32);
    assert_eq!(64_u32.bit_floor(), 64_u32);
    assert_eq!(65_u32.bit_floor(), 64_u32);
    assert_eq!(127_u32.bit_floor(), 64_u32);
    assert_eq!(128_u32.bit_floor(), 128_u32);
    assert_eq!(129_u32.bit_floor(), 128_u32);
    assert_eq!(255_u32.bit_floor(), 128_u32);
    assert_eq!(256_u32.bit_floor(), 256_u32);
    assert_eq!(257_u32.bit_floor(), 256_u32);
  }
}
