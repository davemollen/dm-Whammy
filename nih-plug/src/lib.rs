use nih_plug::prelude::*;
use std::sync::Arc;
use whammy::{FloatExt, Whammy};
mod whammy_parameters;
use whammy_parameters::WhammyParameters;
mod editor;

struct DmWhammy {
  params: Arc<WhammyParameters>,
  whammy: Whammy,
}

impl Default for DmWhammy {
  fn default() -> Self {
    let params = Arc::new(WhammyParameters::default());
    Self {
      params: params.clone(),
      whammy: Whammy::new(44100.),
    }
  }
}

impl DmWhammy {
  pub fn get_dry_wet_levels(&self) -> (f32, f32) {
    (
      Self::dbtoa(self.params.dry.value()),
      Self::dbtoa(self.params.wet.value()),
    )
  }

  fn dbtoa(level: f32) -> f32 {
    if level <= -70. {
      0.
    } else {
      level.dbtoa()
    }
  }
}

impl Plugin for DmWhammy {
  const NAME: &'static str = "dm-Whammy";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-Whammy";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(1),
    main_output_channels: NonZeroU32::new(1),
    ..AudioIOLayout::const_default()
  }];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.whammy = Whammy::new(buffer_config.sample_rate);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let (dry_level, wet_level) = self.get_dry_wet_levels();
    let pitch = self.params.pitch.value();

    buffer.iter_samples().for_each(|mut channel_samples| {
      let sample = channel_samples.iter_mut().next().unwrap();
      let whammy_output = self.whammy.process(*sample, pitch, dry_level, wet_level);
      *sample = whammy_output;
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmWhammy {
  const CLAP_ID: &'static str = "dm-Whammy";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A pitchshift plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Mono,
    ClapFeature::Utility,
    ClapFeature::PitchShifter,
  ];
}

impl Vst3Plugin for DmWhammy {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-Whammy.......";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx,
    Vst3SubCategory::Mono,
    Vst3SubCategory::PitchShift,
  ];
}

nih_export_clap!(DmWhammy);
nih_export_vst3!(DmWhammy);
