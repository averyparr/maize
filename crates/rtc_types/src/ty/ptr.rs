use inkwell::{
    AddressSpace,
    context::ContextRef,
    types::PointerType,
    values::{AnyValueEnum, BasicMetadataValueEnum, BasicValue, PointerValue},
};

use crate::{
    codegen::FnCodegen,
    ty::{AnyTy, M, P, R, SizedTy, Ty, ValTy},
    val::Val,
};

#[derive(Clone, Copy)]
pub struct DefaultAddressSpace;

pub trait Addrspace: Copy {
    const AS_U16: u16;
}

impl Addrspace for DefaultAddressSpace {
    const AS_U16: u16 = 0;
}

pub trait DereferencableTy:
    for<'a> ValTy<Type<'a> = PointerType<'a>, Value<'a> = PointerValue<'a>>
{
    type Pointee: AnyTy;
    type Space: Addrspace;
    unsafe fn on_underlying_raw<'a, F>(val: &Val<'a, Self>, f: F) -> Val<'a, Self>
    where
        F: FnOnce(
            Val<'a, P<*const Self::Pointee, Self::Space>>,
        ) -> Val<'a, P<*const Self::Pointee, Self::Space>>;

    type Parametrized<'short>: 'short
        + DereferencableTy<Pointee = Self::Pointee, Parametrized<'short> = Self::Parametrized<'short>>
    where
        Self: 'short;
    fn parametrize<'a, 'b>(val: &'b mut Val<'a, Self>) -> Val<'a, Self::Parametrized<'b>>;
    fn shorten_by_ref<'a, 'b, 'c>(
        val: &'b mut Val<'a, Self::Parametrized<'c>>,
    ) -> Val<'a, Self::Parametrized<'b>>
    where
        'c: 'b,
        Self: 'c;
    fn shorten<'a, 'b, 'c>(val: Val<'a, Self::Parametrized<'c>>) -> Val<'a, Self::Parametrized<'b>>
    where
        'c: 'b,
        Self: 'c;
}

pub trait PtrTy: DereferencableTy {}

impl<T: AnyTy, Space: Addrspace> DereferencableTy for P<*const T, Space> {
    type Pointee = T;
    type Space = Space;
    unsafe fn on_underlying_raw<'a, F>(val: &Val<'a, Self>, f: F) -> Val<'a, Self>
    where
        F: FnOnce(
            Val<'a, P<*const Self::Pointee, Self::Space>>,
        ) -> Val<'a, P<*const Self::Pointee, Self::Space>>,
    {
        f(*val)
    }
    type Parametrized<'short>
        = Self
    where
        Self: 'short;
    fn parametrize<'a, 'b>(val: &'b mut Val<'a, Self>) -> Val<'a, Self::Parametrized<'b>> {
        *val
    }
    fn shorten_by_ref<'a, 'b, 'c>(
        val: &'b mut Val<'a, Self::Parametrized<'c>>,
    ) -> Val<'a, Self::Parametrized<'b>>
    where
        'c: 'b,
        Self: 'c,
    {
        *val
    }
    fn shorten<'a, 'b, 'c>(val: Val<'a, Self::Parametrized<'c>>) -> Val<'a, Self::Parametrized<'b>>
    where
        'c: 'b,
        Self: 'c,
    {
        val
    }
}
impl<T: AnyTy, Space: Addrspace> DereferencableTy for P<*mut T, Space> {
    type Pointee = T;
    type Space = Space;
    unsafe fn on_underlying_raw<'a, F>(val: &Val<'a, Self>, f: F) -> Val<'a, Self>
    where
        F: FnOnce(
            Val<'a, P<*const Self::Pointee, Self::Space>>,
        ) -> Val<'a, P<*const Self::Pointee, Self::Space>>,
    {
        f(val.as_const()).as_mut()
    }
    type Parametrized<'short>
        = Self
    where
        Self: 'short;
    fn parametrize<'a, 'b>(val: &'b mut Val<'a, Self>) -> Val<'a, Self::Parametrized<'b>> {
        *val
    }
    fn shorten_by_ref<'a, 'b, 'c>(
        val: &'b mut Val<'a, Self::Parametrized<'c>>,
    ) -> Val<'a, Self::Parametrized<'b>>
    where
        'c: 'b,
        Self: 'c,
    {
        *val
    }
    fn shorten<'a, 'b, 'c>(val: Val<'a, Self::Parametrized<'c>>) -> Val<'a, Self::Parametrized<'b>>
    where
        'c: 'b,
        Self: 'c,
    {
        val
    }
}
impl<'borrow, T: ValTy, Space: Addrspace> DereferencableTy for R<&'borrow T, Space> {
    type Pointee = T;
    type Space = Space;
    unsafe fn on_underlying_raw<'a, F>(val: &Val<'a, Self>, f: F) -> Val<'a, Self>
    where
        F: FnOnce(
            Val<'a, P<*const Self::Pointee, Self::Space>>,
        ) -> Val<'a, P<*const Self::Pointee, Self::Space>>,
    {
        unsafe { f(val.as_ptr()).as_ref_unchecked() }
    }
    type Parametrized<'short>
        = R<&'short T, Space>
    where
        Self: 'short;
    fn parametrize<'a, 'b>(val: &'b mut Val<'a, Self>) -> Val<'a, Self::Parametrized<'b>> {
        val.reborrow()
    }
    fn shorten_by_ref<'a, 'b, 'c>(
        val: &'b mut Val<'a, Self::Parametrized<'c>>,
    ) -> Val<'a, Self::Parametrized<'b>>
    where
        'c: 'b,
        Self: 'c,
    {
        *val
    }
    fn shorten<'a, 'b, 'c>(val: Val<'a, Self::Parametrized<'c>>) -> Val<'a, Self::Parametrized<'b>>
    where
        'c: 'b,
        Self: 'c,
    {
        val
    }
}
impl<'borrow, T: ValTy, Space: Addrspace> DereferencableTy for M<&'borrow mut T, Space> {
    type Pointee = T;
    type Space = Space;
    unsafe fn on_underlying_raw<'a, F>(val: &Val<'a, Self>, f: F) -> Val<'a, Self>
    where
        F: FnOnce(
            Val<'a, P<*const Self::Pointee, Self::Space>>,
        ) -> Val<'a, P<*const Self::Pointee, Self::Space>>,
    {
        unsafe { f(val.as_ptr()).as_mut().as_mut_unchecked() }
    }
    type Parametrized<'short>
        = M<&'short mut T, Space>
    where
        Self: 'short;
    fn parametrize<'a, 'b>(val: &'b mut Val<'a, Self>) -> Val<'a, Self::Parametrized<'b>> {
        val.reborrow_mut()
    }
    fn shorten_by_ref<'a, 'b, 'c>(
        val: &'b mut Val<'a, Self::Parametrized<'c>>,
    ) -> Val<'a, Self::Parametrized<'b>>
    where
        'c: 'b,
        Self: 'c,
    {
        val.reborrow_mut()
    }
    fn shorten<'a, 'b, 'c>(val: Val<'a, Self::Parametrized<'c>>) -> Val<'a, Self::Parametrized<'b>>
    where
        'c: 'b,
        Self: 'c,
    {
        val
    }
}

impl<T: AnyTy, Space: Addrspace> PtrTy for P<*mut T, Space> {}
impl<T: AnyTy, Space: Addrspace> PtrTy for P<*const T, Space> {}

macro_rules! body {
    (ty, $addrspace: ty) => {
        type AnyType<'ctx> = PointerType<'ctx>;
        fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
            ctx.ptr_type(AddressSpace::from(<$addrspace>::AS_U16))
        }
    };
    (val_ty) => {
        type Value<'ctx> = PointerValue<'ctx>;

        fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
            Self::ty(ctx).get_undef()
        }

        fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
            Self::ty(ctx).const_null()
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

impl<T: ?Sized, Space: Addrspace> AnyTy for P<*mut T, Space>
where
    T: AnyTy,
{
    body!(ty, Space);
}

impl<T: ?Sized, Space: Addrspace> AnyTy for P<*const T, Space>
where
    T: AnyTy,
{
    body!(ty, Space);
}

impl<'a, T: ?Sized, Space: Addrspace> AnyTy for R<&'a T, Space>
where
    T: Ty,
{
    body!(ty, Space);
}

impl<'a, T: ?Sized, Space: Addrspace> AnyTy for M<&'a mut T, Space>
where
    T: Ty,
{
    body!(ty, Space);
}

impl<T: ?Sized, Space: Addrspace> ValTy for P<*const T, Space>
where
    T: AnyTy,
{
    body!(val_ty);
}

impl<T: ?Sized, Space: Addrspace> ValTy for P<*mut T, Space>
where
    T: AnyTy,
{
    body!(val_ty);
}

impl<'a, T: ?Sized, Space: Addrspace> ValTy for R<&'a T, Space>
where
    T: Ty,
{
    body!(val_ty);
}

impl<'a, T: ?Sized, Space: Addrspace> ValTy for M<&'a mut T, Space>
where
    T: Ty,
{
    body!(val_ty);
}

impl<T: AnyTy + ?Sized, Space: Addrspace> P<*const T, Space> {
    /// # Safety:
    /// This function is unsafe in the same way that `::std::ptr::read_unaligned`
    /// is -- it just bitwise-copies from the address. It has the additional safety
    /// guarantee that all instruction metadata passed through `InstructionFunc` must
    /// not interfere with Rust's safety model (e.g. &mut T cannot be readonly but can be noalias)
    unsafe fn read_with_instruction_metadata<'a, 'b>(
        ptr: Val<'a, Self>,
        read_metadata: impl IntoIterator<Item = (&'b str, Option<BasicMetadataValueEnum<'static>>)>,
    ) -> Val<'a, T>
    where
        T: ValTy,
    {
        let raw_ptr = ptr.ll_typed();
        let pointee_ty = T::ty(ptr.ctx());
        let raw_load = unsafe {
            ptr.cx()
                .with_builder(|b| b.build_load(pointee_ty, raw_ptr, "load"))
        }
        .expect("Pointer load should be possible");

        if let Some(ins) = raw_load.as_instruction_value() {
            for (name, args) in read_metadata.into_iter() {
                let kind_id = ptr.ctx().get_kind_id(name);
                ins.set_metadata(ptr.ctx().metadata_node(args.as_slice()), kind_id)
                    .expect("Failed to set metadata");
            }
        }

        // Safety: We have just loaded a value of type PointeeTy, so it's safe to cast it.
        unsafe { Val::new(ptr.cx(), raw_load) }
    }

    /// # Safety:
    /// This function is unsafe in the same way that `::std::ptr::read_unaligned` is --
    /// it just bitwise-copies from the address with assumed alignment.
    /// This can lead to unsafety if Self::PointeeTy is not Copy.
    unsafe fn read_unaligned(ptr: Val<'_, Self>) -> Val<'_, T>
    where
        T: ValTy,
    {
        // The user promised it's safe to read without aligned access,
        // and we're not adding any additional metadata
        unsafe { Self::read_with_instruction_metadata(ptr, []) }
    }

    /// # Safety:
    /// This function is unsafe in the same way that `::std::ptr::read` is --
    /// it just bitwise-copies from the address with assumed alignment.
    /// This can lead to unsafety if Self::PointeeTy is not Copy.
    unsafe fn read(ptr: Val<'_, Self>) -> Val<'_, T>
    where
        T: SizedTy,
    {
        // The user promised it's safe to load and that the pointer is aligned,
        // so it's safe to read with alignment metadata.
        let cx = ptr.cx();
        let align = cx.constant_from(T::ALIGN as u64).ll_typed().into();
        unsafe { Self::read_with_instruction_metadata(ptr, [("align", Some(align))]) }
    }

    fn to_mut_ptr(ptr: Val<'_, Self>) -> Val<'_, P<*mut T, Space>> {
        unsafe { Val::transmute(ptr) }
    }

    fn cast<U: AnyTy + ?Sized>(ptr: Val<'_, Self>) -> Val<'_, P<*const U, Space>> {
        unsafe { Val::transmute(ptr) }
    }

    unsafe fn to_ref_unchecked<'a, 'b>(val: Val<'a, Self>) -> Val<'a, R<&'b T, Space>>
    where
        T: ValTy,
    {
        unsafe { Val::transmute(val) }
    }
}

impl<T: AnyTy, Space: Addrspace> P<*mut T, Space> {
    /// # Safety:
    /// This function is unsafe in the same way that `::std::ptr::write_unaligned`
    /// is -- it just bitwise-copies to the address. It does not drop the value at
    /// the address, so it can leak resources. It also has the requirement that
    /// InstructionFunc doesn't annotate the instruction with any additional metadata
    /// which would make the write unsafe.
    unsafe fn write_with_instruction_metadata<'a, 'b>(
        ptr: Val<'_, Self>,
        val: Val<'_, T>,
        write_metadata: impl IntoIterator<Item = (&'b str, Option<BasicMetadataValueEnum<'static>>)>,
    ) where
        T: ValTy,
    {
        let raw_ptr = ptr.ll_typed();
        let value = val.ll_typed();
        let raw_store = unsafe { ptr.cx().with_builder(|b| b.build_store(raw_ptr, value)) }
            .expect("Pointer store should be possible");

        for (name, args) in write_metadata.into_iter() {
            let kind_id = ptr.ctx().get_kind_id(name);
            raw_store
                .set_metadata(ptr.ctx().metadata_node(args.as_slice()), kind_id)
                .expect("Failed to set metadata");
        }
    }

    /// # Safety:
    /// This function is unsafe in the same way that `::std::ptr::write_unaligned` is --
    /// it just bitwise-copies to the address. It does not drop the value at the address,
    /// so it can leak resources. You must guarantee that the pointer is valid.
    unsafe fn write_unaligned(ptr: Val<'_, Self>, val: Val<'_, T>)
    where
        T: ValTy,
    {
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
    unsafe fn write(ptr: Val<'_, Self>, val: Val<'_, T>)
    where
        T: SizedTy,
    {
        let align = ptr.cx().constant_from(T::ALIGN as u64).ll_typed();
        // Safety: The user promised it was safe to do an aligned write through
        // this pointer, and we know the alignment of the type behind the pointer
        unsafe {
            Self::write_with_instruction_metadata(ptr, val, [("align", Some(align.into()))]);
        }
    }

    fn to_const_ptr(ptr: Val<'_, Self>) -> Val<'_, P<*const T, Space>> {
        unsafe { Val::transmute(ptr) }
    }
    unsafe fn to_mut_unchecked<'a, 'b>(val: Val<'a, Self>) -> Val<'a, M<&'b mut T, Space>>
    where
        T: ValTy,
    {
        unsafe { Val::transmute(val) }
    }

    fn cast_mut<U: AnyTy + ?Sized>(ptr: Val<'_, Self>) -> Val<'_, P<*mut U, Space>> {
        unsafe { Val::transmute(ptr) }
    }
}

impl<'b, T: ValTy + ?Sized, Space: Addrspace> R<&'b T, Space> {
    fn load_metadata(
        _cx: &FnCodegen,
    ) -> impl IntoIterator<Item = (&'static str, Option<BasicMetadataValueEnum<'static>>)> {
        []
    }
    fn to_const_ptr(ptr: Val<'_, Self>) -> Val<'_, P<*const T, Space>> {
        unsafe { Val::transmute(ptr) }
    }
    fn load<'a>(ptr: Val<'a, Self>) -> Val<'a, T>
    where
        T: Copy + SizedTy,
    {
        let ptr_attrs = Self::load_metadata(ptr.cx());
        let ptr = Self::to_const_ptr(ptr);
        unsafe { P::read_with_instruction_metadata(ptr, ptr_attrs) }
    }
    fn reborrow<'a, 'c>(ptr: &'c Val<'a, Self>) -> Val<'a, R<&'c T, Space>>
    where
        'b: 'c,
    {
        fn is_clone<T: Copy>(_: T) {}
        is_clone(*ptr);
        unsafe { Val::transmute(*ptr) }
    }
}

impl<'b, T: ValTy + ?Sized, Space: Addrspace> M<&'b mut T, Space> {
    fn load_metadata(
        _cx: &FnCodegen,
    ) -> impl IntoIterator<Item = (&'static str, Option<BasicMetadataValueEnum<'static>>)> {
        []
    }
    fn store_metadata(
        _cx: &FnCodegen,
    ) -> impl IntoIterator<Item = (&'static str, Option<BasicMetadataValueEnum<'static>>)> {
        []
    }
    fn to_ref(val: Val<'_, Self>) -> Val<'_, R<&'b T, Space>> {
        unsafe { Val::transmute(val) }
    }
    fn to_const_ptr(val: Val<'_, Self>) -> Val<'_, P<*const T, Space>> {
        R::to_const_ptr(Self::to_ref(val))
    }
    fn to_mut_ptr(val: Val<'_, Self>) -> Val<'_, P<*mut T, Space>> {
        unsafe { Val::transmute(val) }
    }
    unsafe fn from_mut_ptr(ptr: Val<'_, P<*mut T, Space>>) -> Val<'_, Self> {
        unsafe { Val::transmute(ptr) }
    }
    fn reborrow<'a, 'c>(ptr: &'c Val<'a, Self>) -> Val<'a, R<&'c T, Space>>
    where
        'b: 'c,
    {
        unsafe { Val::new(ptr.cx(), ptr.raw()) }
    }
    fn reborrow_mut<'a, 'c>(ptr: &'c mut Val<'a, Self>) -> Val<'a, M<&'c mut T, Space>> {
        unsafe { Val::new(ptr.cx(), ptr.raw()) }
    }

    fn load<'a>(ptr: Val<'a, Self>) -> Val<'a, T>
    where
        T: Copy + SizedTy,
    {
        let load_metadata = Self::load_metadata(ptr.cx());
        let ptr = Self::to_const_ptr(ptr);
        unsafe { P::read_with_instruction_metadata(ptr, load_metadata) }
    }

    fn swap<'a>(ptr: Val<'a, Self>, val: Val<'a, T>) -> Val<'a, T>
    where
        T: SizedTy,
    {
        let read_metadata = Self::load_metadata(ptr.cx());
        let read_ptr = R::to_const_ptr(Self::reborrow(&ptr));
        let ret = unsafe { P::read_with_instruction_metadata(read_ptr, read_metadata) };

        let write_metadata = Self::store_metadata(ptr.cx());
        let write_ptr = Self::to_mut_ptr(ptr);
        unsafe { P::write_with_instruction_metadata(write_ptr, val, write_metadata) };

        ret
    }

    fn store<'a>(ptr: Val<'a, Self>, val: Val<'a, T>)
    where
        T: SizedTy,
    {
        if T::NEEDS_DROP {
            let mut res = Self::swap(ptr, val);
            T::inner_drop(&mut res);
        } else {
            let write_metadata = Self::store_metadata(ptr.cx());
            let write_ptr = Self::to_mut_ptr(ptr);
            unsafe { P::write_with_instruction_metadata(write_ptr, val, write_metadata) };
        }
    }
}

impl<'a, T: ?Sized, Space: Addrspace> Val<'a, P<*const T, Space>>
where
    T: AnyTy,
{
    pub fn addrspace_cast<OtherSpace: Addrspace>(self) -> Val<'a, P<*const T, OtherSpace>> {
        let raw_ptr = self.ll_typed();
        let new_ptr_type = P::<*const T, OtherSpace>::ty(self.ctx());
        let new_ptr = unsafe {
            self.cx().with_builder(|b| {
                b.build_address_space_cast(raw_ptr, new_ptr_type, "addrspace_cast")
            })
        }
        .expect("Address space cast should work");
        unsafe { Val::new(self.cx(), new_ptr.as_basic_value_enum()) }
    }
    pub fn ptr_cast<U: AnyTy + ?Sized>(self) -> Val<'a, P<*const U, Space>> {
        P::cast(self)
    }
    pub fn as_mut(self) -> Val<'a, P<*mut T, Space>> {
        P::to_mut_ptr(self)
    }
    pub unsafe fn as_ref_unchecked<'b>(self) -> Val<'a, R<&'b T, Space>>
    where
        T: ValTy,
    {
        unsafe { P::to_ref_unchecked(self) }
    }

    pub unsafe fn read_unaligned(self) -> Val<'a, T>
    where
        T: SizedTy,
    {
        unsafe { P::read_unaligned(self) }
    }
    pub unsafe fn read(self) -> Val<'a, T>
    where
        T: SizedTy,
    {
        unsafe { P::read(self) }
    }
}

impl<'a, T, Space: Addrspace> Val<'a, P<*mut T, Space>>
where
    T: AnyTy,
{
    pub fn addrspace_cast<OtherSpace: Addrspace>(self) -> Val<'a, P<*mut T, OtherSpace>> {
        self.as_const().addrspace_cast().as_mut()
    }
    pub fn ptr_cast<U: AnyTy>(self) -> Val<'a, P<*mut U, Space>> {
        P::cast_mut(self)
    }
    pub fn as_const(self) -> Val<'a, P<*const T, Space>> {
        P::to_const_ptr(self)
    }
    pub unsafe fn as_mut_unchecked<'b>(self) -> Val<'a, M<&'b mut T, Space>>
    where
        T: ValTy,
    {
        unsafe { P::to_mut_unchecked(self) }
    }

    pub unsafe fn read_unaligned(self) -> Val<'a, T>
    where
        T: SizedTy,
    {
        unsafe { P::read_unaligned(self.as_const()) }
    }

    pub unsafe fn read(self) -> Val<'a, T>
    where
        T: SizedTy,
    {
        unsafe { P::read(self.as_const()) }
    }

    pub unsafe fn write_unaligned(self, val: Val<'a, T>)
    where
        T: SizedTy,
    {
        unsafe { P::write_unaligned(self, val) }
    }
    pub unsafe fn write(self, val: Val<'a, T>)
    where
        T: SizedTy,
    {
        unsafe { P::write(self, val) }
    }
}

impl<'a, 'b, T: ValTy + ?Sized, Space: Addrspace> Val<'a, R<&'b T, Space>> {
    pub fn as_ptr(&self) -> Val<'a, P<*const T, Space>> {
        R::to_const_ptr(self.reborrow())
    }
    pub fn reborrow<'c>(&'c self) -> Val<'a, R<&'c T, Space>> {
        R::reborrow(self)
    }
    pub fn load(&self) -> Val<'a, T>
    where
        T: Copy + SizedTy,
    {
        R::load(self.reborrow())
    }
}

impl<'a, 'b, T: ValTy + ?Sized, Space: Addrspace> Val<'a, M<&'b mut T, Space>> {
    pub fn reborrow<'c>(&'c self) -> Val<'a, R<&'c T, Space>> {
        M::reborrow(self)
    }
    pub fn reborrow_mut<'c>(&'c mut self) -> Val<'a, M<&'c mut T, Space>> {
        M::reborrow_mut(self)
    }
    pub fn as_ptr(&self) -> Val<'a, P<*const T, Space>> {
        self.reborrow().as_ptr()
    }
    pub fn as_ptr_mut(&self) -> Val<'a, P<*mut T, Space>> {
        self.as_ptr().as_mut()
    }
    pub fn as_ref(self) -> Val<'a, R<&'b T, Space>> {
        M::to_ref(self)
    }

    pub fn load(&self) -> Val<'a, T>
    where
        T: Copy + SizedTy,
    {
        M::load(unsafe { M::from_mut_ptr(self.as_ptr_mut()) })
    }

    pub fn store(&mut self, val: Val<'a, T>)
    where
        T: SizedTy,
    {
        M::store(self.reborrow_mut(), val);
    }

    pub fn swap(&mut self, val: Val<'a, T>) -> Val<'a, T>
    where
        T: SizedTy,
    {
        M::swap(self.reborrow_mut(), val)
    }
}
