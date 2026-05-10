use crate::layout::{shape::Shape, stride::Stride, tuple::TupleWalkError};

pub mod algorithm;
pub mod shape;
pub mod stride;
pub mod tile;
pub mod tuple;

#[derive(Clone, PartialEq, Debug)]
pub struct Layout(Shape, Stride);

impl Layout {
    pub fn combine(left: Self, right: Self) -> Self {
        let (lshape, lstride) = left.decompose();
        let (rshape, rstride) = right.decompose();
        let shape = Shape::new((lshape.inner(), rshape.inner()));
        let stride = Stride::new((lstride.inner(), rstride.inner()));
        Self(shape, stride)
    }
    pub fn try_new(shape: Shape, stride: Stride) -> Result<Self, TupleWalkError> {
        match shape.get().same_topology(stride.get()) {
            Ok(()) => Ok(Self(shape, stride)),
            Err(mut e) => {
                e.trace.reverse();
                Err(e)
            }
        }
    }

    pub fn new(shape: Shape, stride: Stride) -> Self {
        Self::try_new(shape, stride).expect("Invalid shape/stride passed!")
    }

    pub fn shape(&self) -> &Shape {
        &self.0
    }
    pub fn shape_mut(&mut self) -> &mut Shape {
        &mut self.0
    }
    pub fn stride(&self) -> &Stride {
        &self.1
    }
    pub fn stride_mut(&mut self) -> &mut Stride {
        &mut self.1
    }
    pub fn decompose(self) -> (Shape, Stride) {
        (self.0, self.1)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn same_topology(&self, other: &Self) -> Result<(), TupleWalkError> {
        self.shape().same_topology(other.shape())
    }
    pub fn canonicalize_(&mut self) {
        self.0.canonicalize_();
        self.1.canonicalize_();
    }
    pub fn canonicalize(self) -> Self {
        Self(self.0.canonicalize(), self.1.canonicalize())
    }
}

impl ToString for Layout {
    fn to_string(&self) -> String {
        format!("{}:{}", self.0.get().to_string(), self.1.get().to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::layout::tuple::{Tuple, TupleOrValue};

    use super::*;

    #[test]
    fn test_layout() {
        let mut shape = Shape::new((3, 5, Shape::new((5, 2))));
        let mut stride = Stride::new((4, 5, Stride::new((5, 2))));
        let _ = Layout::try_new(shape.clone(), stride.clone()).expect("Should be a valid layout");
        *stride.get_mut().index_mut(1).unwrap() = TupleOrValue::Tuple(Tuple::new((3, 5)));
        assert_eq!(
            Layout::try_new(shape.clone(), stride.clone()),
            Err(TupleWalkError {
                trace: vec![1],
                reason: tuple::TupleWalkErrorReason::MismatchedTopology(0, 1)
            })
        );
        *shape.get_mut().index_mut(1).unwrap() = TupleOrValue::Tuple(Tuple::new((3, 5)));
        if let TupleOrValue::Tuple(t) = shape.get_mut().index_mut(2).unwrap() {
            t.append(Tuple::new((5,)));
        }
        assert_eq!(
            Layout::try_new(shape.clone(), stride.clone()),
            Err(TupleWalkError {
                reason: tuple::TupleWalkErrorReason::MismatchedLength(0, 1),
                trace: vec![2]
            })
        );
        if let TupleOrValue::Tuple(t) = stride.get_mut().index_mut(2).unwrap() {
            t.append(5);
        }
        assert_eq!(
            Layout::try_new(shape.clone(), stride.clone()),
            Err(TupleWalkError {
                reason: tuple::TupleWalkErrorReason::MismatchedTopology(0, 1),
                trace: vec![2, 2]
            })
        );
    }
}
