use hashers::oz::DJB2Hasher;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::hash::{Hash, Hasher};

pub type Delay = usize;

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct BelId {
    index: Option<u64>,
}

impl BelId {
    pub const fn new() -> Self {
        Self {
            //            index: -1
            index: None,
        }
    }
    pub const fn hash(&self) -> u64 {
        self.index.unwrap()
    }
    pub const fn equals(&self, other: &Self) -> bool {
        self.index.unwrap() == other.index.unwrap()
    }
    pub const fn not_equals(&self, other: &Self) -> bool {
        self.index.unwrap() != other.index.unwrap()
    }
    pub const fn less_than(&self, other: &Self) -> bool {
        self.index.unwrap() < other.index.unwrap()
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct WireId {
    index: Option<u64>,
}

impl WireId {
    pub const fn new() -> Self {
        Self {
            //            index: -1
            index: None,
        }
    }
    pub const fn hash(&self) -> u64 {
        self.index.unwrap()
    }
    pub const fn equals(&self, other: &Self) -> bool {
        self.index.unwrap() == other.index.unwrap()
    }
    pub const fn not_equals(&self, other: &Self) -> bool {
        self.index.unwrap() != other.index.unwrap()
    }
    pub const fn less_than(&self, other: &Self) -> bool {
        self.index.unwrap() < other.index.unwrap()
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, Eq)]
pub struct PipId {
    index: Option<u64>,
}

impl PipId {
    pub const fn new() -> Self {
        Self {
            //            index: -1
            index: None,
        }
    }
    pub const fn hash(&self) -> u64 {
        self.index.unwrap()
    }
}

impl const PartialEq for PipId {
    fn eq(&self, other: &Self) -> bool {
        self.index.unwrap() == other.index.unwrap()
    }

    fn ne(&self, other: &Self) -> bool {
        self.index.unwrap() != other.index.unwrap()
    }
}

impl const PartialOrd for PipId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // TODO: Fix this to return None instead of panicking on unwrapping a none value.
        Some(self.index.unwrap().cmp(&other.index.unwrap()))
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub enum GroupType {
    None,
    Frame,
    MainSW,
    LocalSW,
    LC0SW,
    LC1SW,
    LC2SW,
    LC3SW,
    LC4SW,
    LC5SW,
    LC6SW,
    LC7SW,
}

impl const PartialEq for GroupType {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}

impl const PartialOrd for GroupType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl const Ord for GroupType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

impl GroupType {
    pub const fn new() -> Self {
        GroupType::None
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq)]
pub struct GroupId {
    gtype: GroupType,
    index: Option<u64>,
    active: bool,
    x: i8,
    y: i8,
}

impl Hash for GroupId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.gtype.hash(state);
    }
}

impl const PartialEq for GroupId {
    fn eq(&self, other: &Self) -> bool {
        self.gtype == other.gtype && self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.gtype != other.gtype || self.x != other.x || self.y != other.y
    }
}

impl GroupId {
    pub const fn new() -> Self {
        Self {
            gtype: GroupType::new(),
            index: None,
            active: false,
            x: 0,
            y: 0,
        }
    }
    pub fn get_hash(&self) -> u64 {
        let mut hasher = BuildHasherDefault::<DJB2Hasher>::default().build_hasher();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq)]
pub enum DecalType {
    None,
    Bel,
    Wire,
    Pip,
    Group,
}

impl DecalType {
    pub const fn new() -> Self {
        Self::None
    }
}

impl const PartialEq for DecalType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }

    fn ne(&self, other: &Self) -> bool {
        self != other
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq)]
pub struct DecalId {
    dtype: DecalType,
    index: Option<u64>,
    active: bool,
}

impl DecalId {
    pub const fn new() -> Self {
        Self {
            dtype: DecalType::new(),
            index: None,
            active: false,
        }
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = BuildHasherDefault::<DJB2Hasher>::default().build_hasher();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl const PartialEq for DecalId {
    fn eq(&self, other: &Self) -> bool {
        self.dtype == other.dtype && self.index.unwrap() == other.index.unwrap()
    }

    fn ne(&self, other: &Self) -> bool {
        self.dtype != other.dtype || self.index.unwrap() != other.index.unwrap()
    }
}

impl Hash for DecalId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.dtype.hash(state);
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ArchNetInfo {
    is_global: bool,
    is_reset: bool,
    is_enable: bool,
}

pub struct NetInfo;

pub struct ArchCellInfo {}
