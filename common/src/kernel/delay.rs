use crate::ice40::arch_defs::{
    ArchCellInfo, ArchNetInfo, BelId, ClusterId, DecalId, Delay, PipId, WireId,
};
use std::ops;
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