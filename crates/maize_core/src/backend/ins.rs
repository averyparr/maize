use inkwell::values::{BasicValue, FastMathFlags};

use crate::backend::UntypedValue;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InstructionFlags {
    pub fast_math: FastMathFlags,
    pub signed_wrap: bool,
    pub unsigned_wrap: bool,
}

impl Default for InstructionFlags {
    fn default() -> Self {
        Self {
            fast_math: FastMathFlags::empty(),
            signed_wrap: false,
            unsigned_wrap: false,
        }
    }
}

impl InstructionFlags {
    pub fn apply_to_ins(&self, val: &mut UntypedValue) {
        if let Some(ins) = val.0.as_instruction_value() {
            if ins.can_use_fast_math_flags() {
                ins.set_fast_math_flags(self.fast_math)
                    .expect("Should be able to set...");
            }
            // if we can't set it, why would we care?
            let _ = ins.set_no_unsigned_wrap_flag(self.unsigned_wrap);
            let _ = ins.set_no_signed_wrap_flag(self.signed_wrap);
        }
    }
}
