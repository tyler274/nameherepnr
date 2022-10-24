use super::cell::{CellInfo, PseudoCell};
use super::context::Context;
use super::delay::DelayTrait;
use super::net::NetInfo;
use super::port::PortInfo;
use super::region::Region;
use super::timing::TimingResult;
use super::{cell::HierarchicalCell, id_string::IdString, property::Property};
use crate::ice40::arch_defs::{BelId, GroupId, WireId};
use core::hash::Hash;
use smallvec::SmallVec;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use thunderdome::{Arena, Index};
use thiserror::Error;

pub struct BaseCtx<D: DelayTrait> {
    // Lock to perform mutating actions on the Context.
    //    pub mutex: Mutex,

    // Lock to be taken by UI when wanting to access context - the yield()
    // method will lock/unlock it when its' released the main mutex to make
    // sure the UI is not starved.
    //    pub ui_mutex: Mutex,

    // ID String database.
    pub idstring_idx_to_str: Vec<String>,
    pub idstring_str_to_idx: BTreeMap<String, u64>,

    // Temporary string backing store for logging
    pub log_strs: SmallVec<[String; 100]>,
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
    as_context: Option<Box<Context>>,

    // Has the frontend loaded a design?
    design_loaded: bool,

    all_ui_reload: bool,
    frame_ui_reload: bool,
    bel_ui_reload: BTreeMap<usize, BelId>,
    wire_ui_reload: BTreeMap<usize, WireId>,
    pip_ui_reload: BTreeMap<usize, GroupId>,
}

impl<D> Hash for BaseCtx<D>
where
    D: DelayTrait,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.idstring_idx_to_str.hash(state);
        self.idstring_str_to_idx.hash(state);
        self.settings.hash(state);
        self.log_strs.hash(state);
        // self.pseudo_cells.hash(state);
        self.nets.hash(state);
        self.cells.hash(state);
        self.hierarchy.hash(state);
        self.top_module.hash(state);
        self.net_aliases.hash(state);
        self.ports.hash(state);
        self.port_cells.hash(state);
        self.region.hash(state);
        self.attributes.hash(state);
        self.timing_result.hash(state);
        self.as_context.hash(state);
        self.design_loaded.hash(state);
        self.all_ui_reload.hash(state);
        self.frame_ui_reload.hash(state);
        self.bel_ui_reload.hash(state);
        self.wire_ui_reload.hash(state);
        self.pip_ui_reload.hash(state);
    }
}

impl<D> PartialEq for BaseCtx<D>
where
    D: DelayTrait,
{
    fn eq(&self, other: &Self) -> bool {
        //        self.log_strs == other.log_strs
        self.idstring_idx_to_str == other.idstring_idx_to_str
            && self.idstring_str_to_idx == other.idstring_str_to_idx
            && self.settings == other.settings
            && self.nets == other.nets
            && self.cells == other.cells
            && self.hierarchy == other.hierarchy
            && self.top_module == other.top_module
            && self.net_aliases == other.net_aliases
            && self.ports == other.ports
            && self.port_cells == other.port_cells
            && self.region == other.region
            && self.attributes == other.attributes
            && self.timing_result == other.timing_result
            && self.as_context == other.as_context
            && self.design_loaded == other.design_loaded
            && self.all_ui_reload == other.all_ui_reload
            && self.frame_ui_reload == other.frame_ui_reload
            && self.bel_ui_reload == other.bel_ui_reload
            && self.wire_ui_reload == other.wire_ui_reload
            && self.pip_ui_reload == other.pip_ui_reload
    }
}

impl<D> BaseCtx<D>
where
    D: DelayTrait,
{
    pub const fn new() -> Self {
        Self {
            idstring_idx_to_str: Vec::new(),
            idstring_str_to_idx: BTreeMap::new(),
            log_strs: SmallVec::new_const(),
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

    pub fn get_context(&self) -> Result<&Context, BaseCtxError> {
        Ok(self.as_context.as_ref().ok_or(BaseCtxError::ContextNotInitialized)?.as_ref())
    }
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum BaseCtxError {
    #[error("Context is not initialized.")]
    ContextNotInitialized,
}

impl<D> const Default for BaseCtx<D>
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
