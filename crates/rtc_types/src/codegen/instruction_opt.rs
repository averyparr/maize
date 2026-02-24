use std::u32;

use inkwell::values::InstructionValue;

#[derive(Clone, Copy)]
pub struct InstructionOpt {
    float_flags: u32,
}

impl Default for InstructionOpt {
    fn default() -> Self {
        Self { float_flags: 0 }
    }
}

#[allow(unused)]
const ASSUME_NO_NANS: u32 = 1 << 0;
#[allow(unused)]
const ASSUME_NO_INFS: u32 = 1 << 1;
#[allow(unused)]
const ASSUME_NEG_ZERO_IS_ZERO: u32 = 1 << 2;
#[allow(unused)]
const ALLOW_RCP_APRX: u32 = 1 << 3;
#[allow(unused)]
const ALLOW_FMA_CONTRACT: u32 = 1 << 4;
#[allow(unused)]
const ALLOW_APPROX_FNS: u32 = 1 << 5;
#[allow(unused)]
const ALLOW_REASSOC: u32 = 1 << 6;

impl InstructionOpt {
    pub fn post_process_instruction(&self, ins: InstructionValue<'_>) {
        ins.set_fast_math_flags(self.float_flags);
    }
    pub fn allow_approx_funcs(self) -> bool {
        (self.float_flags & ALLOW_APPROX_FNS) != 0
    }
    pub fn set_fast_math_flags(&mut self, flags: u32) {
        self.float_flags = flags;
    }
    pub fn use_all_fast_math(&mut self) {
        self.set_fast_math_flags(u32::MAX);
    }
}
