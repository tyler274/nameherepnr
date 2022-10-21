
#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Context {
    verbose: bool,
    debug: bool,
    force: bool,
    // Should we disable printing of the location of nets in the critical path?
    disable_critical_path_source_print: bool,
    // True when detailed per-net timing is to be stored / reported
    detailed_timing_report: bool,
}

impl Context {
    pub const fn new() -> Self {
        Self {
            verbose: false,
            debug: false,
            force: false,
            disable_critical_path_source_print: false,
            detailed_timing_report: false,

        }
    }
}