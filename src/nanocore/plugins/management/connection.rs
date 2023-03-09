use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::Side,
};

pub fn handle_packet(id: u32, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  match packet.payload[1] {
    // Invalidate
    Byte(0) => {}

    // Update
    Byte(1) => {}

    // Connection Added
    Byte(2) => {}

    // Connection Removed
    Byte(3) => {}

    // Terminate Connection
    Byte(4) => {
      println!("#{id} Server [Management Plugin]> Terminate a connection");
    }

    _ => {}
  }
}
