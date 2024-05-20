extern crate lv2;
extern crate whammy;
use lv2::prelude::*;
use whammy::{FloatExt, Whammy};

#[derive(PortCollection)]
struct Ports {
  dry: InputPort<Control>,
  wet: InputPort<Control>,
  pitch: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Whammy")]
struct DmWhammy {
  whammy: Whammy,
}

impl DmWhammy {
  pub fn get_dry_wet_levels(&self, ports: &mut Ports) -> (f32, f32) {
    (Self::dbtoa(*ports.dry), Self::dbtoa(*ports.wet))
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
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      whammy: Whammy::new(_plugin_info.sample_rate() as f32),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let (dry_level, wet_level) = self.get_dry_wet_levels(ports);
    let pitch = *ports.pitch;

    for (input, output) in ports.input.iter().zip(ports.output.iter_mut()) {
      *output = self.whammy.process(*input, pitch, dry_level, wet_level);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmWhammy);
