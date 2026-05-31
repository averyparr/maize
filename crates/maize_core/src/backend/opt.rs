use inkwell::OptimizationLevel;

#[derive(Clone, Copy)]
pub enum Opt {
    O0,
    O1,
    O2,
    O3,
}

impl Opt {
    pub fn as_optimization_level(&self) -> OptimizationLevel {
        match self {
            Opt::O0 => OptimizationLevel::None,
            Opt::O1 => OptimizationLevel::Less,
            Opt::O2 => OptimizationLevel::Default,
            Opt::O3 => OptimizationLevel::Aggressive,
        }
    }
    pub fn default_passes(&self) -> &'static str {
        match self {
            Opt::O0 => "default<O0>",
            Opt::O1 => "default<O1>",
            Opt::O2 => "default<O2>",
            Opt::O3 => "default<O3>",
        }
    }
}
