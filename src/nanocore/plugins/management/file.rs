use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::Side,
};

pub fn handle_packet(id: u32, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  match packet.payload[1] {
    // Common Paths
    Byte(0) => {}

    // File System Entries
    Byte(1) => {
      let String(ref path) = packet.payload[2] else {return};

      println!("#{id} Server [Management Plugin]> Read directory: {path}");
    }

    // Execute
    Byte(2) => {
      let String(ref path) = packet.payload[2] else {return};

      // Array of [is_folder, name]
      for chunk in packet.payload[3..].chunks(2) {
        if let [Boolean(_), String(name)] = chunk {
          println!("#{id} Server [Management Plugin]> Execute: {path}\\{name}");
        }
      }
    }

    // Delete
    Byte(3) => {
      let String(ref path) = packet.payload[2] else {return};

      for chunk in packet.payload[3..].chunks(2) {
        if let [Boolean(directory), String(name)] = chunk {
          println!(
            "#{id} Server [Management Plugin]> Deleted {} {path}\\{name}",
            if *directory { "directory" } else { "file" }
          )
        }
      }
    }

    // File Folder Transfer
    Byte(4) => {
      let String(ref path) = packet.payload[2] else {return};

      println!("#{id} Server [Management Plugin]> Downloading directory: {path}");
    }

    // Initialize Transfer
    Byte(5) => {}

    // Write Block Data
    Byte(6) => {
      let String(ref path) = packet.payload[2] else {return};

      println!("#{id} Server [Management Plugin]> Write data to file: {path}");
    }

    // Read Block Data
    Byte(7) => {
      let String(ref path) = packet.payload[2] else {return};

      println!("#{id} Server [Management Plugin]> Read data from file: {path}");
    }

    _ => {}
  }
}
