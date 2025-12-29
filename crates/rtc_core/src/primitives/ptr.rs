use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use inkwell::{
    AddressSpace,
    context::ContextRef,
    values::{BasicValue, BasicValueEnum, InstructionValue, PointerValue},
};

use crate::{
    ty::{FromCtx, Global, M, P, R, Shared, Ty},
    val::{Holds, Val},
};

fn with_attrs<'ctx>(
    ctx: &'ctx ContextRef<'static>,
    attrs: &[&str],
) -> impl Fn(InstructionValue<'ctx>) {
    |ins| {
        for attr_str in attrs.into_iter() {
            let attr = ctx.get_kind_id(*attr_str);
            let metadata = ctx.metadata_node(&[]);
            ins.set_metadata(metadata, attr)
                .expect("Unable to set metadata");
        }
    }
}

pub trait RawPtrOps<'ctx> {
    type Pointee: Ty;
    fn pointee_ty(&self) -> Self::Pointee;
    /// # Safety:
    /// Treat this as identical to loading from a *mut T
    /// You must ensure that the underlying ctx lasts at least
    /// as long as `'ctx
    unsafe fn load_unchecked<'a>(
        &self,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) -> Val<'ctx, Self::Pointee>
    where
        'ctx: 'a;
    /// # Safety:
    /// Treat this as identical to storing to a *mut T
    unsafe fn store_unchecked<'a, Value>(
        &mut self,
        val: Value,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) where
        Value: Holds<T = Self::Pointee>,
        'ctx: 'a;
}

pub trait RefOps<'ctx>: RawPtrOps<'ctx> {
    fn load(&self) -> Val<'ctx, Self::Pointee>;
}

pub trait MutOps<'ctx>: RefOps<'ctx> {
    fn store<Value>(&mut self, val: Value)
    where
        Value: Holds<T = Self::Pointee>;
}

impl<'r, 'lt, T> Val<'lt, R<'r, T>>
where
    T: Ty,
{
    pub fn as_ptr(&self) -> Val<'lt, P<T>> {
        Val::new(self.cx(), self.to_underlying().as_basic_value_enum())
    }
}

impl<'m, 'lt, T> Val<'lt, M<'m, T>>
where
    T: Ty,
{
    pub fn as_ref(&self) -> Val<'lt, R<'m, T>> {
        Val::new(self.cx(), self.to_underlying().as_basic_value_enum())
    }
    pub fn as_ptr(&self) -> Val<'lt, P<T>> {
        self.as_ref().as_ptr()
    }

    pub fn inc(&mut self, val: Val<'lt, T>) {}
}

impl<'lt, T> RawPtrOps<'lt> for Val<'lt, P<T>>
where
    T: Ty,
{
    type Pointee = T;
    fn pointee_ty(&self) -> Self::Pointee {
        Self::Pointee::new(self.cx().ctx())
    }

    /// # Safety:
    /// See `RawPtrOps::load_unchecked`.
    unsafe fn load_unchecked<'a>(
        &self,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) -> Val<'lt, Self::Pointee>
    where
        'lt: 'a,
    {
        let pointee_ty = self.pointee_ty().basic_ty();
        let ptr = self.to_underlying();
        let cx = self.cx();
        // Safety: User promised the load is valid!
        let inner_val =
            unsafe { cx.load(pointee_ty, ptr, Some(<Self::Pointee as Ty>::ALIGN), for_ins) };
        // Safety: User promised 'ctx lasts as long as the underlying FnCodegen!
        let cx_extended = unsafe { self.cx_with_lifetime() };
        Val::new(cx_extended, inner_val)
    }
    unsafe fn store_unchecked<'a, Value>(
        &mut self,
        val: Value,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) where
        Value: Holds<T = Self::Pointee>,
        'lt: 'a,
    {
        let ptr_val = self.to_underlying();
        let value = val.to_underlying();
        let cx = self.cx();
        // Safety: User promised that storing to *mut T is valid
        unsafe { cx.store(ptr_val, value, Some(<Self::Pointee as Ty>::ALIGN), for_ins) };
    }
}

impl<'r, 'lt, T> RawPtrOps<'lt> for Val<'lt, R<'r, T>>
where
    T: Ty,
{
    type Pointee = T;
    fn pointee_ty(&self) -> Self::Pointee {
        self.as_ptr().pointee_ty()
    }
    unsafe fn load_unchecked<'a>(
        &self,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) -> Val<'lt, Self::Pointee>
    where
        'lt: 'a,
    {
        let ctx = self.cx().ctx();
        let with_readonly = with_attrs(&ctx, &["invariant.load", "noalias"]);
        if let Some(for_ins) = for_ins {
            let for_ins = |ins| {
                for_ins(ins);
                with_readonly(ins);
            };
            // SAFETY: We hold a shared reference
            unsafe { self.as_ptr().load_unchecked(Some(&for_ins)) }
        } else {
            // SAFETY: We hold a shared reference
            unsafe { self.as_ptr().load_unchecked(Some(&with_readonly)) }
        }
    }
    unsafe fn store_unchecked<'a, Value>(
        &mut self,
        val: Value,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) where
        Value: Holds<T = Self::Pointee>,
        'lt: 'a,
    {
        // SAFETY: User promised! Even though this makes no sense
        unsafe { self.as_ptr().store_unchecked(val, for_ins) }
    }
}

impl<'m, 'lt, T> RawPtrOps<'lt> for Val<'lt, M<'m, T>>
where
    T: Ty,
{
    type Pointee = T;
    fn pointee_ty(&self) -> Self::Pointee {
        self.as_ptr().pointee_ty()
    }
    unsafe fn load_unchecked<'a>(
        &self,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) -> Val<'lt, Self::Pointee>
    where
        'lt: 'a,
    {
        let ctx = self.cx().ctx();
        let with_noalias = with_attrs(&ctx, &["noalias"]);
        if let Some(for_ins) = for_ins {
            let for_ins = |ins| {
                for_ins(ins);
                with_noalias(ins);
            };
            unsafe { self.as_ptr().load_unchecked(Some(&for_ins)) }
        } else {
            unsafe { self.as_ptr().load_unchecked(Some(&with_noalias)) }
        }
    }
    unsafe fn store_unchecked<'a, Value>(
        &mut self,
        val: Value,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) where
        Value: Holds<T = Self::Pointee>,
        'lt: 'a,
    {
        let ctx = self.cx().ctx();
        let with_noalias = with_attrs(&ctx, &["noalias"]);
        if let Some(for_ins) = for_ins {
            let for_ins = |ins| {
                for_ins(ins);
                with_noalias(ins);
            };
            unsafe { self.store_unchecked(val, Some(&for_ins)) }
        } else {
            unsafe { self.as_ptr().store_unchecked(val, Some(&with_noalias)) }
        }
    }
}

impl<'r, 'lt, T> RefOps<'lt> for Val<'lt, R<'r, T>>
where
    T: Ty,
{
    fn load(&self) -> Val<'lt, Self::Pointee> {
        // Safety: We hold a shared reference
        unsafe { self.load_unchecked(None) }
    }
}

impl<'m, 'lt, T> RefOps<'lt> for Val<'lt, M<'m, T>>
where
    T: Ty,
{
    fn load(&self) -> Val<'lt, Self::Pointee> {
        // Safety: We hold an exclusive reference
        unsafe { self.load_unchecked(None) }
    }
}

impl<'m, 'lt, T> MutOps<'lt> for Val<'lt, M<'m, T>>
where
    T: Ty,
{
    fn store<Value>(&mut self, val: Value)
    where
        Value: Holds<T = Self::Pointee>,
    {
        // Safety: We hold an exclusive reference
        unsafe { self.store_unchecked(val, None) }
    }
}

macro_rules! impl_ptr_wrapper {
    ($wrapper_name: ident) => {
        impl<'lt, Ptr> RawPtrOps<'lt> for Val<'lt, $wrapper_name<Ptr>>
        where
            Val<'lt, Ptr>: RawPtrOps<'lt>,
            Ptr: Ty,
        {
            type Pointee = <Val<'lt, Ptr> as RawPtrOps<'lt>>::Pointee;
            fn pointee_ty(&self) -> Self::Pointee {
                self.to_inner().pointee_ty()
            }
            unsafe fn load_unchecked<'a>(
                &self,
                for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
            ) -> Val<'lt, Self::Pointee>
            where
                'lt: 'a,
            {
                // SAFETY: User promised!
                unsafe { self.to_inner().load_unchecked(for_ins) }
            }
            unsafe fn store_unchecked<'a, Value>(
                &mut self,
                val: Value,
                for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
            ) where
                Value: Holds<T = Self::Pointee>,
                'lt: 'a,
            {
                // SAFETY: User promised!
                unsafe { self.to_inner().store_unchecked(val, for_ins) }
            }
        }

        impl<'lt, Ptr> RefOps<'lt> for Val<'lt, $wrapper_name<Ptr>>
        where
            Val<'lt, Ptr>: RefOps<'lt>,
            Ptr: Ty + 'lt,
        {
            fn load(&self) -> Val<'lt, Self::Pointee> {
                self.to_inner().load()
            }
        }

        impl<'lt, Ptr> MutOps<'lt> for Val<'lt, $wrapper_name<Ptr>>
        where
            Val<'lt, Ptr>: MutOps<'lt>,
            Ptr: Ty + 'lt,
        {
            fn store<Value>(&mut self, val: Value)
            where
                Value: Holds<T = Self::Pointee>,
            {
                self.to_inner().store(val)
            }
        }
    };
}

impl_ptr_wrapper!(Global);
impl_ptr_wrapper!(Shared);
