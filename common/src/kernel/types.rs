/// Types defined in this header use one or more user defined types (e.g. BelId).
/// If a new common type is desired that doesn't depend on a user defined type,
/// either put it in it's own header, or in nextpnr_base_types.h.
// TODO: Need to figure out the cargo feature based method to
// import the relevant arch definitions.
use crate::ice40::arch_defs::{ArchNetInfo, BelId, DecalId, Delay, PipId, WireId};
use crate::kernel::base_types::{Loc, PlaceStrength};
use crate::kernel::id_string::IdString;
use crate::kernel::property::Property;
use ordered_float::NotNan;
use std::collections::hash_map::HashMap;
use std::collections::BTreeMap;
use std::ops;
use std::ops::Deref;
use typed_index_collections::TiVec;

#[derive(Debug, Copy, Clone, PartialOrd)]
pub struct DecalXY {
    decal: DecalId,
    x: f64,
    y: f64,
}

impl DecalXY {
    pub const fn new() -> Self {
        Self {
            decal: DecalId::new(),
            x: 0.0,
            y: 0.0,
        }
    }
}

impl const PartialEq for DecalXY {
    fn eq(&self, other: &Self) -> bool {
        self.decal == other.decal && self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct BelPin {
    bel: BelId,
    pin: IdString,
}

impl const PartialEq for BelPin {
    fn eq(&self, other: &Self) -> bool {
        self.bel == other.bel && self.pin == other.pin
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Region {
    name: IdString,

    constr_bels: bool,
    constr_wires: bool,
    constr_pips: bool,

    bels: BTreeMap<BelId, u64>,
    wires: BTreeMap<WireId, u64>,
    piplocs: BTreeMap<Loc, u64>,
}

//impl const PartialEq for Region {
//    fn eq(&self, other: &Self) -> bool {
//        self.name == other.name
//            && self.constr_bels == other.constr_bels
//            && self.constr_wires == other.constr_wires
//            && self.constr_pips == other.constr_pips
//        //   && self.bels == other.bels
//    }
//}

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct PipMap {
    pip: PipId,
    strength: PlaceStrength,
}

impl PipMap {
    pub const fn new() -> Self {
        Self {
            pip: PipId::new(),
            strength: PlaceStrength::new(),
        }
    }
}

impl Default for PipMap {
    fn default() -> Self {
        Self::new()
    }
}

impl const PartialEq for PipMap {
    fn eq(&self, other: &Self) -> bool {
        self.pip == other.pip && self.strength == other.strength
    }
}

#[derive(Debug, Clone, Hash, Eq)]
pub struct PortRef {
    cell: Option<Box<CellInfo>>,
    port: IdString,
    budget: Delay,
}

impl const PartialEq for PortRef {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port
            && self.budget == other.budget
            && match (&self.cell, &other.cell) {
                (None, None) => true,
                (None, Some(_)) => false,
                (Some(_), None) => false,
                (Some(s), Some(o)) => s.deref() == o.deref(),
            }
    }
}

impl PortRef {
    pub const fn new() -> Self {
        Self {
            cell: None,
            port: IdString::new(),
            budget: Delay::new(),
        }
    }
}

impl Default for PortRef {
    fn default() -> Self {
        Self::new()
    }
}

/// minimum and maximum delay
#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct DelayPair {
    min_delay: Delay,
    max_delay: Delay,
}

impl DelayPair {
    pub const fn new() -> Self {
        Self {
            min_delay: Delay::new(),
            max_delay: Delay::new(),
        }
    }
    pub const fn with_delay(delay: Delay) -> Self {
        Self {
            min_delay: delay,
            max_delay: delay,
        }
    }
    pub const fn with_min_max(min_delay: Delay, max_delay: Delay) -> Self {
        Self {
            min_delay,
            max_delay,
        }
    }

    pub const fn min_delay(&self) -> Delay {
        self.min_delay
    }

    pub const fn max_delay(&self) -> Delay {
        self.max_delay
    }
}

impl Default for DelayPair {
    fn default() -> Self {
        Self::new()
    }
}

impl const ops::Add<DelayPair> for DelayPair {
    type Output = DelayPair;

    fn add(self, rhs: DelayPair) -> Self::Output {
        DelayPair {
            min_delay: self.min_delay + rhs.min_delay,
            max_delay: self.max_delay + rhs.max_delay,
        }
    }
}

impl const ops::Sub<DelayPair> for DelayPair {
    type Output = DelayPair;

    fn sub(self, rhs: DelayPair) -> Self::Output {
        DelayPair {
            min_delay: self.min_delay - rhs.min_delay,
            max_delay: self.max_delay - rhs.max_delay,
        }
    }
}

impl const PartialEq for DelayPair {
    fn eq(&self, other: &Self) -> bool {
        self.min_delay == other.min_delay && self.max_delay == other.max_delay
    }
}

/// four-quadrant, min and max rise and fall delay
#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct DelayQuad {
    rise: DelayPair,
    fall: DelayPair,
}

impl const PartialEq for DelayQuad {
    fn eq(&self, other: &Self) -> bool {
        self.rise == other.rise && self.fall == other.fall
    }
}

impl DelayQuad {
    pub const fn new() -> Self {
        Self {
            rise: DelayPair::new(),
            fall: DelayPair::new(),
        }
    }

    pub const fn with_delay(delay: Delay) -> Self {
        Self {
            rise: DelayPair::with_delay(delay),
            fall: DelayPair::with_delay(delay),
        }
    }

    pub const fn with_min_max(min_delay: Delay, max_delay: Delay) -> Self {
        Self {
            rise: DelayPair::with_min_max(min_delay, max_delay),
            fall: DelayPair::with_min_max(min_delay, max_delay),
        }
    }

    pub const fn with_rise_fall(rise: DelayPair, fall: DelayPair) -> Self {
        Self { rise, fall }
    }

    pub const fn with_rise_fall_min_max(
        min_rise: Delay,
        max_rise: Delay,
        min_fall: Delay,
        max_fall: Delay,
    ) -> Self {
        Self {
            rise: DelayPair::with_min_max(min_rise, max_rise),
            fall: DelayPair::with_min_max(min_fall, max_fall),
        }
    }

    pub const fn min_rise_delay(&self) -> Delay {
        self.rise.min_delay()
    }

    pub const fn max_rise_delay(&self) -> Delay {
        self.rise.max_delay()
    }

    pub const fn min_fall_felay(&self) -> Delay {
        self.fall.min_delay()
    }

    pub const fn max_fall_delay(&self) -> Delay {
        self.fall.max_delay()
    }

    pub const fn min_delay(&self) -> Delay {
        std::cmp::min(self.rise.min_delay(), self.fall.min_delay())
    }

    pub const fn max_delay(&self) -> Delay {
        std::cmp::max(self.rise.max_delay(), self.fall.max_delay())
    }

    pub const fn delay_pair(&self) -> DelayPair {
        DelayPair::with_min_max(self.min_delay(), self.max_delay())
    }
}

impl const ops::Add<DelayQuad> for DelayQuad {
    type Output = DelayQuad;

    fn add(self, rhs: DelayQuad) -> Self::Output {
        DelayQuad {
            rise: self.rise + rhs.rise,
            fall: self.fall + rhs.fall,
        }
    }
}

impl const ops::Sub<DelayQuad> for DelayQuad {
    type Output = DelayQuad;

    fn sub(self, rhs: DelayQuad) -> Self::Output {
        DelayQuad {
            rise: self.rise - rhs.rise,
            fall: self.fall - rhs.fall,
        }
    }
}

impl const Default for DelayQuad {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NetInfo {
    arch_net_info: ArchNetInfo,
    name: IdString,
    hierarchy_path: IdString,
    udata: i32,

    driver: PortRef,
    users: TiVec<usize, PortRef>,
    attrs: BTreeMap<IdString, Property>,

    // wire -> uphill_pip
    wires: BTreeMap<WireId, PipMap>,

    aliases: Vec<IdString>, // entries in net_aliases that point to this net

    clk_constr: Box<ClockConstraint>,

    region: Option<Box<Region>>,
}

//impl const PartialEq for NetInfo {
//    fn eq(&self, other: &Self) -> bool {
//        self.arch_net_info == other.arch_net_info
//            && self.name == other.name
//            && self.hierarchy_path == other.hierarchy_path
//            && self.udata == other.udata
//            && self.driver == other.driver && self.users == other.users
//    }
//}

impl NetInfo {
    pub fn new() -> Self {
        Self {
            arch_net_info: ArchNetInfo::new(),
            name: IdString::new(),
            hierarchy_path: IdString::new(),
            udata: 0,
            driver: PortRef::new(),
            users: TiVec::new(),
            attrs: BTreeMap::new(),
            wires: BTreeMap::new(),
            aliases: Vec::new(),
            clk_constr: Box::new(ClockConstraint::new()),
            region: None,
        }
    }
    pub fn with_name(name: IdString) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

impl Default for NetInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub enum PortType {
    In = 0,
    Out = 1,
    InOut = 2,
}

impl const PartialEq for PortType {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PortInfo {
    name: IdString,
    net: Box<NetInfo>,
    port_type: PortType,
    user_index: TiVec<usize, PortRef>,
}

//impl const PartialEq for PortInfo {
//    fn eq(&self, other: &Self) -> bool {
//        todo!()
//    }
//}

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub enum TimingPortClass {
    ClockInput,     // Clock input to a sequential cell
    GenClock,       // Generated clock output (PLL, DCC, etc)
    RegisterInput, // Input to a register, with an associated clock (may also have comb. fanout too)
    RegisterOutput, // Output from a register
    CombInput,     // Combinational input, no paths end here
    CombOutput,    // Combinational output, no paths start here
    StartPoint,    // Unclocked primary startpoint, such as an IO cell output
    EndPoint,      // Unclocked primary endpoint, such as an IO cell input
    Ignore, // Asynchronous to all clocks, "don't care", and should be ignored (false path) for analysis
}

impl TimingPortClass {
    pub const fn new() -> Self {
        TimingPortClass::Ignore
    }
}

impl const PartialEq for TimingPortClass {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub enum ClockEdge {
    RisingEdge,
    FallingEdge,
}

impl const PartialEq for ClockEdge {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct TimingClockingInfo {
    clock_port: IdString, // Port name of clock domain
    edge: ClockEdge,
    setup: DelayPair,      // Input timing checks
    hold: DelayPair,       // Input timing checks
    clock_to_q: DelayQuad, // Output clock-to-Q time
}

impl const PartialEq for TimingClockingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.clock_port == other.clock_port
            && self.edge == other.edge
            && self.setup == other.setup
            && self.hold == other.hold
            && self.clock_to_q == other.clock_to_q
    }
}

impl TimingClockingInfo {
    pub const fn new() -> Self {
        Self {
            clock_port: IdString::new(),
            edge: ClockEdge::RisingEdge,
            setup: DelayPair::new(),
            hold: DelayPair::new(),
            clock_to_q: DelayQuad::new(),
        }
    }
}

pub trait PseudoCell {
    fn get_location(&self) -> Loc {
        Loc::origin()
    }
    fn get_port_wire(&self, port: IdString) -> Option<WireId> {
        None
    }
    fn get_delay(&self, from_port: IdString, to_port: IdString, delay: &DelayQuad) -> bool {
        false
    }
    fn get_port_timing_class(&self, port: IdString, clock_info_count: &u64) -> TimingPortClass {
        TimingPortClass::new()
    }
    fn get_port_clocking_info(&self, port: IdString, index: u64) -> TimingClockingInfo {
        TimingClockingInfo::new()
    }
}

pub struct RegionPlug {
    port_wires: BTreeMap<IdString, WireId>,
    loc: Loc
}

impl RegionPlug {
    pub const fn new(loc: Loc) -> Self {
        Self { port_wires: BTreeMap::new(), loc }
    }
}

impl PseudoCell for RegionPlug {
    fn get_location(&self) -> Loc {
        self.loc
    }

    fn get_port_wire(&self, port: IdString) -> Option<WireId> {
        if let Some(wire) = self.port_wires.get(&port) {
            Some(*wire)
        } else {
            None
        }
    }

    // TODO: partial reconfiguration region timing
    fn get_delay(&self, from_port: IdString, to_port: IdString, delay: &DelayQuad) -> bool {
        false
    }

    fn get_port_timing_class(&self, port: IdString, clock_info_count: &u64) -> TimingPortClass {
        TimingPortClass::Ignore
    }

    fn get_port_clocking_info(&self, port: IdString, index: u64) -> TimingClockingInfo {
        TimingClockingInfo::new()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct CellInfo;

impl const PartialEq for CellInfo {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ClockConstraint;

impl ClockConstraint {
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct Context {}
