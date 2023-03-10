use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 197777867899540155910458662952782250294;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  let Some(Byte(command)) = packet.payload.get(0) else {return};

  match *command {
    // Machine Name
    0 => {}

    // Drives
    1 => {}

    // Files
    2 => {
      if side.is_client() {
        return;
      }

      let Some(path) = state.get_value("nanobrowser-path") else {return};
      println!("#{} [NanoBrowser]> Reading directory: {path}", state.id());
    }

    // Get Current Directory
    3 => {
      if side.is_server() {
        return;
      }

      let Some(String(path)) = packet.payload.get(1) else {return};
      state.set_value("nanobrowser-path", path);
    }

    // Set Current Directory
    4 => {
      if side.is_client() {
        return;
      }

      let Some(String(path)) = packet.payload.get(1) else {return};
      state.set_value("nanobrowser-path", path);
    }

    // Download
    5 => {
      if side.is_client() {
        return;
      }

      let Some(String(local_path)) = packet.payload.get(1) else {return};
      let Some(String(remote_path)) = packet.payload.get(2) else {return};

      println!(
        "#{} [NanoBrowser]> Downloading file: {local_path} -> {remote_path}",
        state.id()
      );
    }

    // Upload
    6 => {
      if side.is_client() {
        return;
      }

      let Some(String(path)) = packet.payload.get(2) else {return};

      println!("#{} [NanoBrowser]> Uploading file: {path}", state.id());
    }

    // Open
    7 => {
      if side.is_client() {
        return;
      }

      let Some(String(path)) = packet.payload.get(2) else {return};
      println!("#{} [NanoBrowser]> Open file: {path}", state.id());
    }

    // Delete
    8 => {
      if side.is_client() {
        return;
      }

      let Some(String(path)) = packet.payload.get(1) else {return};
      println!("#{} [NanoBrowser]> Delete path: {path}", state.id());
    }

    // Create Directory
    9 => {
      if side.is_client() {
        return;
      }

      let Some(String(path)) = packet.payload.get(1) else {return};
      println!("#{} [NanoBrowser]> Create directory: {path}", state.id());
    }

    // Rename
    10 => {
      if side.is_client() {
        return;
      }

      let Some(String(old_path)) = packet.payload.get(1) else {return};
      let Some(String(new_path)) = packet.payload.get(2) else {return};
      let Some(Boolean(directory)) = packet.payload.get(3) else {return};

      println!(
        "#{} [NanoBrowser]> Renaming {}: {old_path} -> {new_path}",
        state.id(),
        if *directory { "directory" } else { "file" }
      );
    }

    _ => {}
  }
}
