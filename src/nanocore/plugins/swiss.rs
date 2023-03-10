use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 94186362168708302010152756118690361493;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  let Some(Byte(command)) = packet.payload.get(0) else {return};
  let Some(Byte(subcommand)) = packet.payload.get(1) else {return};
  let Some(String(instruction)) = packet.payload.get(2) else {return};

  if *command != 0 || *subcommand != 0 {
    return;
  }

  let instruction = match instruction.as_str() {
    "DisableWebcamLights" => "Disable",
    "EnableWebcamLights" => "Enable",
    _ => "Unknown",
  };

  println!(
    "#{} [NanoCoreSwiss]> {instruction} Webcam Lights",
    state.id()
  );
}
