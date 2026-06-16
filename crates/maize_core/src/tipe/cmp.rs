use inkwell::{
    FloatPredicate, IntPredicate,
    builder::BuilderError,
    values::{BasicValue, FloatMathValue, IntMathValue},
};

use crate::{backend::UntypedValue, tipe::Ty, val::Val};

impl<T> Val<T>
where
    T: Ty,
    T::LLVal: IntMathValue<'static>,
{
    fn try_icmp(self, rhs: Self, op: IntPredicate) -> Result<Val<bool>, BuilderError> {
        let fn_ref = self.fn_ref().clone();
        // Safety: we are going to just build an int compare which is safe given these
        // two values existence
        let b = unsafe { fn_ref.curr_bb_builder() };
        let lhs = self.typed();
        let rhs = rhs.typed();
        let ret = b
            .build_int_compare(op, lhs, rhs, "icmp")?
            .as_basic_value_enum();
        let ret = UntypedValue::new(ret);
        Ok(unsafe { Val::new(fn_ref, ret) })
    }

    fn icmp(self, rhs: Self, op: IntPredicate) -> Val<bool> {
        self.try_icmp(rhs, op)
            .expect("Should be able to build icmp")
    }
}

impl<T> Val<T>
where
    T: Ty,
    T::LLVal: FloatMathValue<'static>,
{
    fn try_fcmp(self, rhs: Self, op: FloatPredicate) -> Result<Val<bool>, BuilderError> {
        let fn_ref = self.fn_ref().clone();
        // Safety: we are going to just build an int compare which is safe given these
        // two values existence
        let b = unsafe { fn_ref.curr_bb_builder() };
        let lhs = self.typed();
        let rhs = rhs.typed();
        let ret = b
            .build_float_compare(op, lhs, rhs, "icmp")?
            .as_basic_value_enum();
        let ret = UntypedValue::new(ret);
        Ok(unsafe { Val::new(fn_ref, ret) })
    }

    fn fcmp(self, rhs: Self, op: FloatPredicate) -> Val<bool> {
        self.try_fcmp(rhs, op)
            .expect("Should be able to build icmp")
    }
}

macro_rules! impl_cmp {
    (signed: $($tipes: ty),*) => {
        $(
            impl Val<$tipes> {
                pub fn eq(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::EQ)
                }
                pub fn ne(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::NE)
                }
                pub fn lt(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::SLT)
                }
                pub fn le(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::SLE)
                }
                pub fn gt(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::SGT)
                }
                pub fn ge(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::SGE)
                }
                pub fn eq_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::EQ)
                }
                pub fn ne_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::NE)
                }
                pub fn lt_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::SLT)
                }
                pub fn le_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::SLE)
                }
                pub fn gt_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::SGT)
                }
                pub fn ge_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::SGE)
                }
            }
        )*
    };
    (unsigned: $($tipes: ty),*) => {
        $(
            impl Val<$tipes> {
                pub fn eq(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::EQ)
                }
                pub fn ne(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::NE)
                }
                pub fn lt(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::ULT)
                }
                pub fn le(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::ULE)
                }
                pub fn gt(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::UGT)
                }
                pub fn ge(self, rhs: Self) -> Val<bool> {
                    self.icmp(rhs, IntPredicate::UGE)
                }

                pub fn eq_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::EQ)
                }
                pub fn ne_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::NE)
                }
                pub fn lt_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::ULT)
                }
                pub fn le_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::ULE)
                }
                pub fn gt_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::UGT)
                }
                pub fn ge_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.icmp(rhs, IntPredicate::UGE)
                }
            }
        )*
    };
    (float: $($tipes: ty),*) => {
        $(
            impl Val<$tipes> {
                pub fn eq(self, rhs: Self) -> Val<bool> {
                    self.fcmp(rhs, FloatPredicate::UEQ)
                }
                pub fn ne(self, rhs: Self) -> Val<bool> {
                    self.fcmp(rhs, FloatPredicate::UNE)
                }
                pub fn le(self, rhs: Self) -> Val<bool> {
                    self.fcmp(rhs, FloatPredicate::ULE)
                }
                pub fn lt(self, rhs: Self) -> Val<bool> {
                    self.fcmp(rhs, FloatPredicate::ULT)
                }
                pub fn ge(self, rhs: Self) -> Val<bool> {
                    self.fcmp(rhs, FloatPredicate::UGE)
                }
                pub fn gt(self, rhs: Self) -> Val<bool> {
                    self.fcmp(rhs, FloatPredicate::UGT)
                }

                pub fn eq_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.fcmp(rhs, FloatPredicate::UEQ)
                }
                pub fn ne_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.fcmp(rhs, FloatPredicate::UNE)
                }
                pub fn le_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.fcmp(rhs, FloatPredicate::ULE)
                }
                pub fn lt_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.fcmp(rhs, FloatPredicate::ULT)
                }
                pub fn ge_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.fcmp(rhs, FloatPredicate::UGE)
                }
                pub fn gt_const(self, rhs: $tipes) -> Val<bool> {
                    let rhs = self.constant(rhs);
                    self.fcmp(rhs, FloatPredicate::UGT)
                }
            }
        )*
    };
}

impl_cmp!(signed: i8, i16, i32, i64);
impl_cmp!(unsigned: u8, u16, u32, u64);
impl_cmp!(float: f32, f64);
