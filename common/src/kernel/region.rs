use crate::ice40::arch_defs::{BelId, WireId};
use crate::kernel::base_types::Loc;
use crate::kernel::id_string::IdString;
use std::collections::BTreeMap;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Region {
    pub name: IdString,

    pub constr_bels: bool,
    constr_wires: bool,
    constr_pips: bool,

    pub bels: BTreeMap<BelId, u64>,
    wires: BTreeMap<WireId, u64>,
    piplocs: BTreeMap<Loc, u64>,
}

impl Region {
    pub fn new() -> Self {
        Self {
            name: IdString::new(),
            constr_bels: false,
            constr_wires: false,
            constr_pips: false,
            bels: BTreeMap::new(),
            wires: BTreeMap::new(),
            piplocs: BTreeMap::new(),
        }
    }
}

impl Default for Region {
    fn default() -> Self {
        Self::new()
    }
}
