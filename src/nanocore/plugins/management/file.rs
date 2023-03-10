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
    // Common Paths
    0 => {}

    // File System Entries
    1 => {
      let Some(String(path)) = packet.payload.get(2) else {return};

      println!("#{id} [Management Plugin]> Read directory: {path}");
    }

    // Execute
    2 => {
      let Some(String(path)) = packet.payload.get(2) else {return};
      let Some(subsection) = packet.payload.get(3..) else {return};

      // Array of [is_folder, name]
      for chunk in subsection.chunks(2) {
        if let [Boolean(_), String(name)] = chunk {
          println!("#{id} [Management Plugin]> Execute: {path}\\{name}");
        }
      }
    }

    // Delete
    3 => {
      let Some(String(path)) = packet.payload.get(2) else {return};
      let Some(subsection) = packet.payload.get(3..) else {return};

      for chunk in subsection.chunks(2) {
        if let [Boolean(directory), String(name)] = chunk {
          println!(
            "#{id} [Management Plugin]> Deleted {} {path}\\{name}",
            if *directory { "directory" } else { "file" }
          )
        }
      }
    }

    // File Folder Transfer
    4 => {
      let Some(String(path)) = packet.payload.get(2) else {return};

      println!("#{id} [Management Plugin]> Downloading directory: {path}");
    }

    // Initialize Transfer
    5 => {}

    // Write Block Data
    6 => {
      let Some(String(path)) = packet.payload.get(2) else {return};

      println!("#{id} [Management Plugin]> Write data to file: {path}");
    }

    // Read Block Data
    7 => {
      let Some(String(path)) = packet.payload.get(2) else {return};

      println!("#{id} [Management Plugin]> Read data from file: {path}");
    }

    _ => {}
  }
}
