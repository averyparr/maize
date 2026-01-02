use inkwell::values::{BasicValue, PointerValue};

use crate::{
    traits::{
        holder::Holds,
        ptr::{MutTy, PtrTy, RefTy},
    },
    ty::{
        Ty,
        ptr::{Global, P, R},
    },
    val::Val,
};

impl<'lt, Ptr> Val<'lt, Ptr>
where
    Ptr: PtrTy,
{
    /// # Safety
    /// You must ensure that we hold a pointer loadable as-if a *mut T
    pub unsafe fn load_unchecked(&self) -> Val<'lt, Ptr::Pointee> {
        // Safety: User promised!
        unsafe { Val::new(self.cm(), Ptr::load_ptr_unchecked(self)) }
    }
    /// # Safety
    /// You must ensure we hold a pointer storeable as-if by *mut T
    pub unsafe fn store_unchecked<Holder: Holds<T = Ptr::Pointee>>(
        &self,
        to_store: Val<'lt, Holder>,
    ) {
        // Safety: User promised!
        let _ins = unsafe { Ptr::store_ptr_unchecked(self, to_store.get().to_underlying()) };
    }
}

impl<'lt, Ref> Val<'lt, Ref>
where
    Ref: RefTy<Value = PointerValue<'static>>,
{
    pub fn to_ptr(&self) -> Val<'lt, P<Ref::Pointee>> {
        // Safety: This is just a pointer decay
        unsafe { Val::new(self.cm(), self.to_underlying()) }
    }
}

impl<'lt, Ref> Val<'lt, Ref>
where
    Ref: RefTy,
{
    pub fn load(&self) -> Val<'lt, Ref::Pointee> {
        // Safety: We hold a shared reference
        unsafe { Val::new(self.cm(), Ref::load_ptr(self)) }
    }
}

impl<'lt, T> Val<'lt, Global<R<&T>>>
where
    T: Ty,
{
    /// I'm _fairly_ sure this is safe, so long as
    /// - We never allow you to convert Global\<Mut> -> Global\<Ref>
    /// as then we can be confident that no thread can write to
    /// the data you're reading from.
    pub fn load_nc(&self) -> Val<'lt, T> {
        let ctx = self.cm().cx().ctx();
        let val = self.load();
        if let Some(ins) = val.to_underlying().as_instruction_value() {
            let metadata_node = ctx.metadata_node(&[]);
            ins.set_metadata(metadata_node, ctx.get_kind_id("invariant.load"))
                .expect("Should be able to add invariant.load metadata");
        }
        val
    }
}

impl<'lt, Mut> Val<'lt, Mut>
where
    Mut: MutTy<Value = PointerValue<'static>>,
{
    pub fn to_ref(&self) -> Val<'lt, R<&Mut::Pointee>> {
        // Safety: We hold an exclusive reference so no one else can mutate
        unsafe { Val::new(self.cm(), self.to_underlying()) }
    }
}

impl<'lt, Mut> Val<'lt, Mut>
where
    Mut: MutTy,
{
    pub fn store<Holder: Holds<T = Mut::Pointee>>(&self, to_store: Val<'lt, Holder>) {
        // Safety: We hold an exclusive reference
        let _ins = unsafe { Mut::store(self, to_store.get().to_underlying()) };
    }
}
