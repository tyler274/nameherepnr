use ordered_float::NotNan;

use crate::kernel::delay::{DelayPair, DelayQuad, DelayTrait};
use crate::kernel::id_string::IdString;
use std::collections::BTreeMap;
use std::hash::Hash;

use super::delay::Delay;
use super::id_string::IdPair;
use super::segment::Segment;

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
pub struct TimingResult<DelayType>
where
    DelayType: DelayTrait,
{
    // Achieved and target Fmax for all clock domains
    clock_fmax: BTreeMap<IdString, ClockFmax>,
    // Single domain critical paths
    clock_paths: BTreeMap<IdString, CriticalPath<DelayType>>,
    // Cross-domain critical paths
    xclock_paths: Vec<CriticalPath<DelayType>>,

    // Detailed net timing data
    detailed_net_timings: BTreeMap<IdString, Vec<NetSinkTiming<DelayType>>>,
}

impl<D> TimingResult<D> where D: DelayTrait {
    pub const fn new() -> Self {
        Self {
            clock_fmax: BTreeMap::new(),
            clock_paths: BTreeMap::new(),
            xclock_paths: Vec::new(),
            detailed_net_timings: BTreeMap::new(),
        }
    }
}

impl<D> const Default for TimingResult<D>
where D: DelayTrait
{
    fn default() -> Self {
        Self::new()
    }
}
