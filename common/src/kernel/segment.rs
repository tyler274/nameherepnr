use super::{delay::{DelayTrait, Delay}, id_string::{IdPair, IdString}};

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