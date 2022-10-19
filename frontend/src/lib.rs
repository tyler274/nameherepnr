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
    is_top: bool,
    is_blackbox: bool,
    is_whitebox: bool,
    // inline bool is_box() const { return is_blackbox || is_whitebox; }
    instantiated_celltypes: Option<String>,
}

impl ModuleInfo {
    pub const fn new() -> Self {
        Self {
            is_top: false,
            is_blackbox: false,
            is_whitebox: false,
            instantiated_celltypes: None, // IdString
        }
    }
    fn is_box(&self) -> bool {
        self.is_blackbox || self.is_whitebox
    }
}
