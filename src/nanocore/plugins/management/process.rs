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
    // Start
    0 => {
      println!("#{id} [Management Plugin]> Start process manager");
    }

    // Stop
    1 => {
      println!("#{id} [Management Plugin]> Stop process manager");
    }

    // Invalidate
    2 => {}

    // Update
    3 => {}

    // Process Added
    4 => {}

    // Process Removed
    5 => {}

    // Suspend Process
    6 => {
      let Some(Int(pid)) = packet.payload.get(2) else {return};

      println!("#{id} [Management Plugin]> Suspend process: {pid}");
    }

    // Resume Process
    7 => {
      let Some(Int(pid)) = packet.payload.get(2) else {return};

      println!("#{id} [Management Plugin]> Resume process: {pid}");
    }

    // Terminate Process
    8 => {
      let Some(Int(pid)) = packet.payload.get(2) else {return};
      let Some(Boolean(tree)) = packet.payload.get(3) else {return};

      println!(
        "#{id} [Management Plugin]> Terminate process{}: {pid}",
        if *tree { " tree" } else { "" }
      );
    }

    // Restart Process
    9 => {
      let Some(Int(pid)) = packet.payload.get(2) else {return};

      println!("#{id} [Management Plugin]> Restart process: {pid}");
    }

    // Create Process
    10 => {
      let Some(String(command)) = packet.payload.get(2) else {return};
      let Some(Boolean(admin)) = packet.payload.get(3) else {return};

      println!("#{id} [Management Plugin]> Create process: {command} ({admin})");
    }

    _ => {}
  }
}
