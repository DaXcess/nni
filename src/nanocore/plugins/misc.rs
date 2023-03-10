use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 277370744057032129978684004403413424461;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  let Some(Byte(command)) = packet.payload.get(0) else {return};
  let id = state.id();

  match *command {
    0 => {
      let Some(String(url)) = packet.payload.get(1) else {return};

      println!("#{id} [Misc Tools]> Open Website: {url}");
    }
    3 => {
      println!("#{id} [Misc Tools]> Open CD Tray");
    }
    4 => {
      println!("#{id} [Misc Tools]> Close CD Tray");
    }
    5 => {
      println!("#{id} [Misc Tools]> Swap mouse buttons");
    }
    6 => {
      println!("#{id} [Misc Tools]> Revert mouse buttons");
    }
    _ => {}
  }
}
