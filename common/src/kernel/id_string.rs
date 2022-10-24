use super::delay::DelayTrait;
use crate::kernel::base_context::BaseCtx;
use core::hash::Hash;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum IdStringError {
    #[error("Index wasn't found in Base Context.")]
    IndexNotFoundInCtx,
}

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
    pub fn initialize_add<D: DelayTrait>(ctx: &mut BaseCtx<D>, s: &str, idx: u64) {
        assert!(ctx.idstring_str_to_idx.is_empty());
        assert!(ctx.idstring_idx_to_str.len() == idx as usize);
        ctx.idstring_str_to_idx.insert(s.to_string(), idx);
        ctx.idstring_idx_to_str.push(s.to_string());
    }
    pub const fn new() -> Self {
        Self { index: 0 }
    }
    pub const fn with_index(index: u64) -> Self {
        let mut x = Self::new();
        x.index = index;
        x
    }
    pub fn set<D: DelayTrait>(&mut self, ctx: &mut BaseCtx<D>, s: &str) {
        let it = ctx.idstring_str_to_idx.iter().position(|x| x.0 == s);
        if let Some(found) = it {
            self.index = found as u64;
        } else {
            self.index = ctx.idstring_idx_to_str.len() as u64;
            ctx.idstring_str_to_idx.insert(s.to_string(), self.index);
            ctx.idstring_idx_to_str.push(s.to_string());
        }
    }

    pub fn with_ctx_str<D: DelayTrait>(ctx: &mut BaseCtx<D>, s: &str) -> Self {
        let mut x = Self::new();
        x.set(ctx, s);
        x
    }

    pub fn to_string<D: DelayTrait>(&self, ctx: &BaseCtx<D>) -> Result<String, IdStringError> {
        // TODO: Cleanup the string references and lifetimes here and elsewhere in the file.
        Ok(ctx
            .idstring_idx_to_str
            .get(self.index as usize)
            .ok_or(IdStringError::IndexNotFoundInCtx)?
            .to_string())
    }

    pub fn is_empty(&self) -> bool {
        self.index == 0
    }

    pub const fn inside(&self, rhs: &Self) -> bool {
        self == rhs
    }

    pub const fn inside_list(&self, args: &[&IdString]) -> bool {
        let mut result = false;
        let mut i = 0;
        loop {
            let s = args[i];
            result = result || s == self;
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
