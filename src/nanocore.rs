pub mod plugins;

use crate::decrypt::{
  decrypt_des_cbc, Packet,
  Payload::{self, Guid},
};
use std::{cmp::min, collections::HashMap, fmt::Display, rc::Rc, sync::Mutex};
use uuid::Uuid;

use self::plugins::PluginManager;

const PASSPHRASE: &[u8] = &[0x72, 0x20, 0x18, 0x78, 0x8c, 0x29, 0x48, 0x97];

#[derive(Clone, Copy)]
pub enum Side {
  Client,
  Server,
}

impl Display for Side {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Side::Client => write!(f, "Client"),
      Side::Server => write!(f, "Server"),
    }
  }
}

#[allow(dead_code)]
impl Side {
  pub fn is_client(&self) -> bool {
    matches!(self, Side::Client)
  }

  pub fn is_server(&self) -> bool {
    matches!(self, Side::Server)
  }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct PipeIdentifier {
  server_ip: String,
  server_port: u16,
  pipe_id: Uuid,
  pipe_name: String,
}

#[derive(Clone)]
pub struct PipeTracker {
  track: Rc<Mutex<HashMap<PipeIdentifier, u32>>>,
}

impl PipeTracker {
  pub fn new() -> Self {
    Self {
      track: Rc::new(Mutex::new(HashMap::new())),
    }
  }

  fn add(&self, pipe: PipeIdentifier, client_id: u32) {
    let mut track = self.track.lock().unwrap();
    track.insert(pipe, client_id);
  }

  fn get(&self, pipe: &PipeIdentifier) -> Option<u32> {
    let mut track = self.track.lock().unwrap();
    track.remove(pipe)
  }
}

pub struct NanoCoreState {
  id: u32,

  server_ip: String,
  server_port: u16,

  hosts: [NanoNetworkState; 2],
  pipe_tracker: PipeTracker,

  plugin_manager: Rc<PluginManager>,

  state: NanoState,
}

#[derive(Debug)]
pub struct NanoState {
  id: u32,
  pipe: Option<PipeIdentifier>,
  map: HashMap<String, String>,
}

impl NanoState {
  pub fn new(id: u32) -> Self {
    Self {
      id,
      pipe: None,
      map: HashMap::new(),
    }
  }

  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn pipe_name(&self) -> Option<String> {
    self.pipe.as_ref().map(|pipe| pipe.pipe_name.clone())
  }

  pub fn get_value(&self, key: &str) -> Option<String> {
    self.map.get(key).map(|s| s.clone())
  }

  pub fn set_value(&mut self, key: impl Into<String>, value: impl Into<String>) {
    self.map.insert(key.into(), value.into());
  }
}

impl NanoCoreState {
  pub fn new(
    id: u32,
    server_ip: String,
    server_port: u16,
    pipe_tracker: PipeTracker,
    plugin_manager: Rc<PluginManager>,
  ) -> Self {
    Self {
      id,

      server_ip,
      server_port,

      hosts: [NanoNetworkState::new(), NanoNetworkState::new()],

      pipe_tracker,
      plugin_manager,

      state: NanoState::new(id),
    }
  }

  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn process_packet(&mut self, packet: &[u8], side: Side) -> Result<(), ()> {
    let host = if matches!(side, Side::Client) {
      &mut self.hosts[0]
    } else {
      &mut self.hosts[1]
    };

    for packet in host.process_packet(packet, 0)? {
      self.handle_packet(packet, side);
    }

    Ok(())
  }

  fn handle_packet(&mut self, packet: Packet, side: Side) {
    match packet.command_id {
      0 => match packet.subcommand_id {
        0 => {
          if side.is_server() {
            return;
          }

          println!(
            "#{} Client -> Hello server, Name: {}, Group: {}, Version: {}, Remote: {}:{}",
            self.id,
            packet.payload[1],
            packet.payload[2],
            packet.payload[3],
            self.server_ip,
            self.server_port
          )
        }
        2 => {
          let Some(Payload::String(pipe_name)) = packet.payload.get(0).cloned() else {return};
          let Some(Guid(pipe_id)) = packet.payload.get(1).cloned() else {return};

          let identifier = PipeIdentifier {
            server_ip: self.server_ip.clone(),
            server_port: self.server_port,
            pipe_id,
            pipe_name: pipe_name.clone(),
          };

          if side.is_client() {
            if let Some(id) = self.pipe_tracker.get(&identifier) {
              self.id = id;
              self.state.id = id;
              self.state.pipe = Some(identifier);
            }

            self
              .plugin_manager
              .handle_pipe_created(&mut self.state, &pipe_name, &pipe_id);
          } else {
            // Need to make sure that once a new client connects it knows it is this client
            self.pipe_tracker.add(identifier, self.id);
          }

          // println!(
          //   "#{} {side} -> Create pipe: {} ({})",
          //   self.id, pipe_name, pipe_id
          // );
        }
        // Plugin command
        4 => {
          self
            .plugin_manager
            .handle_packet(&mut self.state, packet, side);
        }
        _ => {}
      },
      1 => match packet.subcommand_id {
        0 => {
          // Validate client plugin hash
        }
        1 => {
          // Client plugin info / update?
        }
        2 => {
          // Server plugin list
          let count = packet.payload.len() / 3;

          for chunk in packet.payload.chunks(3) {
            if let [Payload::Guid(uuid), _, _] = chunk {
              if let Some(name) = self.plugin_manager.plugin_name(uuid) {
                println!(
                  "#{} {} -> Plugin: {} ({})",
                  self.id,
                  side.to_string(),
                  name,
                  uuid
                );
              } else {
                println!("#{} {} -> Plugin: {}", self.id, side.to_string(), uuid);
              }
            }
          }

          println!("#{} {} -> Plugin count: {count}", self.id, side.to_string());
        }
        3 => {
          // Load plugins
          for chunk in packet.payload.chunks(5) {
            if let [Payload::Guid(uuid), _, Payload::String(name), _, _] = chunk {
              println!(
                "#{} {} -> Load plugin: {} ({})",
                self.id,
                side.to_string(),
                name,
                uuid
              );
            }
          }
        }
        _ => {}
      },
      _ => {}
    }
  }

  pub fn connection_closed(&self) {
    self.plugin_manager.connection_closed(&self.state);
  }
}

const MAX_PACKET_SIZE: i32 = 0;

pub struct NanoNetworkState {
  length_acquired: bool,
  length_bytes_read: usize,
  length_buffer: [u8; 4],

  bytes_read: usize,
  buffer: Vec<u8>,

  /// Whether this is certainly not a NanoCore connection
  is_invalid: bool,
}

impl NanoNetworkState {
  pub fn new() -> Self {
    Self {
      length_acquired: false,
      length_bytes_read: 0,
      length_buffer: [0; 4],

      bytes_read: 0,
      buffer: vec![],

      is_invalid: false,
    }
  }

  pub fn process_packet(&mut self, packet: &[u8], mut offset: usize) -> Result<Vec<Packet>, ()> {
    let mut packets = vec![];

    if self.is_invalid {
      return Ok(packets);
    }

    if !self.length_acquired {
      // We don't know yet how big the packet is going to be
      let count = min(packet.len() - offset, 4 - self.length_bytes_read);
      self.length_buffer[self.length_bytes_read..self.length_bytes_read + count]
        .copy_from_slice(&packet[offset..offset + count]);
      offset += count;

      // Check if we have finished reading the entire length buffer
      self.length_bytes_read += count;
      if self.length_bytes_read != 4 {
        return Ok(packets);
      }

      // We know how big the packet is going to be
      let packet_size = i32::from_le_bytes(self.length_buffer);

      if packet_size < 0 || packet_size > MAX_PACKET_SIZE {
        // This is most certainly not a NanoCore connection
        self.is_invalid = true;
        return Err(());
      }

      self.bytes_read = 0;
      self.length_bytes_read = 0;
      self.length_acquired = true;
      self.buffer = vec![0; packet_size as usize];

      // Check if we have finished reading the entire packet
      if offset >= packet.len() {
        return Ok(packets);
      }

      // Continue processing the packet now that we know the length
      packets.extend(self.process_packet(packet, offset)?);
    } else {
      // Read (a part of) the entire packet
      let count = min(self.buffer.len() - self.bytes_read, packet.len() - offset);
      self.buffer[self.bytes_read..self.bytes_read + count]
        .copy_from_slice(&packet[offset..offset + count]);

      self.bytes_read += count;

      if self.bytes_read == self.buffer.len() {
        // All bytes for this packet have been read

        self.length_acquired = false;

        if let Ok(packet) = decrypt_des_cbc(PASSPHRASE, &mut self.buffer) {
          packets.push(packet);
        }
      }

      // Check if we have finished reading the entire packet
      if count >= packet.len() - offset {
        return Ok(packets);
      }

      // Continue processing the packet
      packets.extend(self.process_packet(packet, offset + count)?);
    }

    Ok(packets)
  }
}
