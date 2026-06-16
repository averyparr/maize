use std::marker::PhantomData;

use inkwell::{
    AddressSpace,
    types::BasicType,
    values::{BasicValue, InstructionValue},
};

use crate::{
    ContextRef, PointerType, PointerValue,
    backend::{FnCtx, UntypedValue},
    tipe::Ty,
    val::Val,
};

#[derive(Clone, Copy, PartialEq)]
pub struct A<Ptr, const ADDRSPACE: u16>(PhantomData<Ptr>);

impl<Ptr: Ty<LLType = PointerType>> From<Val<Ptr>> for Val<A<Ptr, 0>> {
    fn from(value: Val<Ptr>) -> Self {
        let (fn_ref, value) = value.decompose();
        unsafe { Val::new(fn_ref, value) }
    }
}

impl<T: Ty> Ty for *const T
where
    T: Sized,
{
    type LLType = PointerType;
    type LLVal = PointerValue;

    fn raw_ty(ctx: ContextRef) -> Self::LLType {
        ctx.ptr_type(AddressSpace::default())
    }

    fn ty(cg: &crate::backend::FnCtx) -> crate::backend::ErasedType {
        crate::backend::ErasedType(Self::raw_ty(cg.ctx()).as_basic_type_enum())
    }

    fn type_val(val: crate::backend::UntypedValue) -> Self::LLVal {
        val.0.into_pointer_value()
    }

    fn const_val(self, _: crate::backend::FnRef) -> Val<Self>
    where
        Self: Copy,
    {
        panic!("Pointers should not be used for const values!")
    }
}

impl<T: Ty> Ty for *mut T
where
    T: Sized,
{
    type LLType = PointerType;
    type LLVal = PointerValue;

    fn raw_ty(ctx: ContextRef) -> Self::LLType {
        ctx.ptr_type(AddressSpace::default())
    }

    fn ty(cg: &crate::backend::FnCtx) -> crate::backend::ErasedType {
        crate::backend::ErasedType(Self::raw_ty(cg.ctx()).as_basic_type_enum())
    }

    fn type_val(val: crate::backend::UntypedValue) -> Self::LLVal {
        val.0.into_pointer_value()
    }

    fn const_val(self, _: crate::backend::FnRef) -> Val<Self>
    where
        Self: Copy,
    {
        panic!("Pointers should not be used for const values!")
    }
}

impl<T: Ty> Ty for &'_ T {
    type LLType = PointerType;
    type LLVal = PointerValue;

    fn raw_ty(ctx: ContextRef) -> Self::LLType {
        ctx.ptr_type(AddressSpace::default())
    }

    fn ty(cg: &crate::backend::FnCtx) -> crate::backend::ErasedType {
        crate::backend::ErasedType(Self::raw_ty(cg.ctx()).as_basic_type_enum())
    }

    fn type_val(val: crate::backend::UntypedValue) -> Self::LLVal {
        val.0.into_pointer_value()
    }

    fn type_metadata_on_function(func: &FnCtx, param: &mut impl Iterator<Item = u32>) {
        let param = param.next().expect("Should be in-bounds");
        func.set_param_alignment(param, T::align());
        func.set_param_metadata(
            param,
            "dereferenceable",
            std::mem::size_of::<T>()
                .try_into()
                .expect("usize -> u64 overflow"),
        );
        func.set_param_metadata(param, "readonly", 0);
        func.set_param_metadata(param, "nonnull", 0);
        func.set_param_metadata(param, "noalias", 0);
    }

    fn const_val(self, _: crate::backend::FnRef) -> Val<Self>
    where
        Self: Copy,
    {
        panic!("Pointers should not be used for const values!")
    }
}

impl<T: Ty> Ty for &'_ mut T
where
    T: Sized,
{
    type LLType = PointerType;
    type LLVal = PointerValue;

    fn raw_ty(ctx: ContextRef) -> Self::LLType {
        ctx.ptr_type(AddressSpace::default())
    }

    fn ty(cg: &crate::backend::FnCtx) -> crate::backend::ErasedType {
        crate::backend::ErasedType(Self::raw_ty(cg.ctx()).as_basic_type_enum())
    }

    fn type_val(val: crate::backend::UntypedValue) -> Self::LLVal {
        val.0.into_pointer_value()
    }

    fn type_metadata_on_function(func: &FnCtx, param: &mut impl Iterator<Item = u32>) {
        let param = param.next().expect("Should be in-bounds");
        func.set_param_alignment(param, T::align());
        func.set_param_metadata(
            param,
            "dereferenceable",
            std::mem::size_of::<T>()
                .try_into()
                .expect("usize -> u64 overflow"),
        );
        func.set_param_metadata(param, "nonnull", 0);
        func.set_param_metadata(param, "noalias", 0);
    }

    fn const_val(self, _: crate::backend::FnRef) -> Val<Self>
    where
        Self: Sized,
    {
        panic!("Pointers should not be used for const values!")
    }
}

impl<Ptr: Ty<LLType = PointerType>, const ADDRSPACE: u16> Ty for A<Ptr, ADDRSPACE> {
    type LLType = Ptr::LLType;
    type LLVal = Ptr::LLVal;

    fn raw_ty(ctx: ContextRef) -> Self::LLType {
        assert_eq!(
            Ptr::raw_ty(ctx).as_basic_type_enum(),
            ctx.ptr_type(AddressSpace::default()).as_basic_type_enum()
        );
        ctx.ptr_type(AddressSpace::from(ADDRSPACE))
    }

    fn type_val(val: UntypedValue) -> Self::LLVal {
        Ptr::type_val(val)
    }

    fn const_val(self, fn_ref: crate::backend::FnRef) -> Val<Self>
    where
        Self: Sized,
    {
        panic!("Pointers should not be used for const values!")
    }
    fn align() -> usize
    where
        Self: Sized,
    {
        Ptr::align()
    }
    fn size() -> usize
    where
        Self: Sized,
    {
        Ptr::size()
    }
    fn ty(cg: &FnCtx) -> crate::backend::ErasedType {
        crate::backend::ErasedType(Self::raw_ty(cg.ctx()).into())
    }
    fn type_metadata(fn_ctx: &FnCtx, value: &mut UntypedValue) {
        Ptr::type_metadata(fn_ctx, value);
    }
    fn type_metadata_on_function(fn_ctx: &FnCtx, idx: &mut impl Iterator<Item = u32>) {
        Ptr::type_metadata_on_function(fn_ctx, idx);
    }
    fn undef_val(fn_ref: crate::backend::FnRef) -> Val<Self>
    where
        Self: Sized,
    {
        let raw = Self::raw_ty(fn_ref.ctx()).get_undef();
        unsafe { Val::new(fn_ref, crate::backend::UntypedValue(raw.into())) }
    }
}

impl<T: Ty> Val<*const T> {
    pub fn ptr_cast<U: Ty>(self) -> Val<*const U> {
        let (fn_ref, value) = self.decompose();
        // Safety: *const T -> *const U is safe
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn as_mut(self) -> Val<*mut T> {
        let (fn_ref, value) = self.decompose();
        // Safety: *const T -> *mut T is safe
        unsafe { Val::new(fn_ref, value) }
    }

    pub unsafe fn assume_ref<'a>(self) -> Val<&'a T> {
        let (fn_ref, value) = self.decompose();
        // Safety: User promised!
        unsafe { Val::new(fn_ref, value) }
    }

    pub unsafe fn read(self) -> Val<T> {
        let fn_ref = self.fn_ref();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let raw = b
            .build_load(T::ty(fn_ref).0, self.typed(), "ptr_load")
            .expect("Load should succeed");
        // Safety: User promised!
        unsafe { Val::new(fn_ref.clone(), UntypedValue(raw)) }
    }

    pub fn as_u64(self) -> Val<u64> {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let raw = b
            .build_ptr_to_int(self.typed(), u64::raw_ty(fn_ref.ctx()), "ptr_to_u64")
            .expect("ptr->int should succeed");
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }

    pub unsafe fn assume_space<const SPACE: u16>(self) -> Val<A<*mut T, SPACE>> {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let raw = b
            .build_address_space_cast(
                self.typed(),
                A::<*mut T, SPACE>::raw_ty(fn_ref.ctx()),
                "generic_to_space",
            )
            .expect("addrspace cast should succced");
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }
}

impl<T: Ty> Val<*mut T> {
    pub fn ptr_cast<U: Ty>(self) -> Val<*mut U> {
        let (fn_ref, value) = self.decompose();
        // Safety: *mut T -> *mut U is safe
        unsafe { Val::new(fn_ref, value) }
    }
    pub fn as_const(self) -> Val<*const T> {
        let (fn_ref, value) = self.decompose();
        // Safety: &T -> *const T is safe
        unsafe { Val::new(fn_ref, value) }
    }

    pub unsafe fn read(self) -> Val<T> {
        unsafe { self.as_const().read() }
    }

    pub unsafe fn assume_ref<'a>(self) -> Val<&'a T> {
        let (fn_ref, value) = self.decompose();
        // Safety: user promised!
        unsafe { Val::new(fn_ref, value) }
    }

    pub unsafe fn assume_mut<'a>(self) -> Val<&'a mut T> {
        let (fn_ref, value) = self.decompose();
        // Safety: user promised!
        unsafe { Val::new(fn_ref, value) }
    }

    pub unsafe fn write(self, val: Val<T>) -> InstructionValue<'static> {
        let fn_ref = self.fn_ref();
        let b = unsafe { fn_ref.curr_bb_builder() };
        b.build_store(self.typed(), val.raw().0)
            .expect("Store should have succeeded")
    }

    pub fn as_u64(self) -> Val<u64> {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let raw = b
            .build_ptr_to_int(self.typed(), u64::raw_ty(fn_ref.ctx()), "ptr_to_u64")
            .expect("ptr->int should succeed");
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }

    pub unsafe fn assume_space<const SPACE: u16>(self) -> Val<A<*mut T, SPACE>> {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let raw = b
            .build_address_space_cast(
                self.typed(),
                A::<*mut T, SPACE>::raw_ty(fn_ref.ctx()),
                "generic_to_space",
            )
            .expect("addrspace cast should succced");
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }
}

impl<'a, T: Ty> Val<&'a T> {
    pub fn as_ptr(self) -> Val<*const T> {
        let (fn_ref, value) = self.decompose();
        // Safety: &T -> *const T is safe
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn reborrow<'b>(&'b self) -> Val<&'b T>
    where
        'a: 'b,
    {
        let fn_ref = self.fn_ref().clone();
        let value = self.raw();
        // Safety: Shortening poitner life is OK
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn load(&self) -> Val<T>
    where
        T: Copy,
    {
        let val = unsafe { self.reborrow().as_ptr().read() };
        if let Some(ins) = val.raw().0.as_instruction_value() {
            ins.set_alignment(T::align().try_into().expect("usize -> u32 overflow"))
                .expect("Align setting should have worked");
        }
        val
    }
}

impl<'a, T: Ty> Val<&'a mut T> {
    pub fn reborrow<'b>(&'b mut self) -> Val<&'b mut T>
    where
        'a: 'b,
    {
        let fn_ref = self.fn_ref().clone();
        let value = self.raw();
        // Safety: Shortening poitner life is OK
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn as_ptr(self) -> Val<*const T> {
        let (fn_ref, value) = self.decompose();
        // Safety: &mut T -> *const T is safe
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn as_mut_ptr(self) -> Val<*mut T> {
        let (fn_ref, value) = self.decompose();
        // Safety: &mut T -> *mut T is safe
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn as_ref<'b>(&'b self) -> Val<&'b T>
    where
        'a: 'b,
    {
        let fn_ref = self.fn_ref().clone();
        let value = self.raw();
        // Safety: Shortening lifetime is safe; &mut T -> &T safe
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn load(&self) -> Val<T>
    where
        T: Copy,
    {
        let val = unsafe { self.as_ref().as_ptr().read() };
        if let Some(ins) = val.raw().0.as_instruction_value() {
            ins.set_alignment(T::align().try_into().expect("usize -> u32 overflow"))
                .expect("Align setting should have worked");
        }
        val
    }

    pub fn store(&mut self, val: Val<T>)
    where
        T: Copy,
    {
        let ins = unsafe { self.reborrow().as_mut_ptr().write(val) };
        ins.set_alignment(T::align().try_into().expect("usize -> u32 overflow"))
            .expect("Align setting should have worked");
    }
}

impl<T: Ty, const ADDRSPACE: u16> Val<A<*const T, ADDRSPACE>> {
    pub fn ptr_cast<U: Ty>(self) -> Val<A<*const U, ADDRSPACE>> {
        let (fn_ref, value) = self.decompose();
        // Safety: *const T -> *const U is safe
        unsafe { Val::new(fn_ref, value) }
    }
    pub fn as_mut(self) -> Val<A<*mut T, ADDRSPACE>> {
        let (fn_ref, value) = self.decompose();
        // Safety: User promised!
        unsafe { Val::new(fn_ref, value) }
    }

    pub unsafe fn assume_ref<'a>(self) -> Val<A<&'a T, ADDRSPACE>> {
        let (fn_ref, value) = self.decompose();
        // Safety: User promised!
        unsafe { Val::new(fn_ref, value) }
    }

    pub unsafe fn read(self) -> Val<T> {
        let fn_ref = self.fn_ref();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let raw = b
            .build_load(T::ty(fn_ref).0, self.typed(), "ptr_load")
            .expect("Load should succeed");
        // Safety: User promised!
        unsafe { Val::new(fn_ref.clone(), UntypedValue(raw)) }
    }

    pub fn as_u64(self) -> Val<u64> {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let raw = b
            .build_ptr_to_int(self.typed(), u64::raw_ty(fn_ref.ctx()), "ptr_to_u64")
            .expect("ptr->int should succeed");
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }

    pub fn to_generic(self) -> Val<*const T> {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let raw = b
            .build_address_space_cast(self.typed(), <*const T>::raw_ty(fn_ref.ctx()), "to_generic")
            .expect("addrspace cast should succeed");
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }
}

impl<T: Ty, const ADDRSPACE: u16> Val<A<*mut T, ADDRSPACE>> {
    pub fn ptr_cast<U: Ty>(self) -> Val<A<*mut U, ADDRSPACE>> {
        let (fn_ref, value) = self.decompose();
        // Safety: *mut T -> *mut U is safe
        unsafe { Val::new(fn_ref, value) }
    }
    pub fn as_const(self) -> Val<A<*const T, ADDRSPACE>> {
        let (fn_ref, value) = self.decompose();
        // Safety: *mut T -> *const T is safe
        unsafe { Val::new(fn_ref, value) }
    }

    pub unsafe fn read(self) -> Val<T> {
        unsafe { self.as_const().read() }
    }

    pub unsafe fn assume_ref<'a>(self) -> Val<A<&'a T, ADDRSPACE>> {
        let (fn_ref, value) = self.decompose();
        // Safety: User promised *mut T -> &T
        unsafe { Val::new(fn_ref, value) }
    }

    pub unsafe fn assume_mut<'a>(self) -> Val<A<&'a mut T, ADDRSPACE>> {
        let (fn_ref, value) = self.decompose();
        // Safety: User promised *mut T -> &mut T
        unsafe { Val::new(fn_ref, value) }
    }

    pub unsafe fn write(self, val: Val<T>) -> InstructionValue<'static> {
        let fn_ref = self.fn_ref();
        let b = unsafe { fn_ref.curr_bb_builder() };
        b.build_store(self.typed(), val.raw().0)
            .expect("Store should have succeeded")
    }

    pub fn as_u64(self) -> Val<u64> {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let raw = b
            .build_ptr_to_int(self.typed(), u64::raw_ty(fn_ref.ctx()), "ptr_to_u64")
            .expect("ptr->int should succeed");
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }

    pub fn to_generic(self) -> Val<*mut T> {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let raw = b
            .build_address_space_cast(self.typed(), <*const T>::raw_ty(fn_ref.ctx()), "to_generic")
            .expect("addrspace cast should succeed");
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }
}

impl<'a, T: Ty, const ADDRSPACE: u16> Val<A<&'a T, ADDRSPACE>> {
    pub fn as_ptr(self) -> Val<A<*const T, ADDRSPACE>> {
        let (fn_ref, value) = self.decompose();
        // Safety: &T -> *const T is safe
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn reborrow<'b>(&'b self) -> Val<A<&'b T, ADDRSPACE>>
    where
        'a: 'b,
    {
        let fn_ref = self.fn_ref().clone();
        let value = self.raw();
        // Safety: Shortening poitner life is OK
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn load(&self) -> Val<T>
    where
        T: Copy,
    {
        let val = unsafe { self.reborrow().as_ptr().read() };
        if let Some(ins) = val.raw().0.as_instruction_value() {
            ins.set_alignment(T::align().try_into().expect("usize -> u32 overflow"))
                .expect("Align setting should have worked");
        }
        val
    }

    pub fn load_nc(&self) -> Val<T>
    where
        T: Copy,
    {
        let ret = self.load();
        if let Some(ins) = ret.raw().0.as_instruction_value() {
            let ctx = ret.fn_ref().ctx();
            let kind_id = ctx.get_kind_id("invariant.load");
            ins.set_metadata(ctx.metadata_node(&[]), kind_id)
                .expect("Should be able to set metadata")
        }
        ret
    }
}

impl<'a, T: Ty, const ADDRSPACE: u16> Val<A<&'a mut T, ADDRSPACE>> {
    pub fn reborrow<'b>(&'b mut self) -> Val<A<&'b mut T, ADDRSPACE>>
    where
        'a: 'b,
    {
        let fn_ref = self.fn_ref().clone();
        let value = self.raw();
        // Safety: Shortening poitner life is OK
        unsafe { Val::new(fn_ref, value) }
    }
    pub fn as_ptr(self) -> Val<A<*const T, ADDRSPACE>> {
        let (fn_ref, value) = self.decompose();
        // Safety: &mut -> *const is safe
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn as_mut_ptr(self) -> Val<A<*mut T, ADDRSPACE>> {
        let (fn_ref, value) = self.decompose();
        // Safety: &mut -> *mut is safe
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn as_ref<'b>(&'b self) -> Val<A<&'b T, ADDRSPACE>>
    where
        'a: 'b,
    {
        let fn_ref = self.fn_ref().clone();
        let value = self.raw();
        // Safety: &mut T -> &T is safe; we shorten the life
        unsafe { Val::new(fn_ref, value) }
    }

    pub fn load(&self) -> Val<T>
    where
        T: Copy,
    {
        let val = unsafe { self.as_ref().as_ptr().read() };
        if let Some(ins) = val.raw().0.as_instruction_value() {
            ins.set_alignment(T::align().try_into().expect("usize -> u32 overflow"))
                .expect("Align setting should have worked");
        }
        val
    }

    pub fn store(&mut self, val: Val<T>)
    where
        T: Copy,
    {
        let ins = unsafe { self.reborrow().as_mut_ptr().write(val) };
        ins.set_alignment(T::align().try_into().expect("usize -> u32 overflow"))
            .expect("Align setting should have worked");
    }
}
