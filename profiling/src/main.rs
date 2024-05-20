use whammy::Whammy;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut whammy = Whammy::new(44100.);

  loop {
    let input = generate_signal();
    whammy.process(input, 12., 1., 1.);
  }
}
