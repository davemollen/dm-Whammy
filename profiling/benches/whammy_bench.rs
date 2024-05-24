use criterion::{criterion_group, criterion_main, Criterion};
use whammy::Whammy;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn generate_stereo_signal_stream(length: usize) -> Vec<f32> {
  (0..length).map(|_| generate_signal()).collect()
}

fn whammy_bench(c: &mut Criterion) {
  let mut whammy = Whammy::new(44100.);
  let signal_stream = generate_stereo_signal_stream(44100);

  let params_to_smooth = whammy.get_params(12., 0., 0.);
  whammy.initialize_params(params_to_smooth);
  let (speed, dry_level, wet_level) = params_to_smooth;

  c.bench_function("whammy", |b| {
    b.iter(|| {
      for input in &signal_stream {
        whammy.process(*input, speed, dry_level, wet_level);
      }
    })
  });
}

criterion_group!(benches, whammy_bench);
criterion_main!(benches);
