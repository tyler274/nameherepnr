use crate::kernel::base_clusterinfo::BaseClusterInfo;
use crate::kernel::id_string::IdString;
use hashers::oz::DJB2Hasher;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::hash::{Hash, Hasher};
use std::ops;

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct Delay(u64);

impl const Ord for Delay {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl const From<u64> for Delay {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Delay {
    pub const fn new() -> Self {
        Self(0)
    }
}

impl const ops::Add<Delay> for Delay {
    type Output = Delay;

    fn add(self, rhs: Delay) -> Self::Output {
        Delay(self.0 + rhs.0)
    }
}

impl const ops::Sub<Delay> for Delay {
    type Output = Delay;

    fn sub(self, rhs: Delay) -> Self::Output {
        Delay(self.0 - rhs.0)
    }
}

impl const PartialEq for Delay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl const PartialOrd for Delay {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Default for Delay {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, Eq)]
pub struct BelId {
    index: Option<u64>,
}

impl const PartialEq for BelId {
    fn eq(&self, other: &Self) -> bool {
        match (self.index, other.index) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(s), Some(o)) => s == o,
        }
    }
}

impl PartialOrd for BelId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.index, other.index) {
            (None, None) => Some(std::cmp::Ordering::Equal),
            (None, Some(_)) => Some(std::cmp::Ordering::Less),
            (Some(_), None) => Some(std::cmp::Ordering::Greater),
            (Some(s), Some(o)) => Some(s.cmp(&o)),
        }
    }
}

impl BelId {
    pub const fn new() -> Self {
        Self {
            // index: -1
            index: None,
        }
    }
    pub const fn hash(&self) -> Option<u64> {
        self.index
    }
}

impl Default for BelId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, Eq)]
pub struct WireId {
    index: Option<u64>,
}

impl const PartialEq for WireId {
    fn eq(&self, other: &Self) -> bool {
        match (self.index, other.index) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(s), Some(o)) => s == o,
        }
    }
}

impl const PartialOrd for WireId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.index, other.index) {
            (None, None) => Some(std::cmp::Ordering::Equal),
            (None, Some(_)) => Some(std::cmp::Ordering::Less),
            (Some(_), None) => Some(std::cmp::Ordering::Greater),
            (Some(s), Some(o)) => Some(s.cmp(&o)),
        }
    }
}

impl WireId {
    pub const fn new() -> Self {
        Self {
            //            index: -1
            index: None,
        }
    }
    pub const fn zero() -> Self {
        let mut w = WireId::new();
        w.index = Some(0);
        w
    }
    pub const fn with_index(index: u64) -> Self {
        let mut w = WireId::new();
        w.index = Some(index);
        w
    }
    pub const fn hash(&self) -> u64 {
        self.index.unwrap()
    }
}

impl Default for WireId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, Eq)]
pub struct PipId {
    index: Option<u64>,
}

impl PipId {
    pub const fn new() -> Self {
        Self {
            // index: -1
            index: None,
        }
    }
    pub const fn hash(&self) -> Option<u64> {
        self.index
    }
}

impl const PartialEq for PipId {
    fn eq(&self, other: &Self) -> bool {
        match (self.index, other.index) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(s), Some(o)) => s == o,
        }
    }
}

impl const PartialOrd for PipId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.index, other.index) {
            (None, None) => Some(std::cmp::Ordering::Equal),
            (None, Some(_)) => Some(std::cmp::Ordering::Less),
            (Some(_), None) => Some(std::cmp::Ordering::Greater),
            (Some(s), Some(o)) => Some(s.cmp(&o)),
        }
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

impl Default for GroupType {
    fn default() -> Self {
        Self::new()
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

impl Default for DecalType {
    fn default() -> Self {
        Self::new()
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

impl Default for DecalId {
    fn default() -> Self {
        Self::new()
    }
}

impl const PartialEq for DecalId {
    fn eq(&self, other: &Self) -> bool {
        self.dtype == other.dtype
            && match (self.index, other.index) {
                (None, None) => true,
                (None, Some(_)) => false,
                (Some(_), None) => false,
                (Some(s), Some(o)) => s == o,
            }
    }

    fn ne(&self, other: &Self) -> bool {
        self.dtype != other.dtype
            || match (self.index, other.index) {
                (None, None) => false,
                (None, Some(_)) => true,
                (Some(_), None) => true,
                (Some(s), Some(o)) => s != o,
            }
    }
}

impl Hash for DecalId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.dtype.hash(state);
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq)]
pub struct ArchNetInfo {
    is_global: bool,
    is_reset: bool,
    is_enable: bool,
}
impl ArchNetInfo {
    pub const fn new() -> Self {
        Self {
            is_global: false,
            is_reset: false,
            is_enable: false,
        }
    }
}

impl const PartialEq for ArchNetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.is_global == other.is_global
            && self.is_reset == other.is_reset
            && self.is_enable == other.is_enable
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct NetInfo;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LcInfo {
    dffEnable: bool,
    carryEnable: bool,
    negClk: bool,
    inputCount: i32,
    lutInputMask: u32,
    clk: Box<NetInfo>,
    cen: Box<NetInfo>,
    sr: Box<NetInfo>,
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct IoInfo {
    lvds: bool,
    global: bool,
    negtrig: bool,
    pintype: i32,
    // TODO: clk packing checks...
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct GbInfo {
    for_pad_in: bool,
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LedInfo {
    led_cur_connected: bool,
}

// TODO: Does this need to be a C style Union? repr(C)?
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum CellEnum {
    Lc(LcInfo),
    Io(IoInfo),
    Gb(GbInfo),
    Led(LedInfo),
}

// TODO: Does this need to be repr(C)?
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ArchCellInfo {
    base_cluster_info: BaseClusterInfo,
    cell: CellEnum,
}

pub type BelBucketId = IdString;
pub type ClusterId = IdString;