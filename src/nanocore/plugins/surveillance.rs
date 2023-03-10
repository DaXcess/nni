use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 256455484641607976164979785359717874683;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  let Some(Byte(command)) = packet.payload.get(0) else {return};
  let Some(Byte(subcommand)) = packet.payload.get(1) else {return};

  match *command {
    // Video Command
    0 => {
      match *subcommand {
        // Get Sources
        0 => {
          println!("#{} [Surveillance Plugin]> Get video sources", state.id());
        }

        // Start Feed
        1 => {
          let Some(pipe_name) = state.pipe_name() else {return};

          println!(
            "#{} [Surveillance Plugin]> Video feed {} started",
            state.id(),
            &pipe_name[3..]
          );
        }

        _ => {}
      }
    }

    // Audio Command
    1 => match *subcommand {
      // Get Sources
      0 => {
        println!("#{} [Surveillance Plugin]> Get audio sources", state.id());
      }

      // Start Feed
      1 => {
        let Some(pipe_name) = state.pipe_name() else {return};

        println!(
          "#{} [Surveillance Plugin]> Audio feed {} started",
          state.id(),
          &pipe_name[3..]
        );
      }

      // Stop Feed
      3 => {
        println!("#{} [Surveillance Plugin]> Audio feed stopped", state.id());
      }

      _ => {}
    },

    // Control Command
    2 => match *subcommand {
      // Get Clipboard
      5 => {
        println!("#{} [Surveillance Plugin]> Get clipboard data", state.id());
      }

      // Set Clipboard
      6 => {
        let Some(String(clipboard)) = packet.payload.get(2) else {return};

        println!(
          "#{} [Surveillance Plugin]> Clipboard data received: {clipboard}",
          state.id()
        );
      }

      // Start Task Manager
      7 => {
        println!(
          "#{} [Surveillance Plugin]> Start Windows Task Manager",
          state.id()
        );
      }

      _ => {}
    },

    _ => {}
  }
}

pub fn connection_closed(state: &NanoState) {
  let Some(ref pipe) = state.pipe else {return};

  if pipe.pipe_name.starts_with("VF_") {
    println!(
      "#{} [Surveillance Plugin]> Video feed {} stopped",
      state.id(),
      &pipe.pipe_name[3..]
    )
  }

  if pipe.pipe_name.starts_with("AF_") {
    println!(
      "#{} [Surveillance Plugin]> Audio feed {} stopped",
      state.id(),
      &pipe.pipe_name[3..]
    )
  }
}
