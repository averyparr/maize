use inkwell::{
    AddressSpace,
    context::ContextRef,
    types::PointerType,
    values::{AnyValue, AnyValueEnum, BasicMetadataValueEnum, BasicValue, PointerValue},
};

use crate::{
    codegen::FnCodegen,
    ty::{AnyTy, SizedTy, sized::AlignedTy},
    val::Val,
};

use super::{M, P, R, Ty, ValTy};

macro_rules! body {
    (ty) => {
        type AnyType<'ctx> = PointerType<'ctx>;
        fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
            ctx.ptr_type(AddressSpace::default())
        }
    };
    (val_ty) => {
        type Value<'ctx> = PointerValue<'ctx>;

        fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
            ctx.ptr_type(AddressSpace::default()).get_undef()
        }

        fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
            ctx.ptr_type(AddressSpace::default()).const_null()
        }

        fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
            if let AnyValueEnum::PointerValue(val) = val {
                Some(val)
            } else {
                None
            }
        }
    };
}

impl<T> AnyTy for P<*mut T>
where
    T: AnyTy,
{
    body!(ty);
}

impl<T> AnyTy for P<*const T>
where
    T: AnyTy,
{
    body!(ty);
}

impl<'a, T> AnyTy for R<&'a T>
where
    T: Ty,
{
    body!(ty);
}

impl<'a, T> AnyTy for M<&'a mut T>
where
    T: Ty,
{
    body!(ty);
}

impl<T> ValTy for P<*const T>
where
    T: AnyTy,
{
    body!(val_ty);
}

impl<T> ValTy for P<*mut T>
where
    T: AnyTy,
{
    body!(val_ty);
}

impl<'a, T> ValTy for R<&'a T>
where
    T: Ty,
{
    body!(val_ty);
}

impl<'a, T> ValTy for M<&'a mut T>
where
    T: Ty,
{
    body!(val_ty);
}

/// # Safety:
/// Implementing this trait asserts that values of type `T` are freely
/// interconvertible with P<*const T::PointeeTy> and in particular
/// support the (unsafe) equivalents of ::std::ptr::read[_unaligned]
/// and casts to P<*const T::PointeeTy> and P<*mut T::PointeeTy>.
pub unsafe trait ConstPtrTy:
    for<'a> ValTy<Value<'a> = PointerValue<'a>, Type<'a> = PointerType<'a>> + SizedTy
{
    type PointeeTy: ValTy;

    fn instance_in_addrspace<'ctx>(
        ctx: ContextRef<'ctx>,
        address_space: AddressSpace,
    ) -> Self::Type<'ctx> {
        ctx.ptr_type(address_space)
    }

    /// # Safety:
    /// This function is unsafe in the same way that `::std::ptr::read_unaligned`
    /// is -- it just bitwise-copies from the address. It has the additional safety
    /// guarantee that all instruction metadata passed through `InstructionFunc` must
    /// not interfere with Rust's safety model (e.g. &mut T loads cannot be readonly)
    unsafe fn read_with_instruction_metadata<'a>(
        ptr: Val<'a, Self>,
        metadata: impl IntoIterator<Item = (&'a str, Option<BasicMetadataValueEnum<'a>>)>,
    ) -> Val<'a, Self::PointeeTy> {
        let pointee_ty = Self::PointeeTy::ty(ptr.ctx());
        // Safety: We have a pointer which the user guarantees is valid to read from, so it's safe to build
        // a pointer load at the end of the current BB
        let raw_val = unsafe {
            ptr.cx()
                .with_builder(|b| b.build_load(pointee_ty, ptr.ll_typed(), "load"))
        }
        .expect("Pointer load should be possible");
        let raw_ins = raw_val
            .as_instruction_value()
            .expect("Load should always be an instruction");

        for (name, args) in metadata.into_iter() {
            let kind_id = ptr.ctx().get_kind_id(name);
            raw_ins
                .set_metadata(ptr.ctx().metadata_node(args.as_slice()), kind_id)
                .expect("Failed to set metadata");
        }
        // Safety: We have just loaded a value of type PointeeTy, so it's safe to cast it.
        unsafe { Val::new(ptr.cx(), raw_val.as_any_value_enum()) }
    }

    /// # Safety:
    /// This function is unsafe in the same way that `::std::ptr::read_unaligned` is --
    /// it just bitwise-copies from the address with assumed alignment.
    /// This can lead to unsafety if Self::PointeeTy is not Copy.
    unsafe fn read_unaligned(ptr: Val<'_, Self>) -> Val<'_, Self::PointeeTy> {
        // The user promised it's safe to read without aligned access,
        // and we're not adding any additional metadata
        unsafe { Self::read_with_instruction_metadata(ptr, []) }
    }

    /// # Safety:
    /// This function is unsafe in the same way that `::std::ptr::read` is --
    /// it just bitwise-copies from the address with assumed alignment.
    /// This can lead to unsafety if Self::PointeeTy is not Copy.
    unsafe fn read(ptr: Val<'_, Self>) -> Val<'_, Self::PointeeTy>
    where
        Self::PointeeTy: SizedTy,
    {
        // The user promised it's safe to load and that the pointer is aligned,
        // so it's safe to read with alignment metadata.
        let cx = ptr.cx();
        let align = cx.constant_from(Self::PointeeTy::ALIGN).ll_typed().into();
        unsafe { Self::read_with_instruction_metadata(ptr, [("align", Some(align))]) }
    }

    fn cast_const(ptr: Val<'_, Self>) -> Val<'_, P<*const Self::PointeeTy>> {
        // Safety: User promised that this is safe by implementing the trait
        unsafe { Val::transmute(ptr) }
    }
    fn cast_mut(ptr: Val<'_, Self>) -> Val<'_, P<*mut Self::PointeeTy>> {
        // Safety: Interconverting a *const T to a *mut T is safe because
        // it requires downstream `Unsafe` to make use of it. User promised
        // that the initial cast was safe.
        let const_ptr = Self::cast_const(ptr);
        unsafe { Val::transmute(const_ptr) }
    }
}

unsafe impl<T> ConstPtrTy for P<*const T>
where
    T: ValTy,
{
    type PointeeTy = T;
}
unsafe impl<T> ConstPtrTy for P<*mut T>
where
    T: ValTy,
{
    type PointeeTy = T;
}
unsafe impl<T> ConstPtrTy for R<&T>
where
    T: ValTy,
{
    type PointeeTy = T;
}
unsafe impl<T> ConstPtrTy for M<&mut T>
where
    T: ValTy,
{
    type PointeeTy = T;
}

/// # Safety:
/// Implementing this trait asserts that values of type `T` are freely
/// interconvertible with P<*const T::PointeeTy> and in particular
/// support the (unsafe) equivalents of ::std::ptr::read[_unaligned]
/// and casts to P<*const T::PointeeTy> and P<*mut T::PointeeTy>.
pub unsafe trait MutPtrTy: ConstPtrTy {
    /// # Safety:
    /// This function is unsafe in the same way that `::std::ptr::write_unaligned`
    /// is -- it just bitwise-copies to the address. It does not drop the value at
    /// the address, so it can leak resources. It also has the requirement that
    /// InstructionFunc doesn't annotate the instruction with any additional metadata
    /// which would make the write unsafe.
    unsafe fn write_with_instruction_metadata<'a>(
        ptr: Val<'_, Self>,
        val: Val<'_, Self::PointeeTy>,
        metadata: impl IntoIterator<Item = (&'a str, Option<BasicMetadataValueEnum<'a>>)>,
    ) {
        let raw_ins = unsafe {
            ptr.cx()
                .with_builder(|b| b.build_store(ptr.ll_typed(), val.ll_typed()))
        }
        .expect("Pointer load should be possible");
        for (name, args) in metadata.into_iter() {
            let kind_id = ptr.ctx().get_kind_id(name);
            raw_ins
                .set_metadata(ptr.ctx().metadata_node(args.as_slice()), kind_id)
                .expect("Failed to set metadata");
        }
    }

    /// # Safety:
    /// This function is unsafe in the same way that `::std::ptr::write_unaligned` is --
    /// it just bitwise-copies to the address. It does not drop the value at the address,
    /// so it can leak resources. You must guarantee that the pointer is valid.
    unsafe fn write_unaligned(ptr: Val<'_, Self>, val: Val<'_, Self::PointeeTy>) {
        // Safety: The user promised it was safe to write (unaligned) to this pointer,
        // and we are not introducing any metadata to the instruction.
        unsafe {
            Self::write_with_instruction_metadata(ptr, val, []);
        }
    }

    /// # Safety:
    /// This function is unsafe in the same wya that `::std::ptr::write` is --
    /// it just bitwise-copies to the address. It does not drop the value at the address,
    /// so it can leak resources. You must ensure that the pointer is aligned and valid.
    unsafe fn write(ptr: Val<'_, Self>, val: Val<'_, Self::PointeeTy>)
    where
        Self::PointeeTy: SizedTy,
    {
        let align = ptr.cx().constant_from(Self::PointeeTy::ALIGN).ll_typed();
        // Safety: The user promised it was safe to do an aligned write through
        // this pointer, and we know the alignment of the type behind the pointer
        unsafe {
            Self::write_with_instruction_metadata(ptr, val, [("align", Some(align.into()))]);
        }
    }
}

unsafe impl<T> MutPtrTy for P<*mut T> where T: ValTy {}
unsafe impl<T> MutPtrTy for M<&mut T> where T: ValTy {}

/// # Safety:
/// In order for this to be safe, a `Ty` must trace-like a &'a T
/// and be interconvertible with a R<&'a T>
pub unsafe trait RefTy: ConstPtrTy {
    fn ptr_attrs(
        cx: &FnCodegen,
    ) -> impl IntoIterator<Item = (&str, Option<BasicMetadataValueEnum<'_>>)>
    where
        Self::PointeeTy: SizedTy;

    fn load<'a>(ptr: &Val<'a, Self>) -> Val<'a, Self::PointeeTy>
    where
        Self::PointeeTy: Copy + SizedTy,
    {
        let cx = ptr.cx();
        let ptr = Self::reborrow(ptr);
        unsafe { R::read_with_instruction_metadata(ptr, Self::ptr_attrs(cx)) }
    }
    fn reborrow<'a, 'b>(ptr: &'b Val<'a, Self>) -> Val<'a, R<&'b Self::PointeeTy>>
    where
        'a: 'b,
    {
        // Safety: we are shortening the lifetime from '_ to 'a
        // and otherwise performing a cast to R<&'a T> which the user
        // promised was OK
        unsafe { Val::new(ptr.cx(), ptr.raw()) }
    }
}

unsafe impl<'a, T> RefTy for R<&'a T>
where
    T: ValTy,
{
    fn ptr_attrs(
        _cx: &FnCodegen,
    ) -> impl IntoIterator<Item = (&str, Option<BasicMetadataValueEnum<'_>>)>
    where
        Self::PointeeTy: SizedTy,
    {
        // let size = cx.constant_from(Self::PointeeTy::SIZE);
        // let align = cx.constant_from(Self::PointeeTy::ALIGN);
        [
            // ("align", Some(align.ll_typed().into())),
            // ("dereferenceable", Some(size.ll_typed().into())),
            // ("nonnull", None),
            // ("readonly", None),
        ]
    }
}

unsafe impl<'a, T> RefTy for M<&'a mut T>
where
    T: ValTy,
{
    fn ptr_attrs(
        _cx: &FnCodegen,
    ) -> impl IntoIterator<Item = (&str, Option<BasicMetadataValueEnum<'_>>)>
    where
        Self::PointeeTy: SizedTy,
    {
        // let size = cx.constant_from(Self::PointeeTy::SIZE);
        // let align = cx.constant_from(Self::PointeeTy::ALIGN);
        [
            // ("align", Some(align.ll_typed().into())),
            // ("dereferenceable", Some(size.ll_typed().into())),
            // ("nonnull", None),
            // ("noalias", None),
        ]
    }
}

pub unsafe trait MutTy: RefTy + MutPtrTy {
    fn reborrow_mut<'a, 'b>(ptr: &'b mut Val<'a, Self>) -> Val<'a, M<&'b mut Self::PointeeTy>>
    where
        'a: 'b,
    {
        // Safety: we are shortening the lifetime from '_ to 'a
        // and otherwise performing a cast to M<&'a mut T> which the user
        // promised was OK
        unsafe { Val::new(ptr.cx(), ptr.raw()) }
    }
    fn swap<'a>(ptr_: &mut Val<'a, Self>, val: Val<'a, Self::PointeeTy>) -> Val<'a, Self::PointeeTy>
    where
        Self::PointeeTy: SizedTy,
    {
        let metadata = Self::ptr_attrs(ptr_.cx());
        let ptr = Self::reborrow(ptr_);
        // Safety: We hold a (short-lived) reference and can read with it. The
        // user asserted reads with these metadata were safe when they implemented `RefTy`.
        let at_ptr = unsafe { R::read_with_instruction_metadata(ptr, metadata) };

        let metadata = Self::ptr_attrs(ptr_.cx());
        let to_write = Self::reborrow_mut(ptr_);
        // Safety: We hold a (short-lived) exclusive reference and can write through it. The
        // user asserted reads with these metadata were safe when they implemented `RefTy`.
        let _: () = unsafe { M::write_with_instruction_metadata(to_write, val, metadata) };
        at_ptr
    }
    fn store<'a>(ptr: &mut Val<'a, Self>, val: Val<'a, Self::PointeeTy>)
    where
        Self::PointeeTy: SizedTy,
    {
        if Self::PointeeTy::NEEDS_DROP {
            let mut res = Self::swap(ptr, val);
            Self::PointeeTy::inner_drop(&mut res);
        } else {
            let ptr = Self::reborrow_mut(ptr);
            let metadata = Self::ptr_attrs(ptr.cx());
            let _: () = unsafe { M::write_with_instruction_metadata(ptr, val, metadata) };
        }
    }
}

unsafe impl<T> MutTy for M<&mut T> where T: ValTy {}

pub trait AddrspacePtr {
    type Inner: ConstPtrTy;
    const ADDRSPACE: u16;
}

impl<T> AnyTy for T
where
    T: AddrspacePtr,
{
    type AnyType<'ctx> = <T::Inner as Ty>::Type<'ctx>;
    fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
        ctx.ptr_type(AddressSpace::from(Self::ADDRSPACE))
    }
}

impl<T> ValTy for T
where
    T: AddrspacePtr,
{
    type Value<'ctx> = <T::Inner as ValTy>::Value<'ctx>;

    fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        Self::ty(ctx).get_undef()
    }

    fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        Self::ty(ctx).const_null()
    }

    fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
        T::Inner::try_type_val(val)
    }
}

impl<T> AlignedTy for T
where
    T: AddrspacePtr,
    T::Inner: AlignedTy,
{
    const ALIGN: u32 = T::Inner::ALIGN;
}

impl<T> SizedTy for T
where
    T: AddrspacePtr,
    T::Inner: SizedTy,
{
    const SIZE: u32 = T::Inner::SIZE;
    fn fn_arg_attrs(
        ctx: ContextRef<'_>,
    ) -> impl IntoIterator<Item = inkwell::attributes::Attribute> {
        T::Inner::fn_arg_attrs(ctx)
    }
}

unsafe impl<T> ConstPtrTy for T
where
    T: AddrspacePtr,
    T::Inner: ConstPtrTy,
{
    type PointeeTy = <T::Inner as ConstPtrTy>::PointeeTy;
}

unsafe impl<T> MutPtrTy for T
where
    T: AddrspacePtr,
    T::Inner: MutPtrTy,
{
}

unsafe impl<T> RefTy for T
where
    T: AddrspacePtr,
    T::Inner: RefTy,
{
    fn ptr_attrs(
        cx: &FnCodegen,
    ) -> impl IntoIterator<Item = (&str, Option<BasicMetadataValueEnum<'_>>)>
    where
        Self::PointeeTy: SizedTy,
    {
        T::Inner::ptr_attrs(cx)
    }
}

unsafe impl<T> MutTy for T
where
    T: AddrspacePtr,
    T::Inner: MutTy,
{
}
