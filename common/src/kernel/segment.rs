use super::{delay::{DelayTrait, Delay}, id_string::{IdPair, IdString}};
use core::hash::Hash;

/// Segment type
#[derive(Debug, Copy, Clone, Eq)]
pub enum SegmentType {
    ClkToQ,  // Clock-to-Q delay
    Source,  // Delayless source
    Logic,   // Combinational logic delay
    Routing, // Routing delay
    Setup,   // Setup time in sink
}

impl Hash for SegmentType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (*self as usize).hash(state);
    }
}

impl const PartialEq for SegmentType {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}

#[derive(Debug, Copy, Clone, Eq)]
pub struct Segment<D: DelayTrait> {
    // Type
    segment_type: SegmentType,
    // Net name (routing only)
    net: IdString,
    // From cell.port
    from: IdPair,
    // To cell.port
    to: IdPair,
    // Segment delay
    delay: Delay<D>,
    // Segment budget (routing only)
    budget: Delay<D>,
}

impl<D> Hash for Segment<D> where D: DelayTrait {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.segment_type.hash(state);
        self.net.hash(state);
        self.from.hash(state);
        self.to.hash(state);
        self.delay.hash(state);
        self.budget.hash(state);
    }
}

impl<D> const PartialEq for Segment<D>
where
D: DelayTrait + ~const PartialEq,
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