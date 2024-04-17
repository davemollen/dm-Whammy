use nih_plug::prelude::{FloatParam, FloatRange, Params};
use nih_plug_vizia::ViziaState;
use whammy::{MAX_PITCH, MIN_PITCH};
use std::sync::Arc;
mod custom_formatters;
use custom_formatters::v2s_f32_digits;

use crate::editor;

#[derive(Params)]
pub struct WhammyParameters {
  /// The editor state, saved together with the parameter state so the custom scaling can be
  /// restored.
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "pitch"]
  pub pitch: FloatParam,
}

impl Default for WhammyParameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      pitch: FloatParam::new(
        "Pitch",
        12.,
        FloatRange::Linear {
          min: MIN_PITCH,
          max: MAX_PITCH,
        },
      )
      .with_value_to_string(v2s_f32_digits(2)),
    }
  }
}
