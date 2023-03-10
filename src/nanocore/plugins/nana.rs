use uuid::Uuid;

use crate::{
  decrypt::{
    Packet,
    Payload::{Boolean, Byte, Int, String as PString},
  },
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 248715992346340948431960248413070906761;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  let Some(Byte(command)) = packet.payload.get(0) else {return};

  match *command {
    // Message Box
    3 => {
      println!("#{} [NanoNana]> Showing Message Box", state.id());
    }

    // Open CD Tray
    4 => {
      println!("#{} [NanoNana]> Opening CD Tray", state.id());
    }

    // Close CD Tray
    5 => {
      println!("#{} [NanoNana]> Closing CD Tray", state.id());
    }

    // Beep
    6 => {
      println!("#{} [NanoNana]> Beeping", state.id());
    }

    // Open Process
    7 => {
      let Some(PString(process)) = packet.payload.get(1) else {return};
      let Some(Int(count)) = packet.payload.get(2) else {return};
      let Some(Int(delay)) = packet.payload.get(3) else {return};

      println!(
        "#{} [NanoNana]> Starting process: {process}, {count} time{}, {delay}ms delay",
        state.id(),
        if *count == 1 { "" } else { "s" }
      );
    }

    // Swap Mouse Buttons
    8 => {
      let Some(Int(swap)) = packet.payload.get(1) else {return};

      if *swap == 1 {
        println!("#{} [NanoNana]> Swapping mouse buttons", state.id());
      } else {
        println!("#{} [NanoNana]> Reverting mouse buttons", state.id());
      }
    }

    // Monitor Power
    9 => {
      let Some(Int(on)) = packet.payload.get(1) else {return};

      if *on == 1 {
        println!("#{} [NanoNana]> Power on monitor", state.id());
      } else {
        println!("#{} [NanoNana]> Power off monitor", state.id());
      }
    }

    // Enable Locker
    17 => {
      let Some(PString(password)) = packet.payload.get(1) else {return};
      let Some(PString(message)) = packet.payload.get(2) else {return};
      let Some(Boolean(browser)) = packet.payload.get(3) else {return};

      println!("#{} [NanoNana]> Locking PC", state.id());
      println!(
        "#{} [NanoNana]> Lock password:           {password}",
        state.id()
      );
      println!(
        "#{} [NanoNana]> Lock message:            {message}",
        state.id()
      );
      println!(
        "#{} [NanoNana]> Lock has Web Browser:    {browser}",
        state.id()
      );
      if *browser {
        let Some(PString(url)) = packet.payload.get(4) else {return};

        println!("#{} [NanoNana]> Browser URL:             {url}", state.id());
      }
    }

    // Disable Locker
    18 => {
      println!("#{} [NanoNana]> Disable locker", state.id());
    }

    _ => {}
  }
}

pub fn pipe_created(state: &mut NanoState, name: String, _uuid: Uuid) {
  match name.as_str() {
    "TeamspeakIdentityRecovery" => {
      println!("#{} [NanoNana]> Request Teamspeak ID recovery", state.id());
    }

    "FileZillaServerRecovery" => {
      println!(
        "#{} [NanoNana]> Request FileZilla server recovery",
        state.id()
      );
    }

    "SteamFileRecovery" => {
      println!("#{} [NanoNana]> Request Steam file recovery", state.id());
    }

    "StartupManager" => {
      println!("#{} [NanoNana]> Request startup entries", state.id());
    }

    "ClientThumbnails" => {
      println!("#{} [NanoNana]> Sending client thumbnail", state.id());
    }

    _ => {}
  }
}
