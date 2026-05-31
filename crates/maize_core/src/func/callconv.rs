#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CallConv {
    C,
    Fast,
    Cold,
    PTXKernel,
    PTXDevice,
}

impl CallConv {
    pub fn to_llvm(&self) -> u32 {
        match self {
            Self::C => 0,
            Self::Fast => 8,
            Self::Cold => 9,
            Self::PTXKernel => 71,
            Self::PTXDevice => 72,
        }
    }
}
