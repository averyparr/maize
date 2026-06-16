use std::{cell::RefCell, collections::HashMap};

use inkwell::{
    AddressSpace,
    context::Context,
    module::{Linkage, Module},
    values::{BasicValue, GlobalValue},
};

use crate::{
    ContextRef,
    backend::{FnCtx, FnRef, ToCPU, untyped::ErasedFuncType},
    tipe::Ty,
    val::Val,
};

use super::untyped::UntypedFunc;

#[derive(Debug)]
pub struct LLVM {
    module: Module<'static>,
    strings: RefCell<HashMap<&'static str, GlobalValue<'static>>>,
    cpu: &'static dyn ToCPU,
}

impl PartialEq for LLVM {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module
    }
}

impl LLVM {
    pub fn cpu(&self) -> &dyn ToCPU {
        self.cpu
    }
    pub fn new_named(name: &str, cpu: &'static dyn ToCPU) -> Self {
        thread_local! {
            static CTX: &'static Context = Box::leak(Box::new(Context::create()));
        }
        let module = CTX.with(|c| c.create_module(name));
        let strings = RefCell::new(HashMap::new());
        Self {
            module,
            strings,
            cpu,
        }
    }
    pub fn new(cpu: &'static dyn ToCPU) -> Self {
        Self::new_named("module", cpu)
    }
    pub(crate) fn ctx(&self) -> ContextRef {
        self.module.get_context()
    }

    pub fn inner(&self) -> &Module<'static> {
        &self.module
    }
    pub fn module(&self) -> &Module<'static> {
        &self.module
    }
    pub(crate) fn insert_untyped_func(&self, name: &str, ty: ErasedFuncType) -> UntypedFunc {
        UntypedFunc(self.module.add_function(name, ty.0, None))
    }
    pub fn insert_str(
        &self,
        s: &str,
        name: &str,
        address_space: Option<AddressSpace>,
    ) -> GlobalValue<'static> {
        if let Some(v) = self.strings.borrow().get(s) {
            return *v;
        }
        let ctx = self.ctx();
        let i8_ty = ctx.i8_type();
        let ty = i8_ty.array_type((s.len() + 1).try_into().expect("usize -> u32 overflow"));
        let mut mapped_chars = s
            .bytes()
            .map(|b| ctx.i8_type().const_int(b.into(), false))
            .collect::<Vec<_>>();
        mapped_chars.push(ctx.i8_type().const_zero());
        let global = self.module().add_global(ty, address_space, name);
        global.set_initializer(&i8_ty.const_array(mapped_chars.as_slice()));
        global.set_linkage(Linkage::Internal);
        global.set_unnamed_addr(true);
        self.strings
            .borrow_mut()
            .insert(String::leak(String::from(s)), global);
        global
    }
    pub fn insert_global<T: Ty>(
        &self,
        name: &str,
        address_space: Option<AddressSpace>,
    ) -> GlobalValue<'static> {
        let ty = T::raw_ty(self.ctx());
        let global = self.module().add_global(ty, address_space, name);
        global
    }
    pub fn insert_const<T: Ty>(
        &self,
        v: Val<T>,
        name: &str,
        address_space: Option<AddressSpace>,
    ) -> GlobalValue<'static> {
        assert!(v.raw().0.is_const());
        let global = self.insert_global::<T>(name, address_space);
        global.set_initializer(&v.typed());
        global.set_linkage(Linkage::Internal);
        global.set_unnamed_addr(true);
        global
    }
}
