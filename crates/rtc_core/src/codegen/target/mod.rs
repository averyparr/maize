mod cuda;

pub use cuda::{PTXOptions, SM};

pub enum TargetMachine {
    PTX(PTXOptions),
}

impl TargetMachine {
    pub fn triple(&self) -> &'static str {
        match self {
            Self::PTX(_) => "nvptx64-nvidia-cuda",
        }
    }
    pub fn cpu(&self) -> &'static str {
        match self {
            Self::PTX(ptx) => ptx.cpu(),
        }
    }
    pub fn features(&self) -> &'static str {
        match self {
            Self::PTX(ptx) => ptx.features(),
        }
    }
}
