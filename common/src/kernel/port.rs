use super::cell::CellInfo;
use super::net::UserId;
use crate::kernel::delay::{Delay, DelayTrait};
use crate::kernel::id_string::IdString;
use crate::kernel::net::NetInfo;
use std::hash::Hash;
use thunderdome::{Arena, Index};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq)]
pub struct PortRef<D>
where
    D: DelayTrait,
{
    /// An index into the arena of cells leading to the cell that a port is tied to.
    pub cell: Option<Index<CellInfo<D>>>,
    pub port: IdString,
    pub budget: Delay<D>,
}

impl<D> Hash for PortRef<D> where D:DelayTrait {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cell.hash(state);
        self.port.hash(state);
        self.budget.hash(state);
    }
}

impl<D> PartialEq for PortRef<D>
where
    D: DelayTrait,
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

impl<D> PortRef<D>
where
    D: DelayTrait,
{
    pub const fn new() -> Self
    where
        D: ~const DelayTrait,
    {
        Self {
            cell: None,
            port: IdString::new(),
            budget: Delay::new(),
        }
    }
}

impl<D> Default for PortRef<D>
where
    D: DelayTrait,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PortInfo<D>
where
    D: DelayTrait,
{
    pub name: IdString,
    //    net: Box<NetInfo<DelayType, CellType>>,
    //
    pub net: Option<Index<NetInfo<D>>>,
    pub port_type: PortType,
    pub user_index: Option<UserId>,
}

impl<D> PortInfo<D>
where
    D: DelayTrait,
//    CellType: CellTrait<DelayType>,
{
    pub const fn new() -> Self {
        Self {
            name: IdString::new(),
            //            net: Box::new(NetInfo::new()),
            net: None,
            port_type: PortType::In,
            user_index: None,
            //            _net_phantom: PhantomData,
        }
    }
    pub fn with_arena(_net_arena: &mut Arena<NetInfo<D>>) -> Self {
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

impl<D> const Default for PortInfo<D>
where
    D: DelayTrait,
{
    fn default() -> Self
//    where
//        D: ~const DelayTrait,
    {
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
