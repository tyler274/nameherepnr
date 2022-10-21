use super::{cell::HierarchicalCell, id_string::IdString, property::Property};
use std::collections::vec_deque::VecDeque;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use thunderdome::Index;
//use std::hash::{BuildHasher, BuildHasherDefault};
//use std::hash::{Hash, Hasher};

pub struct BaseCtx {
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
    pub log_strs: VecDeque<String>,

    // Project settings and config switches
    pub settings: BTreeMap<IdString, Property>,

    // Placed nets and cells.
    pub nets: BTreeMap<IdString, Index>,
    pub cells: BTreeMap<IdString, Index>,

    // Hierarchical (non-leaf) cells by full path
    pub hierarchy: BTreeMap<IdString, HierarchicalCell>,
    // This is the root of the above structure
    pub top_module: IdString,

    // Aliases for nets, which may have more than one name due to assignments and hierarchy
    net_aliases: BTreeMap<IdString, IdString>,

    // Top-level ports
    // Index is for PortInfo(s).
    ports: BTreeMap<IdString, Index>,
    // The Index is for CellInfo(s).
    port_cells: BTreeMap<IdString, Index>,

    // Floorplanning regions
    // The Index maps to Region(s), unique_ptrs in nextpnr
    region: BTreeMap<IdString, Index>,
}

impl BaseCtx {}

pub struct GuardedBaseCtx(Arc<Mutex<BaseCtx>>);
