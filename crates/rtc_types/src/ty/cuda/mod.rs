use std::marker::PhantomData;

use crate::ty::{AddrspacePtr, ConstPtrTy};

macro_rules! addrspace_ptrs {
    ($($ptr: ident => $addrspace: literal;)*) => {
        #[allow(unused)]
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
