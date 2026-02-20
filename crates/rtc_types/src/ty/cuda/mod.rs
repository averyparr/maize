use std::marker::PhantomData;

use inkwell::{AddressSpace, values::BasicValue};

use crate::{
    ty::{AddrspacePtr, ConstPtrTy, R, RefTy, SizedTy, ValTy},
    val::Val,
};

macro_rules! addrspace_ptrs {
    ($($ptr: ident => $addrspace: literal;)*) => {
        $(pub struct $ptr<Ptr>(PhantomData<Ptr>);
        impl<Ptr> AddrspacePtr for $ptr<Ptr>
        where
            Ptr: ConstPtrTy,
        {
            type Inner = Ptr;
            const ADDRSPACE: u16 = $addrspace;
        })*
    };
}

addrspace_ptrs!(
    Global => 1;
    Shared => 3;
    Constant => 4;
    Local => 5;
    Tensor => 6;
    Cluster => 7;
);

impl<'a, T> Val<'a, Global<R<&T>>> {
    pub fn load_nc(&self) -> Val<'a, T>
    where
        T: SizedTy + Copy,
    {
        let res = Global::load(self);
        let ins = res
            .ll_typed()
            .as_instruction_value()
            .expect("Load should always be an instruction");
        let metadata = self.ctx().metadata_node(&[]);
        let kind_id = self.ctx().get_kind_id("invariant.load");
        ins.set_metadata(metadata, kind_id)
            .expect("Setting metadata should succeed");
        res
    }
}
