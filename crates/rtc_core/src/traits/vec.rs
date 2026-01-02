use crate::traits::{indexes::IndexableTy, ptr::RefTy};

trait IndexableRef: RefTy<Pointee: IndexableTy> {}
