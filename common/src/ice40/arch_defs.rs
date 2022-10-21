use crate::kernel::base_clusterinfo::BaseClusterInfo;
use crate::kernel::id_string::IdString;
use hashers::oz::DJB2Hasher;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::hash::{Hash, Hasher};
use thunderdome::Index;

pub trait BelIdTrait {}

#[derive(Debug, Copy, Clone, Eq)]
pub struct BelId {
    index: Option<u64>,
}

impl Hash for BelId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl Ord for BelId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
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
    //    pub const fn hash(&self) -> Option<u64> {
    //        self.index
    //    }
}

impl Default for BelId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, Eq)]
pub struct WireId {
    index: Option<u64>,
}

impl Hash for WireId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl Ord for WireId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
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

#[derive(Debug, Copy, Clone, Eq)]
pub struct PipId {
    index: Option<u64>,
}

impl Ord for PipId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl PipId {
    pub const fn new() -> Self {
        Self {
            // index: -1
            index: None,
        }
    }
    //    pub const fn hash(&self) -> Option<u64> {
    //        self.index
    //    }
}

impl Hash for PipId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
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

#[derive(Debug, Copy, Clone, Eq)]
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

impl Hash for GroupType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as usize).hash(state);
    }
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

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq)]
pub enum DecalType {
    None,
    Bel,
    Wire,
    Pip,
    Group,
}

impl Hash for DecalType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as usize).hash(state);
    }
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
}

impl Hash for DecalId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.dtype.hash(state);
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq)]
pub struct ArchNetInfo {
    is_global: bool,
    is_reset: bool,
    is_enable: bool,
}

impl Hash for ArchNetInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.is_global.hash(state);
        self.is_reset.hash(state);
        self.is_enable.hash(state);
    }
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

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LcInfo {
    dff_enable: bool,
    carry_enable: bool,
    neg_clk: bool,
    input_count: i32,
    lut_input_mask: u32,
    //    clk: Box<NetInfo>,
    //    cen: Box<NetInfo>,
    //    sr: Box<NetInfo>,
    clk: Index,
    cen: Index,
    srd: Index,
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct IoInfo {
    lvds: bool,
    global: bool,
    negtrig: bool,
    pintype: i32,
    // TODO: clk packing checks...
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq)]
pub struct GbInfo {
    for_pad_in: bool,
}

impl const PartialEq for GbInfo {
    fn eq(&self, other: &Self) -> bool {
        self.for_pad_in == other.for_pad_in
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq)]
pub struct LedInfo {
    led_cur_connected: bool,
}

impl const PartialEq for LedInfo {
    fn eq(&self, other: &Self) -> bool {
        self.led_cur_connected == other.led_cur_connected
    }
}

// TODO: Does this need to be a C style Union? repr(C)?
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum CellEnum {
    Lc(LcInfo),
    Io(IoInfo),
    Gb(GbInfo),
    Led(LedInfo),
}

//impl const PartialEq for CellEnum {
//    fn eq(&self, other: &Self) -> bool {
//        match (self, other) {
//            (CellEnum::Lc(_), CellEnum::Lc(_)) => todo!(),
//            (CellEnum::Lc(_), CellEnum::Io(_)) => todo!(),
//            (CellEnum::Lc(_), CellEnum::Gb(_)) => todo!(),
//            (CellEnum::Lc(_), CellEnum::Led(_)) => todo!(),
//            (CellEnum::Io(_), CellEnum::Lc(_)) => todo!(),
//            (CellEnum::Io(_), CellEnum::Io(_)) => todo!(),
//            (CellEnum::Io(_), CellEnum::Gb(_)) => todo!(),
//            (CellEnum::Io(_), CellEnum::Led(_)) => todo!(),
//            (CellEnum::Gb(_), CellEnum::Lc(_)) => todo!(),
//            (CellEnum::Gb(_), CellEnum::Io(_)) => todo!(),
//            (CellEnum::Gb(_), CellEnum::Gb(_)) => todo!(),
//            (CellEnum::Gb(_), CellEnum::Led(_)) => todo!(),
//            (CellEnum::Led(_), CellEnum::Lc(_)) => todo!(),
//            (CellEnum::Led(_), CellEnum::Io(_)) => todo!(),
//            (CellEnum::Led(_), CellEnum::Gb(_)) => todo!(),
//            (CellEnum::Led(_), CellEnum::Led(_)) => todo!(),
//        }
//    }
//}

// TODO: Does this need to be repr(C)?
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ArchCellInfo {
    base_cluster_info: BaseClusterInfo,
    cell: CellEnum,
}

impl ArchCellInfo {
    pub const fn new() -> Self {
        todo!()
    }
}

pub type BelBucketId = IdString;
pub type ClusterId = IdString;
