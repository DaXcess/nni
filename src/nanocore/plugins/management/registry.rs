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
    // Get keys
    0 => {
      let Some(Int(hive)) = packet.payload.get(2) else {return};
      let Some(String(subkey)) = packet.payload.get(3) else {return};

      let hive = get_hive(*hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} [Management Plugin]> Get subkeys: {hive}{}{subkey}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Create key
    1 => {
      let Some(Int(hive)) = packet.payload.get(2) else {return};
      let Some(String(subkey)) = packet.payload.get(3) else {return};
      let Some(String(key)) = packet.payload.get(4) else {return};

      let hive = get_hive(*hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} [Management Plugin]> Create new key in {hive}{}{subkey}: {key}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Rename key
    2 => {
      let Some(Int(hive)) = packet.payload.get(2) else {return};
      let Some(String(subkey)) = packet.payload.get(3) else {return};
      let Some(String(oldkey)) = packet.payload.get(4) else {return};
      let Some(String(newkey)) = packet.payload.get(5) else {return};

      let hive = get_hive(*hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} [Management Plugin]> Rename key in {hive}{}{subkey}: {oldkey} -> {newkey}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Delete key
    3 => {
      let Some(Int(hive)) = packet.payload.get(2) else {return};
      let Some(String(subkey)) = packet.payload.get(3) else {return};

      let hive = get_hive(*hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} [Management Plugin]> Delete key: {hive}{}{subkey}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Get values
    4 => {
      let Some(Int(hive)) = packet.payload.get(2) else {return};
      let Some(String(subkey)) = packet.payload.get(3) else {return};

      let hive = get_hive(*hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} [Management Plugin]> Get values: {hive}{}{subkey}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Create or change value
    5 => {
      let Some(Int(hive)) = packet.payload.get(2) else {return};
      let Some(String(subkey)) = packet.payload.get(3) else {return};
      let Some(String(name)) = packet.payload.get(4) else {return};
      let Some(String(value)) = packet.payload.get(5) else {return};

      let hive = get_hive(*hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} [Management Plugin]> Update value in {hive}{}{subkey}: {name} = {value:?}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Rename value
    6 => {
      let Some(Int(hive)) = packet.payload.get(2) else {return};
      let Some(String(subkey)) = packet.payload.get(3) else {return};
      let Some(String(oldname)) = packet.payload.get(4) else {return};
      let Some(String(newname)) = packet.payload.get(5) else {return};

      let hive = get_hive(*hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} [Management Plugin]> Rename value in {hive}{}{subkey}: {oldname} -> {newname}",
        if subkey.is_empty() { "" } else { "\\" }
      );
    }

    // Delete value
    7 => {
      let Some(Int(hive)) = packet.payload.get(2) else {return};
      let Some(String(subkey)) = packet.payload.get(3) else {return};
      let Some(String(name)) = packet.payload.get(4) else {return};

      let hive = get_hive(*hive);
      let subkey = if let Some(idx) = subkey.find('\\') {
        &subkey[idx + 1..]
      } else {
        ""
      };

      println!(
        "#{id} [Management Plugin]> Delete value in {hive}{}{subkey}: {name}",
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
