use whammy::Whammy;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut whammy = Whammy::new(44100.);

  let params_to_smooth = whammy.params_to_smooth(12., 0., 0.);
  whammy.initialize_params_to_smooth(params_to_smooth);
  let (speed, dry_level, wet_level) = params_to_smooth;

  loop {
    let input = generate_signal();
    whammy.process(input, speed, dry_level, wet_level);
  }
}
