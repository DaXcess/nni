use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::Side,
};

pub fn handle_packet(id: u32, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  match packet.payload[1] {
    // Start
    Byte(0) => {
      println!("#{id} Server [Management Plugin]> Start process manager");
    }

    // Stop
    Byte(1) => {
      println!("#{id} Server [Management Plugin]> Stop process manager");
    }

    // Invalidate
    Byte(2) => {}

    // Update
    Byte(3) => {}

    // Process Added
    Byte(4) => {}

    // Process Removed
    Byte(5) => {}

    // Suspend Process
    Byte(6) => {
      let Int(pid) = packet.payload[2] else {return};

      println!("#{id} Server [Management Plugin]> Suspend process: {pid}");
    }

    // Resume Process
    Byte(7) => {
      let Int(pid) = packet.payload[2] else {return};

      println!("#{id} Server [Management Plugin]> Resume process: {pid}");
    }

    // Terminate Process
    Byte(8) => {
      let Int(pid) = packet.payload[2] else {return};
      let Boolean(tree) = packet.payload[3] else {return};

      println!(
        "#{id} Server [Management Plugin]> Terminate process{}: {pid}",
        if tree { " tree" } else { "" }
      );
    }

    // Restart Process
    Byte(9) => {
      let Int(pid) = packet.payload[2] else {return};

      println!("#{id} Server [Management Plugin]> Restart process: {pid}");
    }

    // Create Process
    Byte(10) => {
      let String(ref command) = packet.payload[2] else {return};
      let Boolean(admin) = packet.payload[3] else {return};

      println!("#{id} Server [Management Plugin]> Create process: {command} ({admin})");
    }

    _ => {}
  }
}
