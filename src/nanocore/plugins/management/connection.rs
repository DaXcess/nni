use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::Side,
};

pub fn handle_packet(id: u32, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  let Some(Byte(command)) = packet.payload.get(1) else {return};

  match *command {
    // Invalidate
    0 => {}

    // Update
    1 => {}

    // Connection Added
    2 => {}

    // Connection Removed
    3 => {}

    // Terminate Connection
    4 => {
      println!("#{id} [Management Plugin]> Terminate a connection");
    }

    _ => {}
  }
}
