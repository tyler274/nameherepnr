use crate::kernel::base_context::BaseCtx;
use std::fmt;

#[derive(Debug, Copy, Clone, Hash, Ord, Eq)]
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

impl IdString {
    pub fn initialize_arch(ctx: &BaseCtx) {
        todo!()
    }
    pub fn initialize_add(ctx: &BaseCtx, s: &str, idx: u64) {
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
    pub fn set(&mut self, ctx: &BaseCtx, s: &str) {
        todo!()
    }

    pub fn with_ctx_str(ctx: &BaseCtx, s: &str) -> Self {
        let mut x = Self::new();
        x.set(ctx, s);
        x
    }

    pub fn to_string(&self, ctx: &BaseCtx) -> String {
        todo!()
//        ctx.idstring_idx_to_str.at(self.index)
    }

    pub fn empty(&self) -> bool {
        self.index == 0
    }
    pub const fn hash(&self) -> u64 {
        self.index
    }

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

impl Default for IdString {
    fn default() -> Self {
        Self::new()
    }
}

/// A wrapper around the tuple so I can implement const traits on it.
#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct IdPair(IdString, IdString);

impl const PartialEq for IdPair{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
