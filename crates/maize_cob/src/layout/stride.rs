use crate::layout::tuple::{Tuple, TupleArg, TupleOrValue, TupleWalkError};

#[derive(Clone, PartialEq, Debug)]
pub struct Stride(Tuple<u64>);

impl Into<TupleOrValue<u64>> for Stride {
    fn into(self) -> TupleOrValue<u64> {
        self.inner().into()
    }
}

impl Stride {
    pub fn from_tuple(arg: Tuple<u64>) -> Self {
        Self(arg)
    }
    pub fn new(args: impl TupleArg<u64>) -> Self {
        Self(Tuple::new(args))
    }
    pub fn get(&self) -> &Tuple<u64> {
        &self.0
    }
    pub fn get_mut(&mut self) -> &mut Tuple<u64> {
        &mut self.0
    }
    pub fn inner(self) -> Tuple<u64> {
        self.0
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn same_topology(&self, other: &Self) -> Result<(), TupleWalkError> {
        self.get().same_topology(other.get())
    }
    pub fn canonicalize_(&mut self) {
        self.get_mut().canonicalize_();
    }
    pub fn canonicalize(mut self) -> Self {
        self.canonicalize_();
        self
    }
}
