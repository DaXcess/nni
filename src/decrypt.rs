use binary_reader::{BinaryReader, Endian};
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use des::cipher::block_padding::Pkcs7;
use flate2::write::DeflateDecoder;
use std::{error::Error, fmt::Display, io::Write};
use uuid::Uuid;

type DesCbcDec = cbc::Decryptor<des::Des>;

#[derive(Debug)]
pub struct Packet {
  pub command_id: u8,
  pub subcommand_id: u8,
  pub plugin_guid: Option<Uuid>,
  pub payload: Vec<Payload>,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Payload {
  Boolean(bool) = 0,
  Byte(u8) = 1,
  ByteArray(Vec<u8>) = 2,
  Char(char) = 3,
  CharArray(Vec<char>) = 4,
  Double(f64) = 6,
  Int(i32) = 7,
  Long(i64) = 8,
  SByte(i8) = 9,
  Short(i16) = 10,
  Float(f32) = 11,
  String(String) = 12,
  UInt(u32) = 13,
  ULong(u64) = 14,
  UShort(u16) = 15,
  DateTime(i64) = 16,
  StringArray(Vec<String>) = 17,
  Guid(Uuid) = 18,
  Unknown,
}

#[allow(dead_code)]
impl Payload {
  pub fn as_bool(self) -> Option<bool> {
    match self {
      Payload::Boolean(b) => Some(b),
      _ => None,
    }
  }

  pub fn as_u8(self) -> Option<u8> {
    match self {
      Payload::Byte(b) => Some(b),
      _ => None,
    }
  }

  pub fn as_u8_vec(self) -> Option<Vec<u8>> {
    match self {
      Payload::ByteArray(b) => Some(b),
      _ => None,
    }
  }

  pub fn as_char(self) -> Option<char> {
    match self {
      Payload::Char(c) => Some(c),
      _ => None,
    }
  }

  pub fn as_char_vec(self) -> Option<Vec<char>> {
    match self {
      Payload::CharArray(c) => Some(c),
      _ => None,
    }
  }

  pub fn as_f64(self) -> Option<f64> {
    match self {
      Payload::Double(d) => Some(d),
      _ => None,
    }
  }

  pub fn as_i32(self) -> Option<i32> {
    match self {
      Payload::Int(i) => Some(i),
      _ => None,
    }
  }

  pub fn as_i64(self) -> Option<i64> {
    match self {
      Payload::Long(l) => Some(l),
      _ => None,
    }
  }

  pub fn as_i8(self) -> Option<i8> {
    match self {
      Payload::SByte(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_i16(self) -> Option<i16> {
    match self {
      Payload::Short(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_f32(self) -> Option<f32> {
    match self {
      Payload::Float(f) => Some(f),
      _ => None,
    }
  }

  pub fn as_string(self) -> Option<String> {
    match self {
      Payload::String(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_u32(self) -> Option<u32> {
    match self {
      Payload::UInt(u) => Some(u),
      _ => None,
    }
  }

  pub fn as_u64(self) -> Option<u64> {
    match self {
      Payload::ULong(u) => Some(u),
      _ => None,
    }
  }

  pub fn as_u16(self) -> Option<u16> {
    match self {
      Payload::UShort(u) => Some(u),
      _ => None,
    }
  }

  pub fn as_i64_datetime(self) -> Option<i64> {
    match self {
      Payload::DateTime(d) => Some(d),
      _ => None,
    }
  }

  pub fn as_string_vec(self) -> Option<Vec<String>> {
    match self {
      Payload::StringArray(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_uuid(self) -> Option<Uuid> {
    match self {
      Payload::Guid(u) => Some(u),
      _ => None,
    }
  }
}

#[allow(dead_code)]
impl Payload {
  pub fn as_ref_bool(&self) -> Option<&bool> {
    match self {
      Payload::Boolean(b) => Some(b),
      _ => None,
    }
  }

  pub fn as_ref_u8(&self) -> Option<&u8> {
    match self {
      Payload::Byte(b) => Some(b),
      _ => None,
    }
  }

  pub fn as_ref_u8_vec(&self) -> Option<&Vec<u8>> {
    match self {
      Payload::ByteArray(b) => Some(b),
      _ => None,
    }
  }

  pub fn as_ref_char(&self) -> Option<&char> {
    match self {
      Payload::Char(c) => Some(c),
      _ => None,
    }
  }

  pub fn as_ref_char_vec(&self) -> Option<&Vec<char>> {
    match self {
      Payload::CharArray(c) => Some(c),
      _ => None,
    }
  }

  pub fn as_ref_f64(&self) -> Option<&f64> {
    match self {
      Payload::Double(d) => Some(d),
      _ => None,
    }
  }

  pub fn as_ref_i32(&self) -> Option<&i32> {
    match self {
      Payload::Int(i) => Some(i),
      _ => None,
    }
  }

  pub fn as_ref_i64(&self) -> Option<&i64> {
    match self {
      Payload::Long(l) => Some(l),
      _ => None,
    }
  }

  pub fn as_ref_i8(&self) -> Option<&i8> {
    match self {
      Payload::SByte(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_ref_i16(&self) -> Option<&i16> {
    match self {
      Payload::Short(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_ref_f32(&self) -> Option<&f32> {
    match self {
      Payload::Float(f) => Some(f),
      _ => None,
    }
  }

  pub fn as_ref_string(&self) -> Option<&String> {
    match self {
      Payload::String(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_ref_u32(&self) -> Option<&u32> {
    match self {
      Payload::UInt(u) => Some(u),
      _ => None,
    }
  }

  pub fn as_ref_u64(&self) -> Option<&u64> {
    match self {
      Payload::ULong(u) => Some(u),
      _ => None,
    }
  }

  pub fn as_ref_u16(&self) -> Option<&u16> {
    match self {
      Payload::UShort(u) => Some(u),
      _ => None,
    }
  }

  pub fn as_ref_i64_datetime(&self) -> Option<&i64> {
    match self {
      Payload::DateTime(d) => Some(d),
      _ => None,
    }
  }

  pub fn as_ref_string_vec(&self) -> Option<&Vec<String>> {
    match self {
      Payload::StringArray(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_ref_uuid(&self) -> Option<&Uuid> {
    match self {
      Payload::Guid(u) => Some(u),
      _ => None,
    }
  }
}

impl Display for Payload {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Payload::Boolean(b) => write!(f, "{}", b),
      Payload::Byte(b) => write!(f, "{}", b),
      Payload::ByteArray(b) => write!(f, "{:?}", b),
      Payload::Char(c) => write!(f, "{}", c),
      Payload::CharArray(c) => write!(f, "{:?}", c),
      Payload::Double(d) => write!(f, "{}", d),
      Payload::Int(i) => write!(f, "{}", i),
      Payload::Long(l) => write!(f, "{}", l),
      Payload::SByte(s) => write!(f, "{}", s),
      Payload::Short(s) => write!(f, "{}", s),
      Payload::Float(_f) => write!(f, "{}", _f),
      Payload::String(s) => write!(f, "{}", s),
      Payload::UInt(u) => write!(f, "{}", u),
      Payload::ULong(u) => write!(f, "{}", u),
      Payload::UShort(u) => write!(f, "{}", u),
      Payload::DateTime(d) => write!(f, "{}", d),
      Payload::StringArray(s) => write!(f, "{:?}", s),
      Payload::Guid(u) => write!(f, "{}", u),
      Payload::Unknown => write!(f, "Unknown"),
    }
  }
}

pub fn decrypt_des_cbc(
  passphrase: &[u8],
  mut ciphertext: &mut [u8],
) -> Result<Packet, Box<dyn Error>> {
  let decrypter = DesCbcDec::new(passphrase.into(), passphrase.into());
  let mut buffer = decrypter
    .decrypt_padded_mut::<Pkcs7>(&mut ciphertext)
    .map_err(|_| "Failed to decrypt")?
    .to_vec();

  let mut reader = BinaryReader::from_vec(&buffer);
  reader.set_endian(Endian::Little);

  if reader.read_bool()? {
    reader.read_i32()?;

    let writer = Vec::new();
    let mut decoder = DeflateDecoder::new(writer);
    decoder.write_all(&buffer[5..])?;
    let new_buffer = decoder.finish()?;

    buffer = new_buffer;
    reader = BinaryReader::from_vec(&buffer);
    reader.set_endian(Endian::Little);
  }

  let command_id = reader.read_u8()?;
  let subcommand_id = reader.read_u8()?;
  let plugin_guid = if reader.read_bool()? {
    Some(Uuid::from_slice_le(reader.read_bytes(16)?)?)
  } else {
    None
  };

  let mut payload: Vec<Payload> = vec![];

  while reader.pos != buffer.len() {
    match reader.read_u8()? {
      0 => payload.push(Payload::Boolean(reader.read_bool()?)),
      1 => payload.push(Payload::Byte(reader.read_u8()?)),
      2 => {
        let len = reader.read_i32()?;
        payload.push(Payload::ByteArray(
          reader.read_bytes(len as usize)?.to_owned(),
        ));
      }
      3 => payload.push(Payload::Char(reader.read_u8()? as char)),
      4 => {
        let len = reader.read_u32()?;
        payload.push(Payload::CharArray(
          reader
            .read_bytes(len as usize)?
            .iter()
            .map(|&c| c as char)
            .collect(),
        ));
      }
      5 => {
        let len = reader.read_i64()?;
        reader.read_bytes(len as usize)?;
        payload.push(Payload::Unknown);
      }
      6 => payload.push(Payload::Double(reader.read_f64()?)),
      7 => payload.push(Payload::Int(reader.read_i32()?)),
      8 => payload.push(Payload::Long(reader.read_i64()?)),
      9 => payload.push(Payload::SByte(reader.read_i8()?)),
      10 => payload.push(Payload::Short(reader.read_i16()?)),
      11 => payload.push(Payload::Float(reader.read_f32()?)),
      12 => {
        let len = read_varint(&mut reader)?;

        // Read string as utf8
        payload.push(Payload::String(String::from_utf8(
          reader.read_bytes(len as usize)?.to_owned(),
        )?));
      }
      13 => payload.push(Payload::UInt(reader.read_u32()?)),
      14 => payload.push(Payload::ULong(reader.read_u64()?)),
      15 => payload.push(Payload::UShort(reader.read_u16()?)),
      16 => payload.push(Payload::DateTime(reader.read_i64()?)),
      17 => {
        let len = reader.read_i32()?;
        let mut strings = vec![];
        for _ in 0..len {
          let len = read_varint(&mut reader)?;
          strings.push(String::from_utf8(
            reader.read_bytes(len as usize)?.to_owned(),
          )?);
        }
        payload.push(Payload::StringArray(strings));
      }
      18 => payload.push(Payload::Guid(Uuid::from_slice_le(reader.read_bytes(16)?)?)),
      19 => {
        reader.read_i64()?;
        payload.push(Payload::Unknown);
      }
      20 => {
        reader.read_bytes(16)?;
        payload.push(Payload::Unknown);
      }
      21 => {
        let len = read_varint(&mut reader)?;
        reader.read_bytes(len as usize)?;
        payload.push(Payload::Unknown);
      }
      _ => continue,
    }
  }

  Ok(Packet {
    command_id,
    subcommand_id,
    plugin_guid,
    payload,
  })
}

fn read_varint(reader: &mut BinaryReader) -> std::io::Result<u64> {
  let mut len = 0u64;
  let mut shift = 0;
  let mut byte = reader.read_u8()?;
  while byte & 0x80 != 0 {
    len |= (byte as u64 & 0x7F) << shift;
    shift += 7;
    byte = reader.read_u8()?;
  }

  len |= (byte as u64) << shift;

  Ok(len)
}
