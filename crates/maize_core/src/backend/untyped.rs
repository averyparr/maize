use inkwell::{
    types::{BasicType, BasicTypeEnum, FunctionType},
    values::{BasicValueEnum, FunctionValue},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct UntypedValue(pub(crate) BasicValueEnum<'static>);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct ErasedType(pub(crate) BasicTypeEnum<'static>);
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(transparent)]
pub struct UntypedFunc(pub(crate) FunctionValue<'static>);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct ErasedFuncType(pub(crate) FunctionType<'static>);

impl ErasedType {
    pub(crate) fn func_type(&self, args: &[ErasedType]) -> ErasedFuncType {
        // Safety: repr-transparent
        let param_types = unsafe { std::mem::transmute(args) };
        ErasedFuncType(self.0.fn_type(param_types, false))
    }
}

impl UntypedValue {
    pub(crate) fn new(val: BasicValueEnum<'static>) -> Self {
        Self(val)
    }

    pub fn erased_type(&self) -> ErasedType {
        ErasedType(self.0.get_type())
    }
}

impl UntypedFunc {
    pub fn name(&self) -> &str {
        self.0
            .get_name()
            .to_str()
            .expect("Function names should be utf-8 encoded")
    }
}
