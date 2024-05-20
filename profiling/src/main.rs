use whammy::Whammy;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut whammy = Whammy::new(44100.);

  let pitch = 12.;
  let speed = 1. - 2_f32.powf(pitch / 12.);

  loop {
    let input = generate_signal();
    whammy.process(input, speed, 1., 1.);
  }
}
