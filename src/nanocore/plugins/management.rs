mod connection;
mod console;
mod file;
mod process;
mod registry;

use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 189193294789931535124297045847765788632;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  match packet.payload[0] {
    // Registry command
    Byte(0) => registry::handle_packet(state.id(), packet, side),

    // Process command
    Byte(1) => process::handle_packet(state.id(), packet, side),

    // File command
    Byte(2) => file::handle_packet(state.id(), packet, side),

    // Console command
    Byte(3) => console::handle_packet(state.id(), packet, side),

    // Connection command
    Byte(4) => connection::handle_packet(state.id(), packet, side),

    _ => {}
  }
}
