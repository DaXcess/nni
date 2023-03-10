use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 302677786815878784857885367202273959953;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  let Some(Byte(command)) = packet.payload.get(0) else {return};

  match *command {
    // Misc Command
    0 => {
      let Some(Byte(subcommand)) = packet.payload.get(1) else {return};

      match *subcommand {
        // Script
        0 => {
          let Some(String(expression)) = packet.payload.get(2) else {return};
          let split = expression.split("|").collect::<Vec<_>>();
          if split.len() < 1 {
            return;
          }

          let script_type = split[0];

          if script_type == "REM" {
            println!("#{} [MultiCore]> Remove all scripts", state.id());
            return;
          }

          if split.len() < 2 {
            return;
          }

          let script_data = split[1];

          println!(
            "#{} [MultiCore]> Run script: [{script_type}]\n{script_data}",
            state.id()
          );
        }

        // Recover
        1 => {
          println!("#{} [MultiCore]> Recover system credentials", state.id());
        }

        // Manage
        3 => {
          let Some(String(directory)) = packet.payload.get(2) else {return};

          match directory.as_str() {
            "GetDrives" => {
              println!("#{} [MultiCore]> Get drives", state.id());
            }
            _ => {
              println!("#{} [MultiCore]> Read directory: {directory}", state.id());
            }
          }
        }

        // Upload
        4 => {
          let Some(String(file)) = packet.payload.get(2) else {return};

          println!("#{} [MultiCore]> Download file: {file}", state.id());
        }

        _ => {}
      }
    }

    // Stress Command
    1 => {
      let Some(Byte(stress_type)) = packet.payload.get(1) else {return};
      let Some(String(expression)) = packet.payload.get(2) else {return};

      let stress_type = match *stress_type {
        0 => "SLOW_LORRIS",
        _ => "UNKNOWN",
      };

      let chunk = expression.split("|").collect::<Vec<_>>();
      if chunk.len() < 4 {
        return;
      }

      let host = chunk[0];
      let threads = chunk[1];
      let time = chunk[2];
      let data = chunk[3];

      println!(
        "#{} [MultiCore]> Start stress test: [{stress_type}] Host: {host}, Threads: {threads}, Time: {time}, Data: {data}",
        state.id()
      );
    }

    _ => {}
  }
}
