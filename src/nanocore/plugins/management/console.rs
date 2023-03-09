use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::Side,
};

pub fn handle_packet(id: u32, packet: Packet, side: Side) {
  match packet.payload[1] {
    // Start
    Byte(0) => {
      if side.is_client() {
        return;
      }

      println!("#{id} Server [Management Plugin]> Start remote shell");
    }

    // Stop
    Byte(1) => {
      println!("#{id} {side} [Management Plugin]> Stop remote shell");
    }

    // Read
    Byte(2) => {
      if side.is_server() {
        return;
      }

      let Some(data) = packet.payload[2].as_ref_string() else {return};
      println!("#{id} Client [Management Plugin]> Console data: {data}");
    }

    // Write
    Byte(3) => {
      if side.is_client() {
        return;
      }

      let Some(data) = packet.payload[2].as_ref_string() else {return};
      println!("#{id} Server [Management Plugin]> Console command: {data}");
    }

    _ => {}
  }
}
