use crate::{
    backend::UntypedValue,
    tipe::{A, Ty},
    val::Val,
};

impl<T: Ty, const N: usize> Val<*const [T; N]> {
    pub fn index(self, idx: Val<u64>) -> Val<*const T> {
        let tipe = <T>::raw_ty(self.fn_ref().ctx());
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let gep =
            unsafe { b.build_in_bounds_gep(tipe, self.typed(), &[idx.typed().into()], "gep_arr") }
                .expect("GEP should have succeeded");
        unsafe { Val::new(self.fn_ref().clone(), UntypedValue(gep.into())) }
    }

    pub fn index_static(self, idx: usize) -> Val<*const T> {
        let idx = self
            .fn_ref()
            .constant(idx.try_into().expect("usize -> u32 overflow"));
        self.index(idx)
    }
}

impl<T: Ty, const N: usize, const ADDRSPACE: u16> Val<A<*const [T; N], ADDRSPACE>> {
    pub fn index(self, idx: Val<u64>) -> Val<A<*const T, ADDRSPACE>> {
        let tipe = <T>::raw_ty(self.fn_ref().ctx());
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let gep =
            unsafe { b.build_in_bounds_gep(tipe, self.typed(), &[idx.typed().into()], "gep_arr") }
                .expect("GEP should have succeeded");
        unsafe { Val::new(self.fn_ref().clone(), UntypedValue(gep.into())) }
    }

    pub fn index_static(self, idx: usize) -> Val<A<*const T, ADDRSPACE>> {
        let idx = self
            .fn_ref()
            .constant(idx.try_into().expect("usize -> u32 overflow"));
        self.index(idx)
    }
}

impl<T: Ty, const N: usize, const ADDRSPACE: u16> Val<A<*mut [T; N], ADDRSPACE>> {
    pub fn index(self, idx: Val<u64>) -> Val<A<*mut T, ADDRSPACE>> {
        self.as_const().index(idx).as_mut()
    }

    pub fn index_static(self, idx: usize) -> Val<A<*mut T, ADDRSPACE>> {
        self.as_const().index_static(idx).as_mut()
    }
}
