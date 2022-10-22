use crate::{
    ice40::arch_defs::{ArchNetInfo, WireId},
    kernel::{
        delay::DelayTrait, id_string::IdString, port::PortRef, property::Property, region::Region,
        timing::ClockConstraint, types::PipMap,
    },
};
use derive_more::{From, Into};
use std::collections::BTreeMap;
use thunderdome::{Arena, Index};
use typed_index_collections::TiVec;

#[derive(Clone, Copy, Debug, From, Into, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UserId(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetInfo<D>
where
    D: DelayTrait,
{
    arch_net_info: ArchNetInfo,
    name: IdString,
    hierarchy_path: IdString,
    udata: i32,

    pub driver: PortRef<D>,
    // TODO: Measure performance/check implementation of TiVEc, see if its better than O(n) on entry deletion.
    pub users: TiVec<UserId, PortRef<D>>,
    attrs: BTreeMap<IdString, Property>,

    // wire -> uphill_pip
    wires: BTreeMap<WireId, PipMap>,

    aliases: Vec<IdString>, // entries in net_aliases that point to this net

    clk_constr: Option<Index<ClockConstraint<D>>>,
    region: Option<Index<Region>>,
}

impl<D> NetInfo<D>
where
    D: DelayTrait,
{
    pub fn new() -> Self {
        Self {
            arch_net_info: ArchNetInfo::new(),
            name: IdString::new(),
            hierarchy_path: IdString::new(),
            udata: 0,
            driver: PortRef::new(),
            // TODO: Go upstream and get this made const.
            users: TiVec::new(),
            attrs: BTreeMap::new(),
            wires: BTreeMap::new(),
            aliases: Vec::new(),
            clk_constr: None,
            region: None,
        }
    }
    pub fn with_arena(
        region_arena: &mut Arena<Region, Region>,
        clk_constr_arena: &mut Arena<ClockConstraint<D>, ClockConstraint<D>>,
    ) -> Self {
        Self {
            clk_constr: Some(clk_constr_arena.insert(ClockConstraint::new())),
            region: Some(region_arena.insert(Region::new())),
            ..Default::default()
        }
    }
    pub fn with_name(name: IdString) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

impl<D> Default for NetInfo<D>
where
    D: DelayTrait,
{
    fn default() -> Self {
        Self::new()
    }
}
