extern crate lv2;
extern crate whammy;
use lv2::prelude::*;
use whammy::{Params, Whammy};

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
  params: Params,
}

impl Plugin for DmWhammy {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      whammy: Whammy::new(sample_rate),
      params: Params::new(sample_rate),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self.params.set(*ports.pitch, *ports.dry, *ports.wet);

    for (input, output) in ports.input.iter().zip(ports.output.iter_mut()) {
      *output = self.whammy.process(*input, &mut self.params);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmWhammy);
