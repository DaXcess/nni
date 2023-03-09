use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 282162044448697171732495211913591752041;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  match packet.payload[0] {
    Byte(0) => match packet.payload[1] {
      Byte(0) => {
        if side.is_server() {
          return;
        }

        let os_name = packet.payload[3].as_ref_string().unwrap();
        let filename = packet.payload[7].as_ref_string().unwrap();

        println!(
          "#{} Client [Core Plugin]> Hello server, OS: {os_name}, Filename: {filename}",
          state.id()
        )
      }
      _ => {}
    },
    Byte(1) => {
      if side.is_client() {
        return;
      }

      match packet.payload[1] {
        Byte(0) => {
          println!("#{} Server [Core Plugin]> Restart connection", state.id());
        }

        Byte(1) => {
          println!("#{} Server [Core Plugin]> Shutdown connection", state.id());
        }

        Byte(2) => {
          println!("#{} Server [Core Plugin]> Uninstall client", state.id());
        }

        _ => {}
      }
    }
    Byte(2) => {
      if side.is_client() {
        return;
      }

      match packet.payload[1] {
        Byte(0) => {
          println!("#{} Server [Core Plugin]> Restart system", state.id());
        }

        Byte(1) => {
          println!("#{} Server [Core Plugin]> Shutdown system", state.id());
        }

        _ => {}
      }
    }
    _ => {}
  }
}
