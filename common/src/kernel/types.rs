/// Types defined in this header use one or more user defined types (e.g. BelId).
/// If a new common type is desired that doesn't depend on a user defined type,
/// either put it in it's own header, or in nextpnr_base_types.h.
// TODO: Need to figure out the cargo feature based method to
// import the relevant arch definitions.
// TODO: Instead of the above, implement a unified database system that can cleanly represent Xilinx's story.
use crate::ice40::arch_defs::{BelId, DecalId, PipId};
use crate::kernel::base_types::PlaceStrength;
use crate::kernel::id_string::IdString;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, PartialOrd)]
pub struct DecalXY {
    decal: DecalId,
    x: f64,
    y: f64,
}

impl DecalXY {
    pub const fn new() -> Self {
        Self {
            decal: DecalId::new(),
            x: 0.0,
            y: 0.0,
        }
    }
}

impl const PartialEq for DecalXY {
    fn eq(&self, other: &Self) -> bool {
        self.decal == other.decal && self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Copy, Clone, Eq)]
pub struct BelPin {
    bel: BelId,
    pin: IdString,
}

impl Hash for BelPin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bel.hash(state);
        self.pin.hash(state);
    }
}

impl const PartialEq for BelPin {
    fn eq(&self, other: &Self) -> bool {
        self.bel == other.bel && self.pin == other.pin
    }
}

#[derive(Debug, Copy, Clone, Eq)]
pub struct PipMap {
    pip: PipId,
    strength: PlaceStrength,
}

impl Hash for PipMap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pip.hash(state);
        self.strength.hash(state);
    }
}

impl PipMap {
    pub const fn new() -> Self {
        Self {
            pip: PipId::new(),
            strength: PlaceStrength::new(),
        }
    }
}

impl Default for PipMap {
    fn default() -> Self {
        Self::new()
    }
}

impl const PartialEq for PipMap {
    fn eq(&self, other: &Self) -> bool {
        self.pip == other.pip && self.strength == other.strength
    }
}
