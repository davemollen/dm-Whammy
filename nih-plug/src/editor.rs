#[path = "./editor/components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
mod ui_data;
use crate::whammy_parameters::WhammyParameters;
use nih_plug::params::Param;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::{
  model::Model,
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::Units::{Pixels, Stretch},
  style::FontWeightKeyword,
  views::{HStack, Label, VStack},
};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;
use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(|| (440, 200))
}

pub(crate) fn create(
  params: Arc<WhammyParameters>,
  editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(
    editor_state,
    ViziaTheming::Custom,
    move |cx, gui_context| {
      let _ = cx.add_stylesheet(STYLE);

      UiData {
        params: params.clone(),
        gui_context: gui_context.clone(),
      }
      .build(cx);

      VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.pitch.name(),
            UiData::params,
            params.pitch.as_ptr(),
            |params| &params.pitch,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );
        })
        .child_space(Stretch(1.0));

        Label::new(cx, "Whammy")
          .font_size(32.0)
          .font_weight(FontWeightKeyword::ExtraBold)
          .color("#C9C06A")
          .border_color("#C9C06A")
          .border_width(Pixels(1.0))
          .child_space(Stretch(1.0))
          .width(Pixels(200.0))
          .height(Pixels(36.0))
          .top(Pixels(32.0))
          .bottom(Pixels(32.0))
          .left(Stretch(1.0))
          .right(Stretch(1.0));
      })
      .child_space(Pixels(16.0))
      .background_color("#161616");
    },
  )
}
