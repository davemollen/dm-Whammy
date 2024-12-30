use whammy::{Params, Whammy};

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut whammy = Whammy::new(44100.);
  let mut params = Params::new(44100.);
  params.set(12., 0., 0.);

  loop {
    let input = generate_signal();
    whammy.process(input, &mut params);
  }
}
