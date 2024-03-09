use nih_plug::{
  formatters::{s2v_f32_percentage, v2s_f32_percentage},
  prelude::{FloatParam, FloatRange, Params},
};
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

use crate::editor;

#[derive(Params)]
pub struct ShredmasterParameters {
  /// The editor state, saved together with the parameter state so the custom scaling can be
  /// restored.
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "gain"]
  pub gain: FloatParam,

  #[id = "bass"]
  pub bass: FloatParam,

  #[id = "contour"]
  pub contour: FloatParam,

  #[id = "treble"]
  pub treble: FloatParam,

  #[id = "volume"]
  pub volume: FloatParam,
}

impl Default for ShredmasterParameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      gain: FloatParam::new("Gain", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      bass: FloatParam::new("Bass", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      contour: FloatParam::new("Contour", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      treble: FloatParam::new("Treble", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      volume: FloatParam::new("Volume", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),
    }
  }
}
