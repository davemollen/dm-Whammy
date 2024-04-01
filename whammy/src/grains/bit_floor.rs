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
    assert_eq!(2_u32.bit_floor(), 2_u32);
    assert_eq!(3_u32.bit_floor(), 2_u32);
  }
}
