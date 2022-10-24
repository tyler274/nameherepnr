use super::delay::DelayTrait;
use crate::kernel::base_context::BaseCtx;
use core::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, Serialize, Deserialize)]
pub struct IdString {
    index: u64,
}

impl const PartialEq for IdString {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl const PartialOrd for IdString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.index.cmp(&other.index))
    }
}

impl const Ord for IdString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl Hash for IdString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl IdString {
    pub fn initialize_arch<D: DelayTrait>(_ctx: &BaseCtx<D>) {
        todo!()
    }
    pub fn initialize_add<D: DelayTrait>(_ctx: &BaseCtx<D>, _s: &str, _idx: u64) {
        todo!()
    }
    pub const fn new() -> Self {
        Self { index: 0 }
    }
    pub const fn with_index(index: u64) -> Self {
        let mut x = Self::new();
        x.index = index;
        x
    }
    pub fn set<D: DelayTrait>(&mut self, _ctx: &BaseCtx<D>, _s: &str) {
        todo!()
    }

    pub fn with_ctx_str<D: DelayTrait>(ctx: &BaseCtx<D>, s: &str) -> Self {
        let mut x = Self::new();
        x.set(ctx, s);
        x
    }

    pub fn to_string<D: DelayTrait>(&self, _ctx: &BaseCtx<D>) -> String {
        todo!()
        //        ctx.idstring_idx_to_str.at(self.index)
    }

    pub fn empty(&self) -> bool {
        self.index == 0
    }
    //    pub const fn hash(&self) -> u64 {
    //        self.index
    //    }

    pub const fn equals(&self, other: &Self) -> bool {
        self.index == other.index
    }

    pub const fn inside(&self, rhs: &Self) -> bool {
        self.equals(rhs)
    }

    pub const fn inside_list(&self, args: &[&IdString]) -> bool {
        //        args.iter().find(|&s| self.index == s.index).is_some()
        let mut result = false;
        let mut i = 0;
        loop {
            let s = args[i];
            result = result || s.equals(self);
            i += 1;
            if i == args.len() {
                break;
            }
        }
        result
    }
}

impl const Default for IdString {
    fn default() -> Self {
        Self::new()
    }
}

/// A wrapper around the tuple so I can implement const traits on it.
#[derive(Debug, Copy, Clone, Eq, Serialize, Deserialize)]
pub struct IdPair(IdString, IdString);

impl const PartialEq for IdPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Hash for IdPair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}
