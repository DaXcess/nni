use uuid::Uuid;

use crate::{
  decrypt::{
    Packet,
    Payload::{Byte, String as PString, UShort},
  },
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 463021573153331220126860247765896765;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  let Some(Byte(command)) = packet.payload.get(0) else {return};
  let Some(Byte(subcommand)) = packet.payload.get(1) else {return};

  // Proxy Command
  if *command != 0 {
    return;
  }

  // Connect
  if *subcommand != 0 {
    return;
  }

  let Some(PString(address)) = packet.payload.get(3) else {return};
  let Some(UShort(port)) = packet.payload.get(4) else {return};

  println!(
    "#{} [Network Plugin]> Connect: {address}:{port}",
    state.id()
  );
}

pub fn pipe_created(state: &mut NanoState, name: String, _uuid: Uuid) {
  if name == "42C6E406-C8E9-4686-9D95-22C275307589" {
    println!("#{} [Network Plugin]> Activated Reverse Proxy", state.id(),);
  }
}

pub fn connection_closed(state: &NanoState) {
  let Some(pipe_name) = state.pipe_name() else {return};

  if pipe_name == "42C6E406-C8E9-4686-9D95-22C275307589" {
    println!(
      "#{} [Network Plugin]> Deactivated Reverse Proxy",
      state.id(),
    );
  }
}
