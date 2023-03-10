use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::Side,
};

pub fn handle_packet(id: u32, packet: Packet, side: Side) {
  let Some(Byte(command)) = packet.payload.get(1) else {return};

  match *command {
    // Start
    0 => {
      if side.is_client() {
        return;
      }

      println!("#{id} Server [Management Plugin]> Start remote shell");
    }

    // Stop
    1 => {
      println!("#{id} {side} [Management Plugin]> Stop remote shell");
    }

    // Read
    2 => {
      if side.is_server() {
        return;
      }

      let Some(String(data)) = packet.payload.get(2) else {return};
      println!("#{id} Client [Management Plugin]> Console data: {data}");
    }

    // Write
    3 => {
      if side.is_client() {
        return;
      }

      let Some(String(data)) = packet.payload.get(2) else {return};
      println!("#{id} Server [Management Plugin]> Console command: {data}");
    }

    _ => {}
  }
}
