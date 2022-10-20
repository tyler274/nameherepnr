use thiserror::Error;
use thunderdome::{Arena, Index};

use crate::ice40::arch_defs::{ArchCellInfo, BelId, ClusterId, WireId};
use crate::kernel::{
    base_types::Loc,
    delay::{DelayQuad, DelayTrait},
    id_string::IdString,
    timing::{TimingClockingInfo, TimingPortClass},
};
use std::collections::BTreeMap;
use std::fmt::Debug;

use super::base_types::PlaceStrength;
use super::context::Context;
use super::net::NetInfo;
use super::port::{HierarchicalPort, PortBus, PortInfo, PortRef, PortType};
use super::property::Property;
use super::region::Region;

#[const_trait]
pub trait PseudoCell<DelayType>
where
    DelayType: DelayTrait,
{
    fn get_location(&self) -> Loc {
        Loc::origin()
    }
    fn get_port_wire(&self, _port: IdString) -> Option<WireId> {
        None
    }
    fn get_delay(
        &self,
        _from_port: IdString,
        _to_port: IdString,
        _delay: &DelayQuad<DelayType>,
    ) -> bool {
        false
    }
    fn get_port_timing_class(&self, _port: IdString, _clock_info_count: &u64) -> TimingPortClass {
        TimingPortClass::new()
    }
    fn get_port_clocking_info(&self, _port: IdString, _index: u64) -> TimingClockingInfo<DelayType>
    where
        DelayType: ~const DelayTrait,
    {
        TimingClockingInfo::new()
    }
}

// Unstable trait bounds aliasing, to reduce code duplication.
#[const_trait]
pub trait CellTrait<DelayType: DelayTrait>
where
    Self: Debug + PseudoCell<DelayType> + PartialEq + Clone,
{
}

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

impl<DelayType: DelayTrait> PseudoCell<DelayType> for RegionPlug {
    fn get_location(&self) -> Loc {
        self.loc
    }

    fn get_port_wire(&self, port: IdString) -> Option<WireId> {
        //        self.port_wires.get(&port).map(|wire| *wire)
        self.port_wires.get(&port).copied()
    }

    // TODO: partial reconfiguration region timing
    fn get_delay(
        &self,
        _from_port: IdString,
        _to_port: IdString,
        _delay: &DelayQuad<DelayType>,
    ) -> bool {
        false
    }

    fn get_port_timing_class(&self, _port: IdString, _clock_info_count: &u64) -> TimingPortClass {
        TimingPortClass::Ignore
    }

    fn get_port_clocking_info(&self, _port: IdString, _index: u64) -> TimingClockingInfo<DelayType>
    where
        DelayType: ~const DelayTrait,
    {
        TimingClockingInfo::new()
    }
}

#[derive(Debug, Clone)]
pub struct CellInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    _arch_cell_info: ArchCellInfo,
    // Lets try using Arena indices for these.
    //    context: Option<Box<Context>>,
    //    region: Option<Box<Region>>,
    //    pseudo_cell: Option<std::rc::Rc<CellType>>,
    //    rc_self: Weak<Self>,

    // Index to the context within the Context arena.
    _context: Option<Index>,
    // Index to the region within the Region arena.
    region: Option<Index>,
    // TODO: What is a pseudo cell really?
    pseudo_cell: Option<Index>,
    // Index to this cell in the Arena
    self_index: Option<Index>,
    _name: IdString,
    _cell_type: IdString,
    _hierarchy_path: IdString,
    _udata: i32,

    ports: BTreeMap<IdString, PortInfo<DelayType, CellType>>,
    attributes: BTreeMap<IdString, Property>,
    parameters: BTreeMap<IdString, Property>,

    bel: BelId,
    _bel_strength: PlaceStrength,

    // cell is part of a cluster if != ClusterId
    _cluster: ClusterId,
}

impl<DelayType, CellType> const PartialEq for CellInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Error, Debug)]
pub enum CellError {
    #[error("Invalid port type for connect port.")]
    InvalidConnectPortType,
    #[error("Port is already connected.")]
    PortAlreadyConnected,
    #[error("Driver cell is in use.")]
    DriverCellInUse,
    #[error("Net Index not found.")]
    NetIndexNotFound,
}

impl<DelayType, CellType> CellInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
{
    pub fn new() -> Self {
        Self {
            _arch_cell_info: ArchCellInfo::new(),
            _context: None,
            region: None,
            pseudo_cell: None,
            _name: IdString::new(),
            _cell_type: IdString::new(),
            _hierarchy_path: IdString::new(),
            _udata: 0,
            ports: BTreeMap::new(),
            attributes: BTreeMap::new(),
            parameters: BTreeMap::new(),
            bel: BelId::new(),
            _bel_strength: PlaceStrength::new(),
            _cluster: IdString::new(),
            self_index: None,
        }
    }
    pub fn with_arena(
        self_arena: &mut Arena<Self>,
        ctx_arena: &mut Arena<Context>,
        region_arena: &mut Arena<Region>,
    ) -> Index {
        let n = Self {
            _context: Some(ctx_arena.insert(Context::new())),
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
                ..Default::default()
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
                ..Default::default()
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
                ..Default::default()
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
    pub fn test_region(&self, bel: BelId, region_arena: &mut Arena<Region>) -> bool {
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

    pub fn get_location(&self, _bel: BelId, _pcell_arena: &mut Arena<CellType>) -> Loc
    where
        CellType: ~const CellTrait<DelayType> + ~const PseudoCell<DelayType>,
    {
        if let Some(_pseudo_cell) = &self.pseudo_cell {
            //            pseudo_cell.get_location()
            todo!()
        } else {
            assert!(self.bel != BelId::new());
            todo!()
            //  self.context.get_bel_location(bel)
        }
    }

    pub fn get_port(&self, name: IdString) -> Option<Index> {
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
        net: Index,
        net_arena: &mut Arena<NetInfo<DelayType, CellType>>,
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
                    let mut user: PortRef<DelayType, CellType> = PortRef::new();
                    user.cell = self.self_index;
                    user.port = port_name;
                    port.user_index = passed_net.users.push_and_get_key(user);
                    Ok(())
                }
            }
        } else {
            Err(CellError::PortAlreadyConnected)
        }
    }

    pub fn disconnect_port(&mut self, _port: IdString) {
        todo!()
    }

    pub fn connect_ports(
        &mut self,
        _port: IdString,
        _other: &CellInfo<DelayType, CellType>,
        _other_port: IdString,
    ) {
        todo!()
    }

    pub fn move_port_to(
        &mut self,
        _port: IdString,
        _other: &CellInfo<DelayType, CellType>,
        _other_port: IdString,
    ) {
        todo!()
    }

    pub fn rename_port(&mut self, _old_name: IdString, _new_name: IdString) {
        todo!()
    }

    pub fn move_port_bus_to(
        &mut self,
        _old_port_bus: PortBus,
        _new_cell: &CellInfo<DelayType, CellType>,
        _new_port_bus: PortBus,
        _width: i32,
    ) {
        todo!()
    }

    pub fn copy_port_(
        &self,
        _port: IdString,
        _other: &CellInfo<DelayType, CellType>,
        _other_port: IdString,
    ) {
        todo!()
    }

    pub fn copy_port_bus_to(
        &self,
        _old_port_bus: PortBus,
        _new_cell: &CellInfo<DelayType, CellType>,
        _new_port_bus: PortBus,
        _width: i32,
    ) {
        todo!()
    }
}

impl<DelayType, CellType> Default for CellInfo<DelayType, CellType>
where
    DelayType: DelayTrait,
    CellType: CellTrait<DelayType>,
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