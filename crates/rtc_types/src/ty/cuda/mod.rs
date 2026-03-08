use crate::ty::Addrspace;

#[derive(Clone, Copy)]
pub struct Global;
impl Addrspace for Global {
    const AS_U16: u16 = 1;
}

#[derive(Clone, Copy)]
pub struct Shared;
impl Addrspace for Shared {
    const AS_U16: u16 = 3;
}
#[derive(Clone, Copy)]
pub struct Constant;
impl Addrspace for Constant {
    const AS_U16: u16 = 4;
}
#[derive(Clone, Copy)]
pub struct Local;
impl Addrspace for Local {
    const AS_U16: u16 = 5;
}
#[derive(Clone, Copy)]
pub struct Tensor;
impl Addrspace for Tensor {
    const AS_U16: u16 = 6;
}
#[derive(Clone, Copy)]
pub struct Cluster;
impl Addrspace for Cluster {
    const AS_U16: u16 = 7;
}
