//#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
//pub struct CellInfo;

use super::{cell::CellInfo, delay::DelayTrait};

// The 'legacy' cluster data, used for existing arches and to provide a basic implementation for arches without complex
// clustering requirements
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct BaseClusterInfo<D: DelayTrait> {
    constr_children: Vec<CellInfo<D>>,
    constr_x: i64,      // this.x - parent.x
    constr_y: i64,      // this.y - parent.y
    constr_z: i64,      // this.z - parent.z
    constr_abs_z: bool, // parent.z := 0
}


impl<D> BaseClusterInfo<D> where D: DelayTrait {
    pub const fn new() -> Self {
        Self {
            constr_children: Vec::new(),
            constr_x: 0,
            constr_y: 0,
            constr_z: 0,
            constr_abs_z: false,
        }
    }
}
