use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 282162044448697171732495211913591752041;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  let Some(Byte(command)) = packet.payload.get(0) else {return};

  match *command {
    0 => {
      let Some(Byte(subcommand)) = packet.payload.get(1) else {return};
      match *subcommand {
        0 => {
          if side.is_server() {
            return;
          }

          let Some(String(os_name)) = packet.payload.get(3) else {return};
          let Some(String(filename)) = packet.payload.get(7) else {return};

          println!(
            "#{} [Core Plugin]> Hello server, OS: {os_name}, Filename: {filename}",
            state.id()
          )
        }
        _ => {}
      }
    }
    1 => {
      if side.is_client() {
        return;
      }

      match packet.payload[1] {
        Byte(0) => {
          println!("#{} [Core Plugin]> Restart connection", state.id());
        }

        Byte(1) => {
          println!("#{} [Core Plugin]> Shutdown connection", state.id());
        }

        Byte(2) => {
          println!("#{} [Core Plugin]> Uninstall client", state.id());
        }

        _ => {}
      }
    }
    2 => {
      if side.is_client() {
        return;
      }

      match packet.payload[1] {
        Byte(0) => {
          println!("#{} [Core Plugin]> Restart system", state.id());
        }

        Byte(1) => {
          println!("#{} [Core Plugin]> Shutdown system", state.id());
        }

        _ => {}
      }
    }
    _ => {}
  }
}
