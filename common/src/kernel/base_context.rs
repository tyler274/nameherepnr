use super::cell::{CellInfo, PseudoCell};
use super::context::Context;
use super::delay::DelayTrait;
use super::net::NetInfo;
use super::port::PortInfo;
use super::region::Region;
use super::timing::TimingResult;
use super::{cell::HierarchicalCell, id_string::IdString, property::Property};
use crate::ice40::arch_defs::{BelId, GroupId, PipId, WireId};
use ringbuf::{LocalRb, StaticRb};
use std::collections::vec_deque::VecDeque;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use thunderdome::{Arena, Index};
//use std::hash::{BuildHasher, BuildHasherDefault};
//use std::hash::{Hash, Hasher};

pub struct BaseCtx<D: DelayTrait> {
    // Lock to perform mutating actions on the Context.
    //    pub mutex: Mutex,

    // Lock to be taken by UI when wanting to access context - the yield()
    // method will lock/unlock it when its' released the main mutex to make
    // sure the UI is not starved.
    //    pub ui_mutex: Mutex,

    // ID String database.
    pub idstring_idx_to_str: Vec<String>,
    pub idstring_str_to_id: BTreeMap<String, isize>,

    // Temporary string backing store for logging
    pub log_strs: StaticRb<String, 100>,

    // Project settings and config switches
    pub settings: BTreeMap<IdString, Property>,

    // Placed nets and cells.
    //    pub nets: BTreeMap<IdString, Index>,
    //    pub cells: BTreeMap<IdString, Index>,
    pub nets: Arena<NetInfo<D>, NetInfo<D>>,
    pub cells: Arena<CellInfo<D>, CellInfo<D>>,
    pub(crate) pseudo_cells: Arena<Box<dyn PseudoCell<D>>>,

    // Hierarchical (non-leaf) cells by full path
    pub hierarchy: BTreeMap<IdString, HierarchicalCell>,
    // This is the root of the above structure
    pub top_module: IdString,

    // Aliases for nets, which may have more than one name due to assignments and hierarchy
    net_aliases: BTreeMap<IdString, IdString>,

    // Top-level ports
    // Index is for PortInfo(s).
    ports: BTreeMap<IdString, Index<PortInfo<D>>>,
    // The Index is for CellInfo(s).
    port_cells: BTreeMap<IdString, Index<CellInfo<D>>>,

    // Floorplanning regions
    // The Index maps to Region(s), unique_ptrs in nextpnr
    //    region: BTreeMap<IdString, Index>,
    region: Arena<Region, Region>,

    // Context meta data
    attributes: BTreeMap<IdString, Property>,

    // Fmax data post timing analysis
    timing_result: TimingResult<D>,

    // The Index here has a type of of Context
    as_context: Option<Index<Context>>,

    // Has the frontend loaded a design?
    design_loaded: bool,

    all_ui_reload: bool,
    frame_ui_reload: bool,
    bel_ui_reload: BTreeMap<usize, BelId>,
    wire_ui_reload: BTreeMap<usize, WireId>,
    pip_ui_reload: BTreeMap<usize, GroupId>,
}

impl<D> BaseCtx<D>
where
    D: DelayTrait,
{
    pub fn new() -> Self {
        Self {
            idstring_idx_to_str: Vec::new(),
            idstring_str_to_id: BTreeMap::new(),
            log_strs: StaticRb::default(),
            settings: BTreeMap::new(),
            nets: Arena::new(),
            cells: Arena::new(),
            pseudo_cells: Arena::new(),
            hierarchy: BTreeMap::new(),
            top_module: IdString::new(),
            net_aliases: BTreeMap::new(),
            ports: BTreeMap::new(),
            port_cells: BTreeMap::new(),
            region: Arena::new(),
            attributes: BTreeMap::new(),
            timing_result: TimingResult::new(),
            as_context: None,
            design_loaded: false,
            all_ui_reload: false,
            frame_ui_reload: false,
            bel_ui_reload: BTreeMap::new(),
            wire_ui_reload: BTreeMap::new(),
            pip_ui_reload: BTreeMap::new(),
        }
    }
}

impl<D> Default for BaseCtx<D>
where
    D: DelayTrait,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct GuardedBaseCtx<D>(Arc<Mutex<BaseCtx<D>>>)
where
    D: DelayTrait;
