use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 48193860592937645780711544265579805071;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  let Some(Byte(command)) = packet.payload.get(0) else {return};
  let Some(Byte(subcommand)) = packet.payload.get(1) else {return};

  match *command {
    // Password
    0 => {
      match *subcommand {
        // Steal
        0 => {
          println!("#{} [SurveillanceEx Plugin]> Steal passwords", state.id());
        }

        _ => {}
      }
    }

    // Logging
    1 => {
      match *subcommand {
        // Background Keylogger
        0 => {
          let Some(Boolean(enabled)) = packet.payload.get(2) else {return};

          println!(
            "#{} [SurveillanceEx Plugin]> Background keylogger {}",
            state.id(),
            if *enabled { "enabled" } else { "disabled" }
          );
        }

        // Get Log List
        3 => {
          println!(
            "#{} [SurveillanceEx Plugin]> Get keylogger log list",
            state.id()
          );
        }

        // Delete logs
        4 => {
          println!(
            "#{} [SurveillanceEx Plugin]> Delete keylogger file",
            state.id()
          );
        }

        // Download logs
        5 => {
          println!(
            "#{} [SurveillanceEx Plugin]> Download keylogger data",
            state.id()
          );
        }

        _ => {}
      }
    }

    // Keyboard
    2 => {
      match *subcommand {
        // Live keylogger
        2 => {
          let Some(Boolean(enabled)) = packet.payload.get(2) else {return};

          println!(
            "#{} [SurveillanceEx Plugin]> Live keylogger {}",
            state.id(),
            if *enabled { "enabled" } else { "disabled" }
          );
        }

        _ => {}
      }
    }

    // Dns
    3 => {
      match *subcommand {
        // Get DNS List
        0 => {
          println!("#{} [SurveillanceEx Plugin]> Get DNS list", state.id());
        }

        _ => {}
      }
    }

    _ => {}
  }
}
