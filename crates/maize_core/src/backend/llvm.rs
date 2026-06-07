use inkwell::{
    AddressSpace,
    context::{Context, ContextRef},
    module::{Linkage, Module},
    values::{BasicValue, GlobalValue},
};

use crate::{
    backend::{FnCtx, FnRef, untyped::ErasedFuncType},
    tipe::Ty,
    val::Val,
};

use super::untyped::UntypedFunc;

#[derive(PartialEq, Debug)]
pub struct LLVM {
    module: Module<'static>,
}

impl LLVM {
    pub fn new_named(name: &str) -> Self {
        thread_local! {
            static CTX: &'static Context = Box::leak(Box::new(Context::create()));
        }
        let module = CTX.with(|c| c.create_module(name));
        Self { module }
    }
    pub fn new() -> Self {
        Self::new_named("module")
    }
    pub(crate) fn ctx(&self) -> ContextRef<'static> {
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
