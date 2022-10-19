/// Types defined in this header use one or more user defined types (e.g. BelId).
/// If a new common type is desired that doesn't depend on a user defined type,
/// either put it in it's own header, or in nextpnr_base_types.h.
// TODO: Need to figure out the cargo feature based method to
// import the relevant arch definitions.
use crate::ice40::arch_defs::{
    ArchCellInfo, ArchNetInfo, BelId, ClusterId, DecalId, PipId, WireId,
};
use crate::kernel::base_types::{Loc, PlaceStrength};
use crate::kernel::delay::{Delay, DelayPair, DelayQuad, DelayTrait};
use crate::kernel::id_string::{IdPair, IdString};
use crate::kernel::property::Property;
use ordered_float::NotNan;
use std::cmp::PartialEq;
use std::collections::hash_map::HashMap;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops;
use std::ops::Deref;
use typed_index_collections::TiVec;

// Unstable trait bounds aliasing, to reduce code duplication.
pub trait CellTrait<DelayType: DelayTrait> = Debug + PseudoCell<DelayType> + PartialEq + Clone;

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

#[derive(Debug, Clone, Eq)]
pub struct PortRef<DelayType: DelayTrait, CellType: CellTrait<DelayType>> {
    cell: Option<Box<CellInfo<DelayType, CellType>>>,
    port: IdString,
    budget: Delay<DelayType>,
}

impl<DelayType, CellType> const PartialEq for PortRef<DelayType, CellType>
where
    DelayType: DelayTrait + ~const PartialEq,
    CellType: CellTrait<DelayType>,
{
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

impl<DelayType, CellType> PortRef<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    pub const fn new() -> Self
    where
        DelayType: ~const DelayTrait,
    {
        Self {
            cell: None,
            port: IdString::new(),
            budget: Delay::new(),
        }
    }
}

impl<DelayType: DelayTrait, CellType: CellTrait<DelayType>> Default
    for PortRef<DelayType, CellType>
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NetInfo<DelayType: DelayTrait, CellType: CellTrait<DelayType>> {
    arch_net_info: ArchNetInfo,
    name: IdString,
    hierarchy_path: IdString,
    udata: i32,

    driver: PortRef<DelayType, CellType>,
    users: TiVec<usize, PortRef<DelayType, CellType>>,
    attrs: BTreeMap<IdString, Property>,

    // wire -> uphill_pip
    wires: BTreeMap<WireId, PipMap>,

    aliases: Vec<IdString>, // entries in net_aliases that point to this net

    clk_constr: Box<ClockConstraint<DelayType>>,

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

impl<DelayType, CellType> NetInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
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

impl<DelayType, CellType> Default for NetInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
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
pub struct PortInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    name: IdString,
    net: Box<NetInfo<DelayType, CellType>>,
    port_type: PortType,
    user_index: TiVec<usize, PortRef<DelayType, CellType>>,
}

impl<DelayType, CellType> PortInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    pub fn new() -> Self {
        Self {
            name: IdString::new(),
            net: Box::new(NetInfo::new()),
            port_type: PortType::In,
            user_index: TiVec::new(),
        }
    }
    pub fn update_name(&mut self, name: IdString) -> &mut Self {
        self.name = name;
        self
    }
    pub fn update_port(&mut self, port_type: PortType) -> &mut Self {
        self.port_type = port_type;
        self
    }
}

impl<DelayType: DelayTrait, CellType: CellTrait<DelayType>> Default
    for PortInfo<DelayType, CellType>
{
    fn default() -> Self
    where
        DelayType: ~const DelayTrait,
    {
        Self::new()
    }
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
pub struct TimingClockingInfo<DelayType: DelayTrait> {
    clock_port: IdString, // Port name of clock domain
    edge: ClockEdge,
    setup: DelayPair<DelayType>,      // Input timing checks
    hold: DelayPair<DelayType>,       // Input timing checks
    clock_to_q: DelayQuad<DelayType>, // Output clock-to-Q time
}

impl<DelayType> const PartialEq for TimingClockingInfo<DelayType>
where
    DelayType: DelayTrait + ~const PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.clock_port == other.clock_port
            && self.edge == other.edge
            && self.setup == other.setup
            && self.hold == other.hold
            && self.clock_to_q == other.clock_to_q
    }
}

impl<DelayType> TimingClockingInfo<DelayType>
where
    DelayType: DelayTrait,
{
    pub const fn new() -> Self
    where
        DelayType: ~const DelayTrait,
    {
        Self {
            clock_port: IdString::new(),
            edge: ClockEdge::RisingEdge,
            setup: DelayPair::new(),
            hold: DelayPair::new(),
            clock_to_q: DelayQuad::new(),
        }
    }
}
pub trait PseudoCell<DelayType: DelayTrait> {
    fn get_location(&self) -> Loc {
        Loc::origin()
    }
    fn get_port_wire(&self, port: IdString) -> Option<WireId> {
        None
    }
    fn get_delay(
        &self,
        from_port: IdString,
        to_port: IdString,
        delay: &DelayQuad<DelayType>,
    ) -> bool {
        false
    }
    fn get_port_timing_class(&self, port: IdString, clock_info_count: &u64) -> TimingPortClass {
        TimingPortClass::new()
    }
    fn get_port_clocking_info(&self, port: IdString, index: u64) -> TimingClockingInfo<DelayType> {
        TimingClockingInfo::new()
    }
}

pub struct RegionPlug {
    port_wires: BTreeMap<IdString, WireId>,
    loc: Loc,
}

impl RegionPlug {
    pub const fn new(loc: Loc) -> Self {
        Self {
            port_wires: BTreeMap::new(),
            loc,
        }
    }
}

impl<DelayType: DelayTrait> PseudoCell<DelayType> for RegionPlug {
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
    fn get_delay(
        &self,
        from_port: IdString,
        to_port: IdString,
        delay: &DelayQuad<DelayType>,
    ) -> bool {
        false
    }

    fn get_port_timing_class(&self, port: IdString, clock_info_count: &u64) -> TimingPortClass {
        TimingPortClass::Ignore
    }

    fn get_port_clocking_info(&self, port: IdString, index: u64) -> TimingClockingInfo<DelayType> {
        TimingClockingInfo::new()
    }
}

#[derive(Debug, Clone, Eq)]
pub struct CellInfo<DelayType: DelayTrait, CellType: CellTrait<DelayType>> {
    arch_cell_info: ArchCellInfo,
    context: Option<Box<Context>>,

    name: IdString,
    cell_type: IdString,
    hierarchy_path: IdString,
    udata: i32,

    ports: BTreeMap<IdString, PortInfo<DelayType, CellType>>,
    attributes: BTreeMap<IdString, Property>,
    parameters: BTreeMap<IdString, Property>,

    bel: BelId,
    bel_strength: PlaceStrength,

    // cell is part of a cluster if != ClusterId
    cluster: ClusterId,

    region: Option<Box<Region>>,

    pseudo_cell: Option<Box<CellType>>,
}

impl<DelayType: DelayTrait, CellType: CellTrait<DelayType>> const PartialEq
    for CellInfo<DelayType, CellType>
{
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl<DelayType: DelayTrait, CellType: CellTrait<DelayType>> CellInfo<DelayType, CellType> {
    pub fn add_input(&mut self, name: IdString) {
        self.ports.insert(
            name,
            PortInfo {
                name,
                port_type: PortType::In,
                net: self.ports[&name].net.clone(),
                user_index: self.ports[&name].user_index.clone(),
            },
        );
    }
    pub fn add_output(&mut self, name: IdString) {
        self.ports.insert(
            name,
            PortInfo {
                name,
                port_type: PortType::Out,
                net: self.ports[&name].net.clone(),
                user_index: self.ports[&name].user_index.clone(),
            },
        );
    }
    pub fn add_in_out(&mut self, name: IdString) {
        self.ports.insert(
            name,
            PortInfo {
                name,
                port_type: PortType::InOut,
                net: self.ports[&name].net.clone(),
                user_index: self.ports[&name].user_index.clone(),
            },
        );
    }
    pub fn set_param(&mut self, name: IdString, value: Property) {
        self.parameters.insert(name, value);
    }
    pub fn unset_param(&mut self, name: IdString) {
        self.parameters.remove(&name);
    }
    pub fn set_attribute(&mut self, name: IdString, value: Property) {
        self.attributes.insert(name, value);
    }
    pub fn unset_attribute(&mut self, name: IdString) {
        self.attributes.remove(&name);
    }

    // check whether a bel complies with the cell's region constraint
    pub fn test_region(&self, bel: BelId) -> bool {
        if let Some(region) = &self.region {
            region.constr_bels || region.bels.contains_key(&bel)
        } else {
            true
        }
    }

    pub const fn is_pseudo(&self, bel: BelId) -> bool {
        todo!()
    }

    pub const fn get_location(&self, bel: BelId) -> bool {
        todo!()
    }

    pub fn get_port(&self, name: IdString) -> &NetInfo<DelayType, CellType> {
        let found = &self.ports[&name];
        //        if (found == self.ports.last_entry().) {
        //
        //        }
        todo!()
    }

    pub fn connect_port(&mut self, port_name: IdString, net: &NetInfo<DelayType, CellType>) {
        //        let port = self.ports[&port]
        todo!()
    }

    pub fn disconnect_port(&mut self, port: IdString) {
        todo!()
    }

    pub fn connect_ports(
        &mut self,
        port: IdString,
        other: &CellInfo<DelayType, CellType>,
        other_port: IdString,
    ) {
        todo!()
    }

    pub fn move_port_to(
        &mut self,
        port: IdString,
        other: &CellInfo<DelayType, CellType>,
        other_port: IdString,
    ) {
        todo!()
    }

    pub fn rename_port(&mut self, old_name: IdString, new_name: IdString) {
        todo!()
    }

    pub fn move_port_bus_to(
        &mut self,
        old_name: IdString,
        old_offset: i32,
        old_brackets: bool,
        new_cell: &CellInfo<DelayType, CellType>,
        new_name: IdString,
        new_offset: i32,
        new_brackets: bool,
        width: i32,
    ) {
        todo!()
    }

    pub fn copy_port_(
        &self,
        port: IdString,
        other: &CellInfo<DelayType, CellType>,
        other_port: IdString,
    ) {
        todo!()
    }

    pub fn copy_port_bus_to(
        &self,
        old_name: IdString,
        old_offset: i32,
        old_brackets: bool,
        new_cell: &CellInfo<DelayType, CellType>,
        new_name: IdString,
        new_offset: i32,
        new_brackets: bool,
        width: i32,
    ) {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ClockConstraint<DelayType: DelayTrait> {
    high: DelayPair<DelayType>,
    low: DelayPair<DelayType>,
    period: DelayPair<DelayType>,
}

impl<DelayType> ClockConstraint<DelayType>
where
    DelayType: DelayTrait,
{
    pub const fn new() -> Self
    where
        DelayType: ~const DelayTrait,
    {
        Self {
            high: DelayPair::new(),
            low: DelayPair::new(),
            period: DelayPair::new(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ClockFmax {
    achieved: NotNan<f32>,
    constraint: NotNan<f32>,
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct ClockEvent {
    clock: IdString,
    edge: ClockEdge,
}

impl const PartialEq for ClockEvent {
    fn eq(&self, other: &Self) -> bool {
        self.clock == other.clock && self.edge == other.edge
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct ClockPair {
    start: ClockEvent,
    end: ClockEvent,
}

impl const PartialEq for ClockPair {
    fn eq(&self, other: &Self) -> bool {
        self.start == self.end && other.start == other.end
    }
}

/// Segment type
#[derive(Debug, Copy, Clone, Eq)]
pub enum SegmentType {
    ClkToQ,  // Clock-to-Q delay
    Source,  // Delayless source
    Logic,   // Combinational logic delay
    Routing, // Routing delay
    Setup,   // Setup time in sink
}

impl const PartialEq for SegmentType {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}

#[derive(Debug, Copy, Clone, Eq)]
pub struct Segment<DelayType: DelayTrait> {
    // Type
    segment_type: SegmentType,
    // Net name (routing only)
    net: IdString,
    // From cell.port
    from: IdPair,
    // To cell.port
    to: IdPair,
    // Segment delay
    delay: Delay<DelayType>,
    // Segment budget (routing only)
    budget: Delay<DelayType>,
}

impl<DelayType> const PartialEq for Segment<DelayType>
where
    DelayType: DelayTrait + ~const PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.segment_type == other.segment_type
            && self.net == other.net
            && self.from == other.from
            && self.to == other.to
            && self.delay == other.delay
            && self.budget == other.budget
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CriticalPath<DelayType: DelayTrait> {
    // Clock pair
    clock_pair: ClockPair,
    // Total path delay
    delay: Delay<DelayType>,
    // Period (max allowed delay)
    period: Delay<DelayType>,
    // Individual path segments
    segments: Vec<Segment<DelayType>>,
}

/// Holds timing information of a single source to sink path of a net
#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct NetSinkTiming<DelayType: DelayTrait> {
    // Clock event pair
    clock_pair: ClockPair,
    // Cell and port (the sink)
    cell_port: IdPair,
    // Delay
    delay: Delay<DelayType>,
    // Delay budget
    budget: Delay<DelayType>,
}

impl<DelayType> const PartialEq for NetSinkTiming<DelayType>
where
    DelayType: DelayTrait + ~const PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.clock_pair == other.clock_pair
            && self.cell_port == other.cell_port
            && self.delay == other.delay
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimingResult<DelayType: DelayTrait> {
    // Achieved and target Fmax for all clock domains
    clock_fmax: BTreeMap<IdString, ClockFmax>,
    // Single domain critical paths
    clock_paths: BTreeMap<IdString, CriticalPath<DelayType>>,
    // Cross-domain critical paths
    xclock_paths: Vec<CriticalPath<DelayType>>,

    // Detailed net timing data
    detailed_net_timings: BTreeMap<IdString, Vec<NetSinkTiming<DelayType>>>,
}

/// Represents the contents of a non-leaf cell in a design
/// with hierarchy
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct HierarchicalPort {
    name: IdString,
    dir: PortType,
    nets: Vec<IdString>,
    offset: i32,
    up_to: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct HierarchicalCell {
    name: IdString,
    cell_type: IdString,
    parent: IdString,
    fullpath: IdString,
    // Name inside cell instance -> global name
    leaf_cells: BTreeMap<IdString, IdString>,
    nets: BTreeMap<IdString, IdString>,
    // Global name -> name inside cell instance
    leaf_cells_by_global_name: BTreeMap<IdString, IdString>,
    nets_by_global_name: BTreeMap<IdString, IdString>,
    // Cell port to net
    ports: BTreeMap<IdString, HierarchicalPort>,
    // Name inside cell instance -> global name
    hierachical_cells: BTreeMap<IdString, IdString>,
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Context {}
