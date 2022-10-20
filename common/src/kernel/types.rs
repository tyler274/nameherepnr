/// Types defined in this header use one or more user defined types (e.g. BelId).
/// If a new common type is desired that doesn't depend on a user defined type,
/// either put it in it's own header, or in nextpnr_base_types.h.
// TODO: Need to figure out the cargo feature based method to
// import the relevant arch definitions.
// TODO: Instead of the above, implement a unified database system that can cleanly represent Xilinx's story.
use crate::ice40::arch_defs::{
    ArchCellInfo, ArchNetInfo, BelId, ClusterId, DecalId, PipId, WireId,
};
use crate::kernel::base_types::{Loc, PlaceStrength};
use crate::kernel::delay::{Delay, DelayPair, DelayQuad, DelayTrait};
use crate::kernel::id_string::{IdPair, IdString};
use crate::kernel::property::Property;
use ordered_float::NotNan;
use std::cmp::PartialEq;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use thiserror::Error;
use thunderdome::{Arena, Index};
use typed_index_collections::TiVec;

// Unstable trait bounds aliasing, to reduce code duplication.
#[const_trait]
pub trait CellTrait<DelayType: DelayTrait>
where
    Self: Debug + PseudoCell<DelayType> + PartialEq + Clone,
{
}

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

#[derive(Debug, Copy, Clone, Eq)]
pub struct BelPin {
    bel: BelId,
    pin: IdString,
}

impl Hash for BelPin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bel.hash(state);
        self.pin.hash(state);
    }
}

impl const PartialEq for BelPin {
    fn eq(&self, other: &Self) -> bool {
        self.bel == other.bel && self.pin == other.pin
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

impl Region {
    pub fn new() -> Self {
        Self {
            name: IdString::new(),
            constr_bels: false,
            constr_wires: false,
            constr_pips: false,
            bels: BTreeMap::new(),
            wires: BTreeMap::new(),
            piplocs: BTreeMap::new(),
        }
    }
}

impl Default for Region {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, Eq)]
pub struct PipMap {
    pip: PipId,
    strength: PlaceStrength,
}

impl Hash for PipMap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pip.hash(state);
        self.strength.hash(state);
    }
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

#[derive(Debug, Clone)]
pub struct PortRef<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    //    cell: Option<Weak<CellInfo<DelayType, CellType>>>,
    /// An index into the arena of cells leading to the cell that a port is tied to.
    cell: Option<Index>,
    /// A phantom data entry to ensure that the PortRef remains specialized to the cell it was instantiated on.
    /// Might want to track the lifetime of the relevant cell arena as well.
    cell_phantom: PhantomData<CellType>,
    port: IdString,
    budget: Delay<DelayType>,
}

impl<DelayType, CellType> PartialEq for PortRef<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port
            && self.budget == other.budget
            && match (&self.cell, &other.cell) {
                (None, None) => true,
                (None, Some(_)) => false,
                (Some(_), None) => false,
                (Some(s), Some(o)) => s == o,
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
            cell_phantom: PhantomData,
            port: IdString::new(),
            budget: Delay::new(),
        }
    }
}

impl<DelayType, CellType> Default for PortRef<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
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

    //    clk_constr: Box<ClockConstraint<DelayType>>,
    clk_constr: Option<Index>,
    //    region: Option<Box<Region>>,
    region: Option<Index>,
}

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
            // TODO: Go upstream and get this made const.
            users: TiVec::new(),
            attrs: BTreeMap::new(),
            wires: BTreeMap::new(),
            aliases: Vec::new(),
            clk_constr: None,
            region: None,
        }
    }
    pub fn with_arena(
        region_arena: &mut Arena<Region>,
        clk_constr_arena: &mut Arena<ClockConstraint<DelayType>>,
    ) -> Self {
        Self {
            clk_constr: Some(clk_constr_arena.insert(ClockConstraint::new())),
            region: Some(region_arena.insert(Region::new())),
            ..Default::default()
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

#[derive(Debug, Copy, Clone, Eq)]
pub enum PortType {
    In = 0,
    Out = 1,
    InOut = 2,
}

impl Hash for PortType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (*self as usize).hash(state);
    }
}

impl const PartialEq for PortType {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PortInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    name: IdString,
    //    net: Box<NetInfo<DelayType, CellType>>,
    net: Option<Index>,
    _net_phantom: PhantomData<NetInfo<DelayType, CellType>>,
    port_type: PortType,
    user_index: usize,
}

impl<DelayType, CellType> PortInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    pub fn new() -> Self {
        Self {
            name: IdString::new(),
            //            net: Box::new(NetInfo::new()),
            net: None,
            port_type: PortType::In,
            user_index: 0,
            _net_phantom: PhantomData,
        }
    }
    pub fn with_arena(_net_arena: &mut Arena<NetInfo<DelayType, CellType>>) -> Self {
        Self {
            name: IdString::new(),
            //            net: Box::new(NetInfo::new()),
            net: None,
            ..Default::default()
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

#[derive(Debug, Copy, Clone, Eq)]
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

impl Hash for TimingPortClass {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (*self as usize).hash(state);
    }
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

#[derive(Debug, Copy, Clone, Eq)]
pub enum ClockEdge {
    RisingEdge,
    FallingEdge,
}

impl Hash for ClockEdge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (*self as usize).hash(state);
    }
}

impl const PartialEq for ClockEdge {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}

#[derive(Debug, Copy, Clone, Eq)]
pub struct TimingClockingInfo<DelayType: DelayTrait> {
    clock_port: IdString, // Port name of clock domain
    edge: ClockEdge,
    setup: DelayPair<DelayType>,      // Input timing checks
    hold: DelayPair<DelayType>,       // Input timing checks
    clock_to_q: DelayQuad<DelayType>, // Output clock-to-Q time
}

impl<DelayType> Hash for TimingClockingInfo<DelayType>
where
    DelayType: DelayTrait,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.clock_port.hash(state);
        self.edge.hash(state);
        self.setup.hash(state);
        self.hold.hash(state);
        self.clock_to_q.hash(state);
    }
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

#[const_trait]
pub trait PseudoCell<DelayType>
where
    DelayType: DelayTrait,
{
    fn get_location(&self) -> Loc {
        Loc::origin()
    }
    fn get_port_wire(&self, _port: IdString) -> Option<WireId> {
        None
    }
    fn get_delay(
        &self,
        _from_port: IdString,
        _to_port: IdString,
        _delay: &DelayQuad<DelayType>,
    ) -> bool {
        false
    }
    fn get_port_timing_class(&self, _port: IdString, _clock_info_count: &u64) -> TimingPortClass {
        TimingPortClass::new()
    }
    fn get_port_clocking_info(&self, _port: IdString, _index: u64) -> TimingClockingInfo<DelayType>
    where
        DelayType: ~const DelayTrait,
    {
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
        //        self.port_wires.get(&port).map(|wire| *wire)
        self.port_wires.get(&port).copied()
    }

    // TODO: partial reconfiguration region timing
    fn get_delay(
        &self,
        _from_port: IdString,
        _to_port: IdString,
        _delay: &DelayQuad<DelayType>,
    ) -> bool {
        false
    }

    fn get_port_timing_class(&self, _port: IdString, _clock_info_count: &u64) -> TimingPortClass {
        TimingPortClass::Ignore
    }

    fn get_port_clocking_info(&self, _port: IdString, _index: u64) -> TimingClockingInfo<DelayType>
    where
        DelayType: ~const DelayTrait,
    {
        TimingClockingInfo::new()
    }
}

#[derive(Debug, Clone)]
pub struct CellInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    _arch_cell_info: ArchCellInfo,
    // Lets try using Arena indices for these.
    //    context: Option<Box<Context>>,
    //    region: Option<Box<Region>>,
    //    pseudo_cell: Option<std::rc::Rc<CellType>>,
    //    rc_self: Weak<Self>,

    // Index to the context within the Context arena.
    _context: Option<Index>,
    // Index to the region within the Region arena.
    region: Option<Index>,
    // TODO: What is a pseudo cell really?
    pseudo_cell: Option<Index>,
    // Index to this cell in the Arena
    self_index: Option<Index>,
    _name: IdString,
    _cell_type: IdString,
    _hierarchy_path: IdString,
    _udata: i32,

    ports: BTreeMap<IdString, PortInfo<DelayType, CellType>>,
    attributes: BTreeMap<IdString, Property>,
    parameters: BTreeMap<IdString, Property>,

    bel: BelId,
    _bel_strength: PlaceStrength,

    // cell is part of a cluster if != ClusterId
    _cluster: ClusterId,
}

impl<DelayType, CellType> const PartialEq for CellInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Error, Debug)]
pub enum CellError {
    #[error("Invalid port type for connect port.")]
    InvalidConnectPortType,
    #[error("Port is already connected.")]
    PortAlreadyConnected,
    #[error("Driver cell is in use.")]
    DriverCellInUse,
    #[error("Net Index not found.")]
    NetIndexNotFound,
}

impl<DelayType, CellType> CellInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    pub fn new() -> Self {
        Self {
            _arch_cell_info: ArchCellInfo::new(),
            _context: None,
            region: None,
            pseudo_cell: None,
            _name: IdString::new(),
            _cell_type: IdString::new(),
            _hierarchy_path: IdString::new(),
            _udata: 0,
            ports: BTreeMap::new(),
            attributes: BTreeMap::new(),
            parameters: BTreeMap::new(),
            bel: BelId::new(),
            _bel_strength: PlaceStrength::new(),
            _cluster: IdString::new(),
            self_index: None,
        }
    }
    pub fn with_arena(
        self_arena: &mut Arena<Self>,
        ctx_arena: &mut Arena<Context>,
        region_arena: &mut Arena<Region>,
    ) -> Index {
        let n = Self {
            _context: Some(ctx_arena.insert(Context::new())),
            region: Some(region_arena.insert(Region::new())),
            ..Default::default()
        };
        let index = self_arena.insert(n);
        self_arena.get_mut(index).unwrap().self_index = Some(index);
        index
    }
    pub fn add_input(&mut self, name: IdString) {
        self.ports.insert(
            name,
            PortInfo {
                name,
                port_type: PortType::In,
                net: self.ports[&name].net,
                user_index: self.ports[&name].user_index,
                ..Default::default()
            },
        );
    }
    pub fn add_output(&mut self, name: IdString) {
        self.ports.insert(
            name,
            PortInfo {
                name,
                port_type: PortType::Out,
                net: self.ports[&name].net,
                user_index: self.ports[&name].user_index,
                ..Default::default()
            },
        );
    }
    pub fn add_in_out(&mut self, name: IdString) {
        self.ports.insert(
            name,
            PortInfo {
                name,
                port_type: PortType::InOut,
                net: self.ports[&name].net,
                user_index: self.ports[&name].user_index,
                ..Default::default()
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
    pub fn test_region(&self, bel: BelId, region_arena: &mut Arena<Region>) -> bool {
        if let Some(region) = &self.region {
            let reg = region_arena.get(*region).unwrap();
            reg.constr_bels || reg.bels.contains_key(&bel)
        //            region.constr_bels || region.bels.contains_key(&bel)
        } else {
            true
        }
    }

    pub const fn is_pseudo(&self, _bel: BelId) -> bool {
        self.pseudo_cell.is_some()
    }

    pub fn get_location(&self, _bel: BelId, _pcell_arena: &mut Arena<CellType>) -> Loc
    where
        CellType: ~const CellTrait<DelayType> + ~const PseudoCell<DelayType>,
    {
        if let Some(_pseudo_cell) = &self.pseudo_cell {
            //            pseudo_cell.get_location()
            todo!()
        } else {
            assert!(self.bel != BelId::new());
            todo!()
            //  self.context.get_bel_location(bel)
        }
    }

    pub fn get_port(&self, name: IdString) -> Option<Index> {
        let found = self.ports.get(&name);
        if let Some(found_port) = found {
            found_port.net
        } else {
            None
        }
    }

    pub fn connect_port(
        &mut self,
        port_name: IdString,
        //        net: &NetInfo<DelayType, CellType>,
        net: Index,
        net_arena: &mut Arena<NetInfo<DelayType, CellType>>,
    ) -> Result<(), CellError> {
        // Get the port from our btree mapping that matches the passed in port_name value.
        // If there's nothing there just return the default value.
        let port = self.ports.entry(port_name).or_default();
        // Get the index of the net of the port we just looked up.
        let net_index = port.net.ok_or(CellError::NetIndexNotFound)?;
        // Lookup the existing net using this net index, and the net of the index passed in.
        let (self_net, passed_net) = net_arena.get2_mut(net_index, net);
        // Unwrap out net mapping to the port's net, and the net mapping to the passed in net index.
        let self_net = self_net.ok_or(CellError::NetIndexNotFound)?;
        let passed_net = passed_net.ok_or(CellError::NetIndexNotFound)?;
        // Check if the net mapped to the port at the given port_name is the default value.
        if *self_net == Default::default() {
            // Update this cell's port's net index with the passed in net index.
            port.net = Some(net);
            match port.port_type {
                PortType::Out => {
                    // Update the passed in net now since we reassigned self's port's to it alraady.
                    if passed_net.driver.cell.is_none() {
                        passed_net.driver.cell = self.self_index;
                        passed_net.driver.port = port_name;
                        Ok(())
                    } else {
                        Err(CellError::DriverCellInUse)
                    }
                }
                PortType::In | PortType::InOut => {
                    let mut user: PortRef<DelayType, CellType> = PortRef::new();
                    user.cell = self.self_index;
                    user.port = port_name;
                    port.user_index = passed_net.users.push_and_get_key(user);
                    Ok(())
                }
            }
        } else {
            Err(CellError::PortAlreadyConnected)
        }
    }

    pub fn disconnect_port(&mut self, _port: IdString) {
        todo!()
    }

    pub fn connect_ports(
        &mut self,
        _port: IdString,
        _other: &CellInfo<DelayType, CellType>,
        _other_port: IdString,
    ) {
        todo!()
    }

    pub fn move_port_to(
        &mut self,
        _port: IdString,
        _other: &CellInfo<DelayType, CellType>,
        _other_port: IdString,
    ) {
        todo!()
    }

    pub fn rename_port(&mut self, _old_name: IdString, _new_name: IdString) {
        todo!()
    }

    pub fn move_port_bus_to(
        &mut self,
        _old_port_bus: PortBus,
        _new_cell: &CellInfo<DelayType, CellType>,
        _new_port_bus: PortBus,
        _width: i32,
    ) {
        todo!()
    }

    pub fn copy_port_(
        &self,
        _port: IdString,
        _other: &CellInfo<DelayType, CellType>,
        _other_port: IdString,
    ) {
        todo!()
    }

    pub fn copy_port_bus_to(
        &self,
        _old_port_bus: PortBus,
        _new_cell: &CellInfo<DelayType, CellType>,
        _new_port_bus: PortBus,
        _width: i32,
    ) {
        todo!()
    }
}

impl<DelayType, CellType> Default for CellInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone, Eq)]
pub struct PortBus {
    name: IdString,
    offset: i32,
    brackets: bool,
}

impl Hash for PortBus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.offset.hash(state);
        self.brackets.hash(state);
    }
}

impl const PartialEq for PortBus {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.offset == other.offset && self.brackets == other.brackets
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

#[derive(Debug, Copy, Clone, Eq)]
pub struct ClockEvent {
    clock: IdString,
    edge: ClockEdge,
}

impl Hash for ClockEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.clock.hash(state);
        self.edge.hash(state);
    }
}

impl const PartialEq for ClockEvent {
    fn eq(&self, other: &Self) -> bool {
        self.clock == other.clock && self.edge == other.edge
    }
}

#[derive(Debug, Copy, Clone, Eq)]
pub struct ClockPair {
    start: ClockEvent,
    end: ClockEvent,
}

impl Hash for ClockPair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
    }
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
#[derive(Debug, Copy, Clone, Eq)]
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

impl<DelayType> Hash for NetSinkTiming<DelayType>
where
    DelayType: DelayTrait,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.clock_pair.hash(state);
        self.cell_port.hash(state);
        self.delay.hash(state);
        self.budget.hash(state);
    }
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

impl Context {
    pub const fn new() -> Self {
        todo!()
    }
}
