use super::net::UserId;
use crate::kernel::cell::CellTrait;
use crate::kernel::delay::{Delay, DelayTrait};
use crate::kernel::id_string::IdString;
use crate::kernel::net::NetInfo;
use std::hash::Hash;
use std::marker::PhantomData;
use thunderdome::{Arena, Index};

#[derive(Debug, Clone)]
pub struct PortRef<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    //    cell: Option<Weak<CellInfo<DelayType, CellType>>>,
    /// An index into the arena of cells leading to the cell that a port is tied to.
    pub cell: Option<Index>,
    /// A phantom data entry to ensure that the PortRef remains specialized to the cell it was instantiated on.
    /// Might want to track the lifetime of the relevant cell arena as well.
    cell_phantom: PhantomData<CellType>,
    pub port: IdString,
    pub budget: Delay<DelayType>,
}

impl<DelayType, CellType> PartialEq for PortRef<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
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

impl<DelayType, CellType> PortRef<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    pub const fn new() -> Self
    where
        DelayType: ~const DelayTrait,
    {
        Self {
            cell: None,
            cell_phantom: PhantomData,
            port: IdString::new(),
            budget: Delay::new(),
        }
    }
}

impl<DelayType, CellType> Default for PortRef<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, Eq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct PortInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    pub name: IdString,
    //    net: Box<NetInfo<DelayType, CellType>>,
    pub net: Option<Index>,
    pub _net_phantom: PhantomData<NetInfo<DelayType, CellType>>,
    pub port_type: PortType,
    pub user_index: Option<UserId>,
}

impl<DelayType, CellType> PortInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    pub fn new() -> Self {
        Self {
            name: IdString::new(),
            //            net: Box::new(NetInfo::new()),
            net: None,
            port_type: PortType::In,
            user_index: None,
            _net_phantom: PhantomData,
        }
    }
    pub fn with_arena(_net_arena: &mut Arena<NetInfo<DelayType, CellType>>) -> Self {
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

impl<DelayType: DelayTrait, CellType: CellTrait<DelayType>> Default
    for PortInfo<DelayType, CellType>
{
    fn default() -> Self
    where
        DelayType: ~const DelayTrait,
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
