mod browser;
mod core;
mod management;
mod misc;
mod multi;
mod nana;
mod network;
mod security;
mod stress;
mod surveillance;
mod surveillance_ex;
mod swiss;
mod tools;

use super::{NanoState, Side};
use crate::decrypt::Packet;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Plugin {
  name: String,
  handler: Box<dyn Fn(&mut NanoState, Packet, Side) -> ()>,
  pipe_created: Option<Box<dyn Fn(&mut NanoState, String, Uuid) -> ()>>,
  connection_closed: Option<Box<dyn Fn(&NanoState) -> ()>>,
}

impl Plugin {
  pub fn create(
    name: impl Into<String>,
    handler: impl Fn(&mut NanoState, Packet, Side) -> () + 'static,
  ) -> Self {
    Self {
      name: name.into(),
      handler: Box::new(handler),
      pipe_created: None,
      connection_closed: None,
    }
  }

  pub fn set_pipe_created_fn(
    &mut self,
    pipe_created: impl Fn(&mut NanoState, String, Uuid) -> () + 'static,
  ) {
    self.pipe_created = Some(Box::new(pipe_created));
  }

  pub fn set_connection_closed_fn(
    &mut self,
    connection_closed: impl Fn(&NanoState) -> () + 'static,
  ) {
    self.connection_closed = Some(Box::new(connection_closed));
  }
}

pub struct PluginManager {
  plugins: HashMap<u128, Plugin>,
}

impl PluginManager {
  pub fn new() -> Self {
    let core_plugin = Plugin::create("Core Plugin", core::handle_packet);
    let management_plugin = Plugin::create("Management Plugin", management::handle_packet);
    let misc_plugin = Plugin::create("Misc Plugin", misc::handle_packet);
    let multicore_plugin = Plugin::create("MultiCore", multi::handle_packet);
    let swiss_plugin = Plugin::create("NanoCoreSwiss", swiss::handle_packet);
    let nanobrowser_plugin = Plugin::create("NanoBrowser", browser::handle_packet);
    let mut nanonana_plugin = Plugin::create("NanoNana", nana::handle_packet);
    let nanostress_plugin = Plugin::create("NanoStress", stress::handle_packet);
    let mut network_plugin = Plugin::create("Network Plugin", network::handle_packet);
    let security_plugin = Plugin::create("Security Plugin", security::handle_packet);
    let surveillanceex_plugin =
      Plugin::create("SurveillanceEx Plugin", surveillance_ex::handle_packet);
    let mut surveillance_plugin =
      Plugin::create("Surveillance Plugin", surveillance::handle_packet);
    let tools_plugin = Plugin::create("Tools Plugin", tools::handle_packet);

    nanonana_plugin.set_pipe_created_fn(nana::pipe_created);
    network_plugin.set_pipe_created_fn(network::pipe_created);

    network_plugin.set_connection_closed_fn(network::connection_closed);
    surveillance_plugin.set_connection_closed_fn(surveillance::connection_closed);

    let mut plugins: HashMap<u128, Plugin> = HashMap::new();

    plugins.insert(core::PLUGIN_GUID, core_plugin);
    plugins.insert(management::PLUGIN_GUID, management_plugin);
    plugins.insert(misc::PLUGIN_GUID, misc_plugin);
    plugins.insert(multi::PLUGIN_GUID, multicore_plugin);
    plugins.insert(swiss::PLUGIN_GUID, swiss_plugin);
    plugins.insert(browser::PLUGIN_GUID, nanobrowser_plugin);
    plugins.insert(nana::PLUGIN_GUID, nanonana_plugin);
    plugins.insert(stress::PLUGIN_GUID, nanostress_plugin);
    plugins.insert(network::PLUGIN_GUID, network_plugin);
    plugins.insert(security::PLUGIN_GUID, security_plugin);
    plugins.insert(surveillance_ex::PLUGIN_GUID, surveillanceex_plugin);
    plugins.insert(surveillance::PLUGIN_GUID, surveillance_plugin);
    plugins.insert(tools::PLUGIN_GUID, tools_plugin);

    Self { plugins }
  }

  pub fn handle_packet(&self, state: &mut NanoState, packet: Packet, side: Side) {
    let Some(uuid) = packet.plugin_guid else {return};
    let Some(plugin) = self.plugins.get(&uuid.as_u128()) else {return};

    (plugin.handler)(state, packet, side);
  }

  pub fn handle_pipe_created(&self, state: &mut NanoState, pipe_name: &str, pipe_id: &Uuid) {
    for (_, plugin) in self.plugins.iter() {
      let Some(ref handler) = plugin.pipe_created else {continue};
      handler(state, pipe_name.to_string(), pipe_id.clone());
    }
  }

  pub fn connection_closed(&self, state: &NanoState) {
    for (_, plugin) in self.plugins.iter() {
      let Some(ref handler) = plugin.connection_closed else {continue};
      handler(state);
    }
  }

  pub fn plugin_name(&self, uuid: &Uuid) -> Option<&str> {
    self
      .plugins
      .get(&uuid.as_u128())
      .map(|plugin| plugin.name.as_str())
  }
}
