#![feature(portable_simd)]
mod op_amp1;
use op_amp1::OpAmp1;
mod op_amp2;
use op_amp2::OpAmp2;
mod op_amp3;
use op_amp3::OpAmp3;
mod tone_stack;
use tone_stack::ToneStack;
mod clipper;
use clipper::Clipper;
mod contour;
use contour::Contour;
mod smooth_parameters;
use smooth_parameters::SmoothParameters;
pub mod shared {
  pub mod op_amp;
}

pub struct Shredmaster {
  op_amp1: OpAmp1,
  op_amp2: OpAmp2,
  clipper: Clipper,
  tone_stack: ToneStack,
  op_amp3: OpAmp3,
  contour: Contour,
  smooth_parameters: SmoothParameters<5>,
}

impl Shredmaster {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp1: OpAmp1::new(sample_rate),
      op_amp2: OpAmp2::new(sample_rate),
      clipper: Clipper::new(),
      tone_stack: ToneStack::new(sample_rate),
      op_amp3: OpAmp3::new(sample_rate),
      contour: Contour::new(sample_rate),
      smooth_parameters: SmoothParameters::new(sample_rate),
    }
  }

  pub fn process(
    &mut self,
    input: f32,
    gain: f32,
    bass: f32,
    contour: f32,
    treble: f32,
    volume: f32,
  ) -> f32 {
    let [gain, bass, contour, treble, volume] =
      self
        .smooth_parameters
        .process([gain, bass * bass, contour, treble, volume]);

    let op_amp1_output = self.op_amp1.process(input, gain);
    let op_amp2_output = self.op_amp2.process(op_amp1_output);
    let clipper_output = self.clipper.process(op_amp2_output);
    let tone_stack_output = self.tone_stack.process(clipper_output, bass, treble);
    let op_amp3_output = self.op_amp3.process(tone_stack_output);
    let contour_output = self.contour.process(op_amp3_output, contour);
    contour_output * volume
  }
}
