mod decrypt;
mod nanocore;

use std::{collections::HashMap, rc::Rc};

use nanocore::{plugins::PluginManager, NanoCoreState, PipeTracker, Side};
use pnet::{
  datalink::{Channel, Config},
  packet::{
    ethernet::EthernetPacket,
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    tcp::{TcpFlags, TcpPacket},
    Packet,
  },
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
  #[structopt(short, long)]
  interface: String,
}

enum IpPacket<'p> {
  V4(Ipv4Packet<'p>),
  V6(Ipv6Packet<'p>),
}

impl<'p> IpPacket<'p> {
  fn get_source(&self) -> String {
    match self {
      IpPacket::V4(ip) => ip.get_source().to_string(),
      IpPacket::V6(ip) => ip.get_source().to_string(),
    }
  }

  fn get_destination(&self) -> String {
    match self {
      IpPacket::V4(ip) => ip.get_destination().to_string(),
      IpPacket::V6(ip) => ip.get_destination().to_string(),
    }
  }

  fn payload(&'p self) -> &'p [u8] {
    match self {
      IpPacket::V4(ip) => ip.payload(),
      IpPacket::V6(ip) => ip.payload(),
    }
  }
}

struct TcpStateManager {
  states: Vec<TcpState>,
  pipe_tracker: PipeTracker,
  plugin_manager: Rc<PluginManager>,
  current_index: u32,
}

enum Direction {
  S2C,
  C2S,
}

impl TcpStateManager {
  pub fn new() -> Self {
    Self {
      states: vec![],
      pipe_tracker: PipeTracker::new(),
      plugin_manager: Rc::new(PluginManager::new()),
      current_index: 0,
    }
  }

  pub fn process_packet(&mut self, tcp: &TcpPacket, ip: &IpPacket) {
    if tcp.get_flags() & TcpFlags::RST == TcpFlags::RST
      || tcp.get_flags() & TcpFlags::FIN == TcpFlags::FIN
    {
      self.states.retain(|state| {
        let result = (state.client_ip == ip.get_destination()
          && state.client_port == tcp.get_destination()
          && state.server_ip == ip.get_source()
          && state.server_port == tcp.get_source())
          || (state.client_ip == ip.get_source()
            && state.client_port == tcp.get_source()
            && state.server_ip == ip.get_destination()
            && state.server_port == tcp.get_destination());

        if result {
          // println!(
          //   "Connection closed: {}:{} -> {}:{}",
          //   state.client_ip, state.client_port, state.server_ip, state.server_port
          // );

          state.nano_state.connection_closed();
        }

        !result
      });

      return;
    }

    if tcp.get_flags() == TcpFlags::SYN {
      self.states.push(TcpState {
        client_ip: ip.get_source(),
        client_port: tcp.get_source(),
        client_seq: tcp.get_sequence() + 1,
        client_packets: HashMap::new(),

        server_ip: ip.get_destination(),
        server_port: tcp.get_destination(),
        server_seq: 0,
        server_packets: HashMap::new(),

        state: 0,
        nano_state: NanoCoreState::new(
          self.current_index,
          ip.get_destination(),
          tcp.get_destination(),
          self.pipe_tracker.clone(),
          self.plugin_manager.clone(),
        ),
      });
      self.current_index += 1;

      return;
    }

    if tcp.get_flags() == TcpFlags::SYN | TcpFlags::ACK {
      if let Some(state) = self.states.iter_mut().find(|state| {
        state.client_ip == ip.get_destination()
          && state.client_port == tcp.get_destination()
          && state.server_ip == ip.get_source()
          && state.server_port == tcp.get_source()
      }) {
        state.server_seq = tcp.get_sequence() + 1;
        state.state = 1;
      }

      return;
    }

    if tcp.get_flags() & TcpFlags::ACK == TcpFlags::ACK {
      if let Some(state) = self.states.iter_mut().find(|state| {
        state.server_ip == ip.get_destination()
          && state.server_port == tcp.get_destination()
          && state.client_ip == ip.get_source()
          && state.client_port == tcp.get_source()
          && state.state == 1
      }) {
        if state.client_seq != tcp.get_sequence() {
          eprintln!(
            "Client seq mismatch: {} != {}",
            state.client_seq,
            tcp.get_sequence()
          );
          return;
        }

        state.state = 2;
      }
    }

    let (state, direction) = match self.states.iter_mut().find(|state| {
      state.client_ip == ip.get_destination()
        && state.client_port == tcp.get_destination()
        && state.server_ip == ip.get_source()
        && state.server_port == tcp.get_source()
    }) {
      Some(state) => (state, Direction::S2C),
      None => {
        match self.states.iter_mut().find(|state| {
          state.server_ip == ip.get_destination()
            && state.server_port == tcp.get_destination()
            && state.client_ip == ip.get_source()
            && state.client_port == tcp.get_source()
        }) {
          Some(state) => (state, Direction::C2S),
          None => return,
        }
      }
    };

    // Insert packet into stored packets
    match direction {
      Direction::S2C => {
        state
          .server_packets
          .insert(tcp.get_sequence(), tcp.payload().to_vec());
      }

      Direction::C2S => {
        state
          .client_packets
          .insert(tcp.get_sequence(), tcp.payload().to_vec());
      }
    }

    // Process all packets in order, up to the current sequence number
    loop {
      match direction {
        Direction::S2C => {
          let Some(payload) = state.server_packets.remove(&state.server_seq) else { break };

          if state
            .nano_state
            .process_packet(&payload, Side::Server)
            .is_err()
          {
            let id = state.nano_state.id();
            drop(state);

            self
              .states
              .retain(|vec_state| vec_state.nano_state.id() != id);

            break;
          }

          state.server_seq += payload.len() as u32;
        }
        Direction::C2S => {
          let Some(payload) = state.client_packets.remove(&state.client_seq) else { break };

          if state
            .nano_state
            .process_packet(&payload, Side::Client)
            .is_err()
          {
            let id = state.nano_state.id();
            drop(state);

            self
              .states
              .retain(|vec_state| vec_state.nano_state.id() != id);

            break;
          }
          state.client_seq += payload.len() as u32;
        }
      };
    }
  }
}

struct TcpState {
  client_ip: String,
  client_port: u16,
  client_seq: u32,
  client_packets: HashMap<u32, Vec<u8>>,

  server_ip: String,
  server_port: u16,
  server_seq: u32,
  server_packets: HashMap<u32, Vec<u8>>,

  state: u8,
  nano_state: NanoCoreState,
}

fn main() {
  let opts = Opts::from_args();
  let Some(interface) = pnet::datalink::interfaces()
    .into_iter()
    .find(|iface| iface.name == opts.interface) else {
      println!("Interface '{}' not found", opts.interface);
      return;
    };

  let mut tcp_state = TcpStateManager::new();
  let mut rx = match pnet::datalink::channel(
    &interface,
    Config {
      read_buffer_size: 1024 * 1024,
      ..Default::default()
    },
  )
  .unwrap()
  {
    Channel::Ethernet(_, rx) => rx,
    _ => panic!("Unhandled channel type"),
  };

  loop {
    let packet = rx.next().unwrap();

    // Get TCP packet
    let Some(eth) = EthernetPacket::new(&packet) else {continue};
    let ip = match Ipv4Packet::new(eth.payload()) {
      Some(ip) => IpPacket::V4(ip),
      None => match Ipv6Packet::new(eth.payload()) {
        Some(ip) => IpPacket::V6(ip),
        None => continue,
      },
    };
    let Some(tcp) = TcpPacket::new(ip.payload()) else {continue};

    #[cfg(debug_assertions)]
    if tcp.get_destination() != 54984 && tcp.get_source() != 54984 {
      continue;
    };

    tcp_state.process_packet(&tcp, &ip);
  }
}
