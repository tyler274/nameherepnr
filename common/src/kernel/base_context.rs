use super::base_types::Loc;
use super::cell::{CellInfo, PseudoCell};
use super::context::Context;
use super::delay::DelayTrait;
use super::net::NetInfo;
use super::port::{PortInfo, PortType};
use super::region::Region;
use super::timing::TimingResult;
use super::types::DecalXY;
use super::{cell::HierarchicalCell, id_string::IdString, property::Property};
use crate::ice40::arch_defs::{BelId, DecalId, GroupId, PipId, WireId};
use core::hash::Hash;
use ordered_float::NotNan;
use smallvec::SmallVec;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use thunderdome::{Arena, Index};

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
    net_aliases: BTreeMap<IdString, Index<NetInfo<D>>>,

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
    bel_ui_reload: Arena<BelId, BelId>,
    wire_ui_reload: Arena<WireId, WireId>,
    pip_ui_reload: Arena<PipId, PipId>,
    group_ui_reload: Arena<GroupId, GroupId>,
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
            all_ui_reload: true,
            frame_ui_reload: false,
            bel_ui_reload: Arena::new(),
            wire_ui_reload: Arena::new(),
            pip_ui_reload: Arena::new(),
            group_ui_reload: Arena::new(),
        }
    }

    pub fn get_context(&self) -> Result<&Context, BaseCtxError> {
        Ok(self
            .as_context
            .as_ref()
            .ok_or(BaseCtxError::ContextNotInitialized)?
            .as_ref())
    }

    pub fn id(&mut self, s: &str) -> IdString {
        IdString::with_ctx_str(self, s)
    }

    pub fn name_of(&self, name: IdString) -> Result<String, BaseCtxError> {
        name.to_string(self).ok().ok_or(BaseCtxError::NameNotFound)
    }

    pub fn name_of_bel(&self, bel: BelId) -> String {
        let context: &Context = self.get_context().unwrap();
        // FIXME: What is the actual relation ship between the context types here.
        //        let s = *context.log_strs;

        todo!()
    }
    pub fn name_of_wire(&self, wire: WireId) -> String {
        todo!()
    }
    pub fn name_of_pip(&self, pip: PipId) -> String {
        todo!()
    }
    pub fn name_of_group(&self, group: GroupId) -> String {
        todo!()
    }
    // TODO: Rework as part of unified database structure rewrite after MVP.
    // Wrappers of arch functions that take a string and handle IdStringList parsing
    pub fn get_bel_by_name_str(&self, name: &str) -> BelId {
        todo!()
    }
    pub fn get_wire_by_name_str(&self, name: &str) -> WireId {
        todo!()
    }
    pub fn get_pip_by_name_str(&self, name: &str) -> PipId {
        todo!()
    }
    pub fn get_group_by_name_str(&self, name: &str) -> GroupId {
        todo!()
    }
    // --------------------------------------------------------------

    pub fn refresh_ui(&mut self) {
        self.all_ui_reload = true;
    }
    pub fn refresh_ui_frame(&mut self) {
        self.frame_ui_reload = true;
    }
    pub fn refresh_ui_bel(&mut self, bel: BelId) {
        self.bel_ui_reload.insert(bel);
    }
    pub fn refresh_ui_wire(&mut self, wire: WireId) {
        self.wire_ui_reload.insert(wire);
    }
    pub fn refresh_ui_pip(&mut self, pip: PipId) {
        self.pip_ui_reload.insert(pip);
    }
    pub fn refresh_ui_group(&mut self, group: GroupId) {
        self.group_ui_reload.insert(group);
    }
    // --------------------------------------------------------------
    pub fn get_net_by_alias(&self, alias: NetAlias<D>) -> Option<&NetInfo<D>> {
        match alias {
            NetAlias::Id(id_str) => {
                let real_idx = self.net_aliases.get(&id_str).unwrap();
                self.nets.get(*real_idx)
            }
            NetAlias::Index(idx) => self.nets.get(idx),
        }
    }

    // Intended to simplify Python API
    pub fn add_clock(&mut self, net: IdString, freq: NotNan<f32>) {
        todo!()
    }
    pub fn create_rectangular_region(
        &mut self,
        name: IdString,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
    ) {
        todo!()
    }
    pub fn add_bel_to_region(&mut self, name: IdString, bel: BelId) {
        todo!()
    }
    pub fn constrain_cell_to_region(&mut self, cell: IdString, region_name: IdString) {
        todo!()
    }

    // Helper functions for the partial reconfiguration plug API using PseudoCells
    pub fn create_region_plug(name: IdString, region_plug_type: IdString, approc_loc: Loc) {
        todo!()
    }
    pub fn add_plug_pin(
        &mut self,
        plug: IdString,
        pin: IdString,
        direction: PortType,
        wire: WireId,
    ) {
        todo!()
    }

    // Helper functions for Python bindings
    pub fn create_net(&mut self, name: IdString) -> Option<&NetInfo<D>> {
        todo!()
    }
    pub fn connect_port(&mut self, net: IdString, cell: IdString, port: IdString) {
        todo!()
    }
    pub fn disconnect_port(&mut self, cell: IdString, port: IdString) {
        todo!()
    }
    pub fn rip_up_net(&mut self, net: IdString) {
        todo!()
    }
    pub fn lock_net_routing(&mut self, name: IdString) {
        todo!()
    }
    pub fn rename_net(&mut self, old_name: IdString, new_name: IdString) {
        todo!()
    }

    pub fn create_cell(&mut self, name: IdString, cell_type: IdString) {
        todo!()
    }
    pub fn copy_bel_ports(&mut self, cell: IdString, bel: BelId) {
        todo!()
    }

    // Workaround for lack of wrappable constructors
    // TODO: Workaround for what? Guessing something in the python API
    pub fn construct_decal_x_y(decal: DecalId, x: f32, y: f32) -> DecalXY {
        todo!()
    }

    pub fn arch_info_to_attributes() {
        todo!()
    }
    pub fn attributes_to_arch_info() {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NetAlias<D>
where
    D: DelayTrait,
{
    Id(IdString),
    Index(Index<NetInfo<D>>),
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum BaseCtxError {
    #[error("Context is not initialized.")]
    ContextNotInitialized,
    #[error("Name not found in context's indices.")]
    NameNotFound,
    #[error("Net Alias not found.")]
    NetAliasNotFound,
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
