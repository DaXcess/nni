use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 175210694936335182642873981654312140919;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  let Some(Byte(command)) = packet.payload.get(0) else {return};
  let Some(Byte(subcommand)) = packet.payload.get(1) else {return};

  match *command {
    // Tool Command
    0 => {
      if side.is_client() {
        return;
      }

      match *subcommand {
        // Execute Web File
        0 => {
          let Some(String(url)) = packet.payload.get(2) else {return};

          println!("#{} [Tools Plugin]> Execute web file: {url}", state.id());
        }

        // Execute Local File
        1 => {
          let Some(String(filename)) = packet.payload.get(2) else {return};

          println!(
            "#{} [Tools Plugin]> Execute uploaded file: {filename}",
            state.id()
          );
        }

        // Update Web File
        2 => {
          let Some(String(url)) = packet.payload.get(2) else {return};

          println!(
            "#{} [Tools Plugin]> Update client using web file: {url}",
            state.id()
          );
        }

        // Update Local File
        3 => {
          println!(
            "#{} [Tools Plugin]> Update client using uploaded file",
            state.id()
          );
        }

        // Clear Memory
        4 => {
          println!("#{} [Tools Plugin]> Clear memory", state.id());
        }

        // Clear Processes
        5 => {
          println!("#{} [Tools Plugin]> Clear processes", state.id());
        }

        // Request Elevation
        6 => {
          println!("#{} [Tools Plugin]> Requested elevation", state.id());
        }

        _ => {}
      }
    }

    // Chat Command
    1 => {
      match *subcommand {
        // Message
        0 => {
          let Some(String(message)) = packet.payload.get(2) else {return};

          println!(
            "#{} {side} [Tools Plugin]> Send chat message: {message}",
            state.id()
          );
        }

        // Shutdown
        1 => {
          println!("#{} [Tools Plugin]> Close chat session", state.id());
        }

        // Update Options
        2 => {
          if side.is_client() {
            return;
          }

          let Some(Boolean(can_close)) = packet.payload.get(2) else {return};

          println!(
            "#{} [Tools Plugin]> Update chat options: CanClose={can_close}",
            state.id()
          );
        }

        // Update Status (don't care about this)
        3 => {}

        _ => {}
      }
    }

    _ => {}
  }
}
