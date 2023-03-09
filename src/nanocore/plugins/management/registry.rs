use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::Side,
};

pub fn handle_packet(id: u32, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  match packet.payload[1] {
    // Get keys
    Byte(0) => {
      let Int(hive) = packet.payload[2] else {return};
      let String(ref subkey) = packet.payload[3] else {return};

      let hive = get_hive(hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} Server [Management Plugin]> Get subkeys: {hive}{}{subkey}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Create key
    Byte(1) => {
      let Int(hive) = packet.payload[2] else {return};
      let String(ref subkey) = packet.payload[3] else {return};
      let String(ref key) = packet.payload[4] else {return};

      let hive = get_hive(hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} Server [Management Plugin]> Create new key in {hive}{}{subkey}: {key}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Rename key
    Byte(2) => {
      let Int(hive) = packet.payload[2] else {return};
      let String(ref subkey) = packet.payload[3] else {return};
      let String(ref oldkey) = packet.payload[4] else {return};
      let String(ref newkey) = packet.payload[4] else {return};

      let hive = get_hive(hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} Server [Management Plugin]> Rename key in {hive}{}{subkey}: {oldkey} -> {newkey}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Delete key
    Byte(3) => {
      let Int(hive) = packet.payload[2] else {return};
      let String(ref subkey) = packet.payload[3] else {return};

      let hive = get_hive(hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} Server [Management Plugin]> Delete key: {hive}{}{subkey}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Get values
    Byte(4) => {
      let Int(hive) = packet.payload[2] else {return};
      let String(ref subkey) = packet.payload[3] else {return};

      let hive = get_hive(hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} Server [Management Plugin]> Get values: {hive}{}{subkey}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Create or change value
    Byte(5) => {
      let Int(hive) = packet.payload[2] else {return};
      let String(ref subkey) = packet.payload[3] else {return};
      let String(ref name) = packet.payload[4] else {return};
      let String(ref value) = packet.payload[5] else {return};

      let hive = get_hive(hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} Server [Management Plugin]> Update value in {hive}{}{subkey}: {name} = {value:?}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Rename value
    Byte(6) => {
      let Int(hive) = packet.payload[2] else {return};
      let String(ref subkey) = packet.payload[3] else {return};
      let String(ref oldname) = packet.payload[4] else {return};
      let String(ref newname) = packet.payload[4] else {return};

      let hive = get_hive(hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} Server [Management Plugin]> Rename value in {hive}{}{subkey}: {oldname} -> {newname}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Delete value
    Byte(7) => {
      let Int(hive) = packet.payload[2] else {return};
      let String(ref subkey) = packet.payload[3] else {return};
      let String(ref name) = packet.payload[4] else {return};

      let hive = get_hive(hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} Server [Management Plugin]> Delete value in {hive}{}{subkey}: {name}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    _ => {}
  }
}

fn get_hive(hive: i32) -> &'static str {
  match hive {
    -0x7FFFFFFE => "HKEY_LOCAL_MACHINE",
    -0x7FFFFFFF => "HKEY_CURRENT_USER",
    -0x80000000 => "HKEY_CLASSES_ROOT",
    -0x7FFFFFFD => "HKEY_USERS",
    -0x7FFFFFFB => "HKEY_CURRENT_CONFIG",
    _ => "UNKNOWN",
  }
}
