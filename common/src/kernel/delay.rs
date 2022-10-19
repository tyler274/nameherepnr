use std::ops::{self, Add};

//use crate::kernel::base_clusterinfo::BaseClusterInfo;
//use crate::kernel::id_string::IdString;
//use hashers::oz::DJB2Hasher;
//use std::hash::{BuildHasher, BuildHasherDefault};
use num_traits::int::PrimInt;
use std::cmp::{Ord, Ordering};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::marker::Destruct;
use std::ops::{Deref, Sub};

/// The trait that all delays have to implement to fit within our model.
#[const_trait]
pub trait DelayTrait
where
    Self: Debug + Eq + PartialEq + Ord + PartialOrd + Copy + Clone + Sub + Add + Hash + Destruct,
{
    fn new() -> Self {
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct Delay<DelayType: DelayTrait>(DelayType);

impl<DelayType> Delay<DelayType>
where
    DelayType: DelayTrait,
{
    pub const fn new() -> Self
    where
        DelayType: DelayTrait + ~const DelayTrait,
    {
        Delay(DelayType::new())
    }
}

impl<DelayType: DelayTrait + ~const Ord> const Ord for Delay<DelayType> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl const DelayTrait for u64 {
    fn new() -> Self {
        0
    }
}

impl const From<u64> for Delay<u64> {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl<DelayType> const Add for Delay<DelayType>
where
    DelayType: DelayTrait + ~const Add<Output = DelayType>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Delay(self.0 + rhs.0)
    }
}

impl<DelayType> const Sub for Delay<DelayType>
where
    DelayType: DelayTrait + ~const Sub<Output = DelayType>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<DelayType> const PartialEq for Delay<DelayType>
where
    DelayType: DelayTrait + ~const PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<DelayType: DelayTrait + ~const Ord> const PartialOrd for Delay<DelayType> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<DelayType> const Default for Delay<DelayType>
where
    DelayType: DelayTrait + ~const DelayTrait,
{
    fn default() -> Self {
        Self::new()
    }
}

/// minimum and maximum delay
#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct DelayPair<DelayType: DelayTrait> {
    min_delay: Delay<DelayType>,
    max_delay: Delay<DelayType>,
}

impl<DelayType: DelayTrait> DelayPair<DelayType> {
    pub const fn new() -> Self
    where
        DelayType: DelayTrait + ~const DelayTrait,
    {
        Self {
            min_delay: Delay::new(),
            max_delay: Delay::new(),
        }
    }
    pub const fn with_delay(delay: Delay<DelayType>) -> Self {
        Self {
            min_delay: delay,
            max_delay: delay,
        }
    }

    // Does there need to be an invariant such that `min_delay` is
    // always lower than `max_delay`
    pub const fn with_min_max(min_delay: Delay<DelayType>, max_delay: Delay<DelayType>) -> Self {
        Self {
            min_delay,
            max_delay,
        }
    }

    pub const fn min_delay(&self) -> Delay<DelayType> {
        self.min_delay
    }

    pub const fn max_delay(&self) -> Delay<DelayType> {
        self.max_delay
    }
}

impl<DelayType: DelayTrait> Default for DelayPair<DelayType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<DelayType> const Add<DelayPair<DelayType>> for DelayPair<DelayType>
where
    DelayType: DelayTrait + ~const Add<Output = DelayType>,
{
    type Output = DelayPair<DelayType>;

    fn add(self, rhs: DelayPair<DelayType>) -> Self::Output {
        DelayPair {
            min_delay: self.min_delay + rhs.min_delay,
            max_delay: self.max_delay + rhs.max_delay,
        }
    }
}

impl<DelayType> const Sub for DelayPair<DelayType>
where
    DelayType: DelayTrait + ~const Sub<Output = DelayType>,
{
    type Output = Self;

    fn sub(self, rhs: DelayPair<DelayType>) -> Self::Output {
        DelayPair {
            min_delay: self.min_delay - rhs.min_delay,
            max_delay: self.max_delay - rhs.max_delay,
        }
    }
}

impl<DelayType: DelayTrait + ~const PartialEq> const PartialEq for DelayPair<DelayType> {
    fn eq(&self, other: &Self) -> bool {
        self.min_delay == other.min_delay && self.max_delay == other.max_delay
    }
}

/// four-quadrant, min and max rise and fall delay
#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct DelayQuad<DelayType>
where
    DelayType: DelayTrait,
{
    rise: DelayPair<DelayType>,
    fall: DelayPair<DelayType>,
}

impl<DelayType> const PartialEq for DelayQuad<DelayType>
where
    DelayType: DelayTrait + ~const PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.rise == other.rise && self.fall == other.fall
    }
}

impl<DelayType> const PartialOrd for DelayQuad<DelayType>
where
    DelayType: DelayTrait + ~const PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl<DelayType> DelayQuad<DelayType>
where
    DelayType: DelayTrait,
{
    pub const fn new() -> Self
    where
        DelayType: DelayTrait + ~const DelayTrait,
    {
        Self {
            rise: DelayPair::new(),
            fall: DelayPair::new(),
        }
    }

    pub const fn with_delay(delay: Delay<DelayType>) -> Self {
        Self {
            rise: DelayPair::with_delay(delay),
            fall: DelayPair::with_delay(delay),
        }
    }

    pub const fn with_min_max(min_delay: Delay<DelayType>, max_delay: Delay<DelayType>) -> Self {
        Self {
            rise: DelayPair::with_min_max(min_delay, max_delay),
            fall: DelayPair::with_min_max(min_delay, max_delay),
        }
    }

    pub const fn with_rise_fall(rise: DelayPair<DelayType>, fall: DelayPair<DelayType>) -> Self {
        Self { rise, fall }
    }

    pub const fn with_rise_fall_min_max(
        min_rise: Delay<DelayType>,
        max_rise: Delay<DelayType>,
        min_fall: Delay<DelayType>,
        max_fall: Delay<DelayType>,
    ) -> Self {
        Self {
            rise: DelayPair::with_min_max(min_rise, max_rise),
            fall: DelayPair::with_min_max(min_fall, max_fall),
        }
    }

    pub const fn min_rise_delay(&self) -> Delay<DelayType> {
        self.rise.min_delay()
    }

    pub const fn max_rise_delay(&self) -> Delay<DelayType> {
        self.rise.max_delay()
    }

    pub const fn min_fall_felay(&self) -> Delay<DelayType> {
        self.fall.min_delay()
    }

    pub const fn max_fall_delay(&self) -> Delay<DelayType> {
        self.fall.max_delay()
    }

    pub const fn min_delay(&self) -> Delay<DelayType>
    where
        DelayType: DelayTrait + ~const Ord + ~const Destruct,
    {
        std::cmp::min(self.rise.min_delay(), self.fall.min_delay())
    }

    pub const fn max_delay(&self) -> Delay<DelayType>
    where
        DelayType: DelayTrait + ~const Ord + ~const Destruct,
    {
        std::cmp::max(self.rise.max_delay(), self.fall.max_delay())
    }

    pub const fn delay_pair(&self) -> DelayPair<DelayType>
    where
        DelayType: DelayTrait + ~const Ord + ~const Destruct,
    {
        DelayPair::with_min_max(self.min_delay(), self.max_delay())
    }
}

impl<DelayType> const Add<DelayQuad<DelayType>> for DelayQuad<DelayType>
where
    DelayType: DelayTrait + ~const Add<Output = DelayType>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        DelayQuad {
            rise: self.rise + rhs.rise,
            fall: self.fall + rhs.fall,
        }
    }
}

impl<DelayType> const Sub<DelayQuad<DelayType>> for DelayQuad<DelayType>
where
    DelayType: DelayTrait + ~const Sub<Output = DelayType>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        DelayQuad {
            rise: self.rise - rhs.rise,
            fall: self.fall - rhs.fall,
        }
    }
}

impl<DelayType> const Default for DelayQuad<DelayType>
where
    DelayType: DelayTrait + ~const Destruct + ~const Ord + ~const DelayTrait,
{
    fn default() -> Self {
        Self::new()
    }
}
