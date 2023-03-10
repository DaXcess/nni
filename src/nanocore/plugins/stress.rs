use crate::{
  decrypt::{Packet, Payload::*},
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 218203394225714901294119930795099248685;

pub fn handle_packet(state: &mut NanoState, packet: Packet, side: Side) {
  if side.is_client() {
    return;
  }

  let Some(Byte(command)) = packet.payload.get(0) else {return};

  match *command {
    // Start Stress Test
    0 => {
      let Some(Byte(method)) = packet.payload.get(1) else {return};

      match *method {
        // TCP
        0 => {
          let Some(String(address)) = packet.payload.get(2) else {return};
          let Some(String(port)) = packet.payload.get(3) else {return};
          let Some(String(size)) = packet.payload.get(4) else {return};

          state.set_value("nanostress-method", "TCP");

          println!(
            "#{} [NanoCoreStress]> Starting TCP Stress Test: {address}:{port} with {size} bytes",
            state.id()
          );
        }

        // UDP
        1 => {
          let Some(String(address)) = packet.payload.get(2) else {return};
          let Some(String(port)) = packet.payload.get(3) else {return};
          let Some(String(size)) = packet.payload.get(4) else {return};

          state.set_value("nanostress-method", "UDP");

          println!(
            "#{} [NanoCoreStress]> Starting UDP Stress Test: {address}:{port} with {size} bytes",
            state.id()
          );
        }

        // HTTP
        2 => {
          let Some(String(target)) = packet.payload.get(2) else {return};

          state.set_value("nanostress-method", "HTTP");

          println!(
            "#{} [NanoCoreStress]> Starting HTTP Stress Test: {target}",
            state.id()
          );
        }

        // Slow Lorris
        3 => {
          let Some(String(address)) = packet.payload.get(2) else {return};
          let Some(String(port)) = packet.payload.get(3) else {return};
          let Some(String(seconds)) = packet.payload.get(4) else {return};
          let Some(String(threads)) = packet.payload.get(5) else {return};

          state.set_value("nanostress-method", "Slow Lorris");

          println!(
            "#{} [NanoCoreStress]> Starting Slow Lorris Stress Test: {address}:{port} for {seconds} seconds with {threads} threads",
            state.id()
          );
        }

        // SYN
        4 => {
          let Some(String(address)) = packet.payload.get(2) else {return};
          let Some(String(port)) = packet.payload.get(3) else {return};
          let Some(String(threads)) = packet.payload.get(4) else {return};

          state.set_value("nanostress-method", "SYN");

          println!(
          "#{} [NanoCoreStress]> Starting SYN Stress Test: {address}:{port} with {threads} threads",
          state.id()
        );
        }

        _ => {}
      }
    }

    // Stop Stress Test
    2 => {
      let method = state
        .get_value("nanostress-method")
        .unwrap_or_else(|| "Unknown".into());

      println!(
        "#{} [NanoCoreStress]> Stopping {method} Stress Test",
        state.id()
      );
    }

    _ => {}
  }
}
