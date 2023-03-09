mod browser;
mod core;
mod management;

use super::{NanoState, Side};
use crate::decrypt::Packet;
use std::collections::HashMap;
use uuid::Uuid;

pub struct PluginManager {
  plugins: HashMap<u128, (String, Box<dyn Fn(&mut NanoState, Packet, Side) -> ()>)>,
}

impl PluginManager {
  pub fn new() -> Self {
    let mut plugins: HashMap<u128, (String, Box<dyn Fn(&mut NanoState, Packet, Side) -> ()>)> =
      HashMap::new();
    plugins.insert(
      core::PLUGIN_GUID,
      ("Core Plugin".to_string(), Box::new(core::handle_packet)),
    );
    plugins.insert(
      management::PLUGIN_GUID,
      (
        "Management Plugin".to_string(),
        Box::new(management::handle_packet),
      ),
    );
    plugins.insert(
      browser::PLUGIN_GUID,
      ("NanoBrowser".to_string(), Box::new(browser::handle_packet)),
    );

    Self { plugins }
  }

  pub fn handle_packet(&self, state: &mut NanoState, packet: Packet, side: Side) {
    let Some(uuid) = packet.plugin_guid else {return};
    let Some((_, handler)) = self.plugins.get(&uuid.as_u128()) else {return};

    handler(state, packet, side);
  }

  pub fn plugin_name(&self, uuid: &Uuid) -> Option<String> {
    self
      .plugins
      .get(&uuid.as_u128())
      .map(|(name, _)| name.clone())
  }
}
