use thiserror::Error;
use thunderdome::{Arena, Index};

use crate::ice40::arch_defs::{ArchCellInfo, BelId, ClusterId, WireId};
use crate::kernel::{
    base_types::Loc,
    delay::{DelayQuad, DelayTrait},
    id_string::IdString,
    timing::{TimingClockingInfo, TimingPortClass},
};
use core::marker::PhantomData;
use std::collections::btree_map::Entry::Vacant;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;

use super::base_types::PlaceStrength;
use super::context::Context;
use super::net::NetInfo;
use super::port::{HierarchicalPort, PortBus, PortInfo, PortRef, PortType};
use super::property::Property;
use super::region::Region;

#[const_trait]
pub trait PseudoCell<D>
where
    D: DelayTrait,
    Self: Debug,
{
    fn get_location(&self) -> Loc {
        Loc::origin()
    }
    fn get_port_wire(&self, _port: IdString) -> Option<WireId> {
        None
    }
    fn get_delay(&self, _from_port: IdString, _to_port: IdString, _delay: &DelayQuad<D>) -> bool {
        false
    }
    fn get_port_timing_class(&self, _port: IdString, _clock_info_count: &u64) -> TimingPortClass {
        TimingPortClass::new()
    }
    fn get_port_clocking_info(&self, _port: IdString, _index: u64) -> TimingClockingInfo<D>
    where
        D: ~const DelayTrait,
    {
        TimingClockingInfo::new()
    }
}

// Unstable trait bounds aliasing, to reduce code duplication.
//#[const_trait]
//pub trait CellTrait<DelayType: DelayTrait>
//where
//    Self: Debug + PseudoCell<DelayType> + PartialEq + Clone,
//{
//}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionPlug {
    port_wires: BTreeMap<IdString, WireId>,
    loc: Loc,
}

impl RegionPlug {
    pub const fn new(loc: Loc) -> Self {
        Self {
            port_wires: BTreeMap::new(),
            loc,
        }
    }
}

impl<D: DelayTrait> PseudoCell<D> for RegionPlug {
    fn get_location(&self) -> Loc {
        self.loc
    }

    fn get_port_wire(&self, port: IdString) -> Option<WireId> {
        //        self.port_wires.get(&port).map(|wire| *wire)
        self.port_wires.get(&port).copied()
    }

    // TODO: partial reconfiguration region timing
    fn get_delay(&self, _from_port: IdString, _to_port: IdString, _delay: &DelayQuad<D>) -> bool {
        false
    }

    fn get_port_timing_class(&self, _port: IdString, _clock_info_count: &u64) -> TimingPortClass {
        TimingPortClass::Ignore
    }

    fn get_port_clocking_info(&self, _port: IdString, _index: u64) -> TimingClockingInfo<D>
    where
        D: ~const DelayTrait,
    {
        TimingClockingInfo::new()
    }
}

#[derive(Debug, Clone)]
pub struct CellInfo<D>
where
    D: DelayTrait,
{
    arch_cell_info: ArchCellInfo<D>,
    // Lets try using Arena indices for these.
    //    context: Option<Box<Context>>,
    //    region: Option<Box<Region>>,
    //    pseudo_cell: Option<std::rc::Rc<CellType>>,
    //    rc_self: Weak<Self>,

    // Index to the context within the Context arena.
    context: Option<Index<Context>>,
    // Index to the region within the Region arena.
    region: Option<Index<Region>>,
    // TODO: What is a pseudo cell really?
    //    pseudo_cell: Option<Box<dyn PseudoCell<D>>>,
    pseudo_cell: Option<Index>,
    // Index to this cell in the Arena
    self_index: Option<Index<Self>>,
    name: IdString,
    cell_type: IdString,
    hierarchy_path: IdString,
    udata: i32,

    ports: BTreeMap<IdString, PortInfo<D>>,
    attributes: BTreeMap<IdString, Property>,
    parameters: BTreeMap<IdString, Property>,

    bel: BelId,
    bel_strength: PlaceStrength,

    // cell is part of a cluster if != ClusterId
    cluster: ClusterId,
}

impl<D> Hash for CellInfo<D>
where
    D: DelayTrait,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.arch_cell_info.hash(state);
        self.context.hash(state);
        self.region.hash(state);
        self.pseudo_cell.hash(state);
        self.self_index.hash(state);
        self.name.hash(state);
        self.cell_type.hash(state);
        self.hierarchy_path.hash(state);
        self.udata.hash(state);
        self.ports.hash(state);
        self.attributes.hash(state);
        self.parameters.hash(state);
        self.bel.hash(state);
        self.bel_strength.hash(state);
        self.cluster.hash(state);
    }
}

impl<D> Eq for CellInfo<D> where D: DelayTrait {}

impl<D> PartialEq for CellInfo<D>
where
    D: DelayTrait,
{
    fn eq(&self, other: &Self) -> bool {
        self.arch_cell_info == other.arch_cell_info
            && self.context == other.context
            && self.region == other.region
            && self.pseudo_cell == other.pseudo_cell
            && self.self_index == other.self_index
            && self.name == other.name
            && self.cell_type == other.cell_type
            && self.hierarchy_path == other.hierarchy_path
            && self.udata == other.udata
            && self.ports == other.ports
            && self.attributes == other.attributes
            && self.parameters == other.parameters
            && self.bel == other.bel
            && self.bel_strength == other.bel_strength
    }
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum CellError {
    #[error("Invalid port type for connect port.")]
    InvalidConnectPortType,
    #[error("Port is already connected.")]
    PortAlreadyConnected,
    #[error("Driver cell is in use.")]
    DriverCellInUse,
    #[error("Net Index not found.")]
    NetIndexNotFound,
    #[error("Port not found.")]
    PortNotFound,
    // Why is it invalid?
    #[error("Moving and InOut port is invalid.")]
    PortInOutMoveInvalid,
    #[error("Tried to use a User Index that is None.")]
    UserIndexIsNone,
    #[error("User wasn't found in arena.")]
    UserNotFound,
    #[error("Tried to use a Net Index that is None.")]
    NetIndexIsNone,
}

impl<D> CellInfo<D>
where
    D: DelayTrait,
{
    pub fn new() -> Self {
        Self {
            arch_cell_info: ArchCellInfo::new(),
            context: None,
            region: None,
            pseudo_cell: None,
            name: IdString::new(),
            cell_type: IdString::new(),
            hierarchy_path: IdString::new(),
            udata: 0,
            ports: BTreeMap::new(),
            attributes: BTreeMap::new(),
            parameters: BTreeMap::new(),
            bel: BelId::new(),
            bel_strength: PlaceStrength::new(),
            cluster: IdString::new(),
            self_index: None,
        }
    }
    pub fn with_arena(
        self_arena: &mut Arena<Self, Self>,
        ctx_arena: &mut Arena<Context, Context>,
        region_arena: &mut Arena<Region, Region>,
    ) -> Index<Self> {
        let n = Self {
            context: Some(ctx_arena.insert(Context::new())),
            region: Some(region_arena.insert(Region::new())),
            ..Default::default()
        };
        let index = self_arena.insert(n);
        self_arena.get_mut(index).unwrap().self_index = Some(index);
        index
    }
    pub fn add_input(&mut self, name: IdString) {
        self.ports.insert(
            name,
            PortInfo {
                name,
                port_type: PortType::In,
                net: self.ports[&name].net,
                user_index: self.ports[&name].user_index,
            },
        );
    }
    pub fn add_output(&mut self, name: IdString) {
        self.ports.insert(
            name,
            PortInfo {
                name,
                port_type: PortType::Out,
                net: self.ports[&name].net,
                user_index: self.ports[&name].user_index,
            },
        );
    }
    pub fn add_in_out(&mut self, name: IdString) {
        self.ports.insert(
            name,
            PortInfo {
                name,
                port_type: PortType::InOut,
                net: self.ports[&name].net,
                user_index: self.ports[&name].user_index,
            },
        );
    }
    pub fn set_param(&mut self, name: IdString, value: Property) {
        self.parameters.insert(name, value);
    }
    pub fn unset_param(&mut self, name: IdString) {
        self.parameters.remove(&name);
    }
    pub fn set_attribute(&mut self, name: IdString, value: Property) {
        self.attributes.insert(name, value);
    }
    pub fn unset_attribute(&mut self, name: IdString) {
        self.attributes.remove(&name);
    }

    // check whether a bel complies with the cell's region constraint
    pub fn test_region(&self, bel: BelId, region_arena: &mut Arena<Region, Region>) -> bool {
        if let Some(region) = &self.region {
            let reg = region_arena.get(*region).unwrap();
            reg.constr_bels || reg.bels.contains_key(&bel)
            //            region.constr_bels || region.bels.contains_key(&bel)
        } else {
            true
        }
    }

    pub const fn is_pseudo(&self, _bel: BelId) -> bool {
        self.pseudo_cell.is_some()
    }

    pub fn get_location(
        &self,
        _bel: BelId,
        _pcell_arena: &mut Arena<CellInfo<D>, CellInfo<D>>,
    ) -> Loc {
        if let Some(_pseudo_cell) = &self.pseudo_cell {
            //            pseudo_cell.get_location()
            todo!()
        } else {
            assert!(self.bel != BelId::new());
            todo!()
            //  self.context.get_bel_location(bel)
        }
    }

    pub fn get_port(&self, name: IdString) -> Option<Index<NetInfo<D>>> {
        let found = self.ports.get(&name);
        if let Some(found_port) = found {
            found_port.net
        } else {
            None
        }
    }

    pub fn connect_port(
        &mut self,
        port_name: IdString,
        //        net: &NetInfo<DelayType, CellType>,
        net: Index<NetInfo<D>>,
        net_arena: &mut Arena<NetInfo<D>, NetInfo<D>>,
    ) -> Result<(), CellError> {
        // Get the port from our btree mapping that matches the passed in port_name value.
        // If there's nothing there just return the default value.
        let port = self.ports.entry(port_name).or_default();
        // Get the index of the net of the port we just looked up.
        let net_index = port.net.ok_or(CellError::NetIndexNotFound)?;
        // Lookup the existing net using this net index, and the net of the index passed in.
        let (self_net, passed_net) = net_arena.get2_mut(net_index, net);
        // Unwrap out net mapping to the port's net, and the net mapping to the passed in net index.
        let self_net = self_net.ok_or(CellError::NetIndexNotFound)?;
        let passed_net = passed_net.ok_or(CellError::NetIndexNotFound)?;
        // Check if the net mapped to the port at the given port_name is the default value.
        if *self_net == Default::default() {
            // Update this cell's port's net index with the passed in net index.
            port.net = Some(net);
            match port.port_type {
                PortType::Out => {
                    // Update the passed in net now since we reassigned self's port's to it alraady.
                    if passed_net.driver.cell.is_none() {
                        passed_net.driver.cell = self.self_index;
                        passed_net.driver.port = port_name;
                        Ok(())
                    } else {
                        Err(CellError::DriverCellInUse)
                    }
                }
                PortType::In | PortType::InOut => {
                    let mut user: PortRef<D> = PortRef::new();
                    user.cell = self.self_index;
                    user.port = port_name;
                    port.user_index = Some(passed_net.users.push_and_get_key(user));
                    Ok(())
                }
            }
        } else {
            Err(CellError::PortAlreadyConnected)
        }
    }

    pub fn disconnect_port(
        &mut self,
        port_name: IdString,
        net_arena: &mut Arena<NetInfo<D>, NetInfo<D>>,
    ) -> Result<(), CellError> {
        if !self.ports.contains_key(&port_name) {
            let mut port = self
                .ports
                .get_mut(&port_name)
                .ok_or(CellError::PortNotFound)?;
            if let Some(net_index) = port.net {
                let mut net = net_arena
                    .get_mut(net_index)
                    .ok_or(CellError::NetIndexNotFound)?;
                if let Some(user_idx) = port.user_index {
                    net.users.remove(user_idx);
                }
                if net.driver.cell == self.self_index && net.driver.port == port_name {
                    net.driver.cell = None;
                }
                port.net = None;
            }
        }
        Ok(())
    }

    pub fn connect_ports(
        &mut self,
        port: IdString,
        other: &mut CellInfo<D>,
        other_port: IdString,
        net_arena: &mut Arena<NetInfo<D>, NetInfo<D>>,
    ) -> Result<(), CellError> {
        let port_1 = self.ports.get_mut(&port).ok_or(CellError::PortNotFound)?;
        if let Some(p1_net) = port_1.net {
            other.connect_port(other_port, p1_net, net_arena)?
        } else {
            // No net on port1; need to create one
            todo!()
            //            let p1_net = self.context.create_net(self.context.id(self.name.to_string(self.context) + "$conn$" + port.to_string(self.context);
        }

        Ok(())
    }

    pub fn move_port_to(
        &mut self,
        port: IdString,
        other: &mut CellInfo<D>,
        other_port: IdString,
        net_arena: &mut Arena<NetInfo<D>, NetInfo<D>>,
    ) -> Result<(), CellError> {
        let mut old = self.ports.get_mut(&port).ok_or(CellError::PortNotFound)?;
        // Create port on the replacement cell if it doesn't already exist
        if let Vacant(e) = other.ports.entry(other_port) {
            e.insert(PortInfo {
                name: other_port,
                port_type: old.port_type,
                ..Default::default()
            });
        }

        let mut rep = other
            .ports
            .get_mut(&other_port)
            .ok_or(CellError::PortNotFound)?;

        assert!(old.port_type == rep.port_type);

        rep.net = old.net;
        rep.user_index = old.user_index;
        old.net = None;
        old.user_index = None;
        if let Some(r_net_id) = rep.net {
            let mut r_net = net_arena
                .get_mut(r_net_id)
                .ok_or(CellError::NetIndexNotFound)?;
            match rep.port_type {
                PortType::Out => {
                    r_net.driver.cell = other.self_index;
                    r_net.driver.port = other_port;
                    Ok(())
                }
                PortType::In => {
                    let mut load = r_net
                        .users
                        .get_mut(rep.user_index.ok_or(CellError::UserIndexIsNone)?)
                        .ok_or(CellError::UserNotFound)?;
                    load.cell = other.self_index;
                    load.port = other_port;
                    Ok(())
                }
                PortType::InOut => Err(CellError::PortInOutMoveInvalid),
            }
        } else {
            Ok(())
        }
    }

    pub fn rename_port(
        &mut self,
        old_name: IdString,
        new_name: IdString,
        net_arena: &mut Arena<NetInfo<D>, NetInfo<D>>,
    ) -> Result<(), CellError> {
        let mut old = self
            .ports
            .get(&old_name)
            .ok_or(CellError::PortNotFound)?
            .clone();
        if let Some(old_net_index) = old.net {
            let mut old_net = net_arena
                .get_mut(old_net_index)
                .ok_or(CellError::NetIndexNotFound)?;
            if old_net.driver.cell == self.self_index && old_net.driver.port == old_name {
                old_net.driver.port = new_name;
            }
            if let Some(user_index) = old.user_index {
                let mut user = old_net
                    .users
                    .get_mut(user_index)
                    .ok_or(CellError::UserNotFound)?;
                user.port = new_name;
            }
        }
        self.ports.remove(&old_name);
        old.name = new_name;
        self.ports
            .insert(new_name, old)
            .ok_or(CellError::PortAlreadyConnected)?;
        Ok(())
    }

    pub fn move_port_bus_to(
        &mut self,
        old_port_bus: PortBus,
        new_cell: &mut CellInfo<D>,
        new_port_bus: PortBus,
        width: i32,
        net_arena: &mut Arena<NetInfo<D>, NetInfo<D>>,
    ) -> Result<(), CellError> {
        for _i in 0..width {
            // FIXME: correct this after implementing Context.
            let old_port: IdString = IdString::new();
            let new_port: IdString = IdString::new();
            self.move_port_to(old_port, new_cell, new_port, net_arena)?;
            todo!()
        }
        Ok(())
    }

    pub fn copy_port_to(
        &mut self,
        port: IdString,
        other: &mut CellInfo<D>,
        other_port: IdString,
        net_arena: &mut Arena<NetInfo<D>, NetInfo<D>>,
    ) -> Result<(), CellError> {
        let self_port = self.ports.get(&port).ok_or(CellError::PortNotFound)?;

        let mut other_p = other
            .ports
            .get_mut(&other_port)
            .ok_or(CellError::PortNotFound)?;
        other_p.name = other_port;
        other_p.port_type = self_port.port_type;
        other.connect_port(
            other_port,
            self_port.net.ok_or(CellError::NetIndexIsNone)?,
            net_arena,
        )?;
        Ok(())
    }

    pub fn copy_port_bus_to(
        &mut self,
        old_port_bus: PortBus,
        new_cell: &mut CellInfo<D>,
        new_port_bus: PortBus,
        width: i32,
        net_arena: &mut Arena<NetInfo<D>, NetInfo<D>>,
    ) {
        todo!()
    }
}

impl<D> Default for CellInfo<D>
where
    D: DelayTrait,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct HierarchicalCell {
    name: IdString,
    cell_type: IdString,
    parent: IdString,
    fullpath: IdString,
    // Name inside cell instance -> global name
    leaf_cells: BTreeMap<IdString, IdString>,
    nets: BTreeMap<IdString, IdString>,
    // Global name -> name inside cell instance
    leaf_cells_by_global_name: BTreeMap<IdString, IdString>,
    nets_by_global_name: BTreeMap<IdString, IdString>,
    // Cell port to net
    ports: BTreeMap<IdString, HierarchicalPort>,
    // Name inside cell instance -> global name
    hierachical_cells: BTreeMap<IdString, IdString>,
}
