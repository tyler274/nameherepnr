use core::cmp::{Ord, Ordering};
use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use core::marker::Destruct;
use core::ops::{Add, Sub};
use serde::{Serialize, Deserialize};

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

#[derive(Debug, Copy, Clone, Eq, Serialize, Deserialize)]
pub struct Delay<D>(D)
where
    D: DelayTrait;

impl<D> Hash for Delay<D>
where
    D: DelayTrait,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<D> Delay<D>
where
    D: DelayTrait,
{
    pub const fn new() -> Self
    where
        D: DelayTrait + ~const DelayTrait,
    {
        Delay(D::new())
    }
    pub const fn with_delay(value: D) -> Self
    where
        D: DelayTrait + ~const DelayTrait,
    {
        Delay(value)
    }
}

impl<D> const Ord for Delay<D>
where
    D: DelayTrait + ~const Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl const DelayTrait for i64 {
    fn new() -> Self {
        0
    }
}

impl const From<i64> for Delay<i64> {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl<D> const Add for Delay<D>
where
    D: DelayTrait + ~const Add<Output = D>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Delay(self.0 + rhs.0)
    }
}

impl<D> const Sub for Delay<D>
where
    D: DelayTrait + ~const Sub<Output = D>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<D> const PartialEq for Delay<D>
where
    D: DelayTrait + ~const PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<D> const PartialOrd for Delay<D>
where
    D: DelayTrait + ~const Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<D> const Default for Delay<D>
where
    D: DelayTrait + ~const DelayTrait,
{
    fn default() -> Self {
        Self::new()
    }
}

/// minimum and maximum delay
#[derive(Debug, Copy, Clone, Eq, Serialize, Deserialize)]
pub struct DelayPair<D>
where
    D: DelayTrait,
{
    min_delay: Delay<D>,
    max_delay: Delay<D>,
}

impl<D> Hash for DelayPair<D>
where
    D: DelayTrait,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.min_delay.hash(state);
        self.max_delay.hash(state);
    }
}

impl<D> DelayPair<D>
where
    D: DelayTrait,
{
    pub const fn new() -> Self
    where
        D: DelayTrait + ~const DelayTrait,
    {
        Self {
            min_delay: Delay::new(),
            max_delay: Delay::new(),
        }
    }
    pub const fn with_delay(delay: Delay<D>) -> Self {
        Self {
            min_delay: delay,
            max_delay: delay,
        }
    }

    // Does there need to be an invariant such that `min_delay` is
    // always lower than `max_delay`
    pub const fn with_min_max(min_delay: Delay<D>, max_delay: Delay<D>) -> Self {
        Self {
            min_delay,
            max_delay,
        }
    }

    pub const fn min_delay(&self) -> Delay<D> {
        self.min_delay
    }

    pub const fn max_delay(&self) -> Delay<D> {
        self.max_delay
    }
}

impl<D> Default for DelayPair<D>
where
    D: DelayTrait,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<D> const Add<DelayPair<D>> for DelayPair<D>
where
    D: DelayTrait + ~const Add<Output = D>,
{
    type Output = DelayPair<D>;

    fn add(self, rhs: DelayPair<D>) -> Self::Output {
        DelayPair {
            min_delay: self.min_delay + rhs.min_delay,
            max_delay: self.max_delay + rhs.max_delay,
        }
    }
}

impl<D> const Sub for DelayPair<D>
where
    D: DelayTrait + ~const Sub<Output = D>,
{
    type Output = Self;

    fn sub(self, rhs: DelayPair<D>) -> Self::Output {
        DelayPair {
            min_delay: self.min_delay - rhs.min_delay,
            max_delay: self.max_delay - rhs.max_delay,
        }
    }
}

impl<D> const PartialEq for DelayPair<D>
where
    D: DelayTrait + ~const PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.min_delay == other.min_delay && self.max_delay == other.max_delay
    }
}

/// four-quadrant, min and max rise and fall delay
#[derive(Debug, Copy, Clone, Eq, Serialize, Deserialize)]
pub struct DelayQuad<D>
where
    D: DelayTrait,
{
    rise: DelayPair<D>,
    fall: DelayPair<D>,
}

impl<D> Hash for DelayQuad<D>
where
    D: DelayTrait,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rise.hash(state);
        self.fall.hash(state);
    }
}

impl<D> const PartialEq for DelayQuad<D>
where
    D: DelayTrait + ~const PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.rise == other.rise && self.fall == other.fall
    }
}

impl<D> const PartialOrd for DelayQuad<D>
where
    D: DelayTrait + ~const PartialOrd,
{
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl<D> DelayQuad<D>
where
    D: DelayTrait,
{
    pub const fn new() -> Self
    where
        D: DelayTrait + ~const DelayTrait,
    {
        Self {
            rise: DelayPair::new(),
            fall: DelayPair::new(),
        }
    }

    pub const fn with_delay(delay: Delay<D>) -> Self {
        Self {
            rise: DelayPair::with_delay(delay),
            fall: DelayPair::with_delay(delay),
        }
    }

    pub const fn with_min_max(min_delay: Delay<D>, max_delay: Delay<D>) -> Self {
        Self {
            rise: DelayPair::with_min_max(min_delay, max_delay),
            fall: DelayPair::with_min_max(min_delay, max_delay),
        }
    }

    pub const fn with_rise_fall(rise: DelayPair<D>, fall: DelayPair<D>) -> Self {
        Self { rise, fall }
    }

    pub const fn with_rise_fall_min_max(
        min_rise: Delay<D>,
        max_rise: Delay<D>,
        min_fall: Delay<D>,
        max_fall: Delay<D>,
    ) -> Self {
        Self {
            rise: DelayPair::with_min_max(min_rise, max_rise),
            fall: DelayPair::with_min_max(min_fall, max_fall),
        }
    }

    pub const fn min_rise_delay(&self) -> Delay<D> {
        self.rise.min_delay()
    }

    pub const fn max_rise_delay(&self) -> Delay<D> {
        self.rise.max_delay()
    }

    pub const fn min_fall_felay(&self) -> Delay<D> {
        self.fall.min_delay()
    }

    pub const fn max_fall_delay(&self) -> Delay<D> {
        self.fall.max_delay()
    }

    pub const fn min_delay(&self) -> Delay<D>
    where
        D: DelayTrait + ~const Ord + ~const Destruct,
    {
        std::cmp::min(self.rise.min_delay(), self.fall.min_delay())
    }

    pub const fn max_delay(&self) -> Delay<D>
    where
        D: DelayTrait + ~const Ord + ~const Destruct,
    {
        std::cmp::max(self.rise.max_delay(), self.fall.max_delay())
    }

    pub const fn delay_pair(&self) -> DelayPair<D>
    where
        D: DelayTrait + ~const Ord + ~const Destruct,
    {
        DelayPair::with_min_max(self.min_delay(), self.max_delay())
    }
}

impl<D> const Add<DelayQuad<D>> for DelayQuad<D>
where
    D: DelayTrait + ~const Add<Output = D>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        DelayQuad {
            rise: self.rise + rhs.rise,
            fall: self.fall + rhs.fall,
        }
    }
}

impl<D> const Sub<DelayQuad<D>> for DelayQuad<D>
where
    D: DelayTrait + ~const Sub<Output = D>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        DelayQuad {
            rise: self.rise - rhs.rise,
            fall: self.fall - rhs.fall,
        }
    }
}

impl<D> const Default for DelayQuad<D>
where
    D: DelayTrait + ~const Destruct + ~const Ord + ~const DelayTrait,
{
    fn default() -> Self {
        Self::new()
    }
}
