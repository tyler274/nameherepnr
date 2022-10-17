use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;
use std::sync::Mutex;

use hashers::oz::DJB2Hasher;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::hash::{Hash, Hasher};

use crate::kernel::id_string::IdString;
use crate::kernel::property::Property;

pub struct BaseCtx {
    // Lock to perform mutating actions on the Context.
    //    pub mutex: Mutex,

    // Lock to be taken by UI when wanting to access context - the yield()
    // method will lock/unlock it when its' released the main mutex to make
    // sure the UI is not starved.
    //    pub ui_mutex: Mutex,

    // ID String database.
    pub idstring_idx_to_str: Vec<String>,
    pub idstring_str_to_id: HashMap<String, usize>,

    // Temporary string backing store for logging
    pub log_strs: VecDeque<String>,

    // Project settings and config switches
    pub settings: HashMap<IdString, Property, DJB2Hasher>,
    // Placed nets and cells.
    //    pub nets: HashMap<IdString, &NetInfo, DJB2Hasher>,
    //    pub cells: HashMap<IdString, &CellInfo, DJB2Hasher>

    // TODO: Rest of this module
}

impl BaseCtx {}
