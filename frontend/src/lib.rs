pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}

// Used for hierarchy resolution
pub struct ModuleInfo {
    _is_top: bool,
    _is_blackbox: bool,
    _is_whitebox: bool,
    // inline bool is_box() const { return is_blackbox || is_whitebox; }
    _instantiated_celltypes: Option<String>,
}

impl ModuleInfo {
    pub const fn new() -> Self {
        Self {
            _is_top: false,
            _is_blackbox: false,
            _is_whitebox: false,
            _instantiated_celltypes: None, // IdString
        }
    }
    fn _is_box(&self) -> bool {
        self._is_blackbox || self._is_whitebox
    }
}
