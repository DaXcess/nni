use crate::{
  decrypt::Packet,
  nanocore::{NanoState, Side},
};

pub const PLUGIN_GUID: u128 = 127549838209867769420981366413791782579;

pub fn handle_packet(_: &mut NanoState, _: Packet, _: Side) {
  // Security Plugin has no packets of interest
}
