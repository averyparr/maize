#[derive(Clone, PartialEq, Debug)]
pub enum TupleOrValue<T> {
    Tuple(Tuple<T>),
    Value(T),
}

impl<T> TupleOrValue<T> {
    pub fn canonicalize_(&mut self) {
        let to_swap = match self {
            TupleOrValue::Tuple(tuple) => {
                if tuple.len() == 1
                    && let Some(val) = tuple.0.drain(0..).next()
                {
                    Some(val)
                } else {
                    tuple.canonicalize_();
                    None
                }
            }
            TupleOrValue::Value(_) => None,
        };
        if let Some(swp) = to_swap {
            *self = swp;
        }
    }
}

impl<T> TupleOrValue<T> {
    pub fn matches<U>(&self, rhs: &TupleOrValue<U>) -> bool {
        use TupleOrValue as T;
        match (self, rhs) {
            (T::Tuple(_), T::Tuple(_)) | (T::Value(_), T::Value(_)) => true,
            (T::Value(_), T::Tuple(_)) | (T::Tuple(_), T::Value(_)) => false,
        }
    }
    pub fn is_tuple(&self) -> bool {
        matches!(self, TupleOrValue::Tuple(_))
    }
    pub fn tuple(&self) -> &Tuple<T> {
        match self {
            TupleOrValue::Tuple(tuple) => tuple,
            TupleOrValue::Value(_) => panic!("Requested tuple when variant was value"),
        }
    }
    pub fn value(&self) -> &T {
        match self {
            TupleOrValue::Tuple(_) => panic!("Requested value when variant was tuple"),
            TupleOrValue::Value(v) => v,
        }
    }
}

impl<T> From<Tuple<T>> for TupleOrValue<T> {
    fn from(value: Tuple<T>) -> Self {
        Self::Tuple(value)
    }
}

impl<T> From<T> for TupleOrValue<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

pub trait TupleArg<T> {
    fn as_erased_vec(self) -> Vec<TupleOrValue<T>>;
}

macro_rules! impl_tuple_arg {
    ($($args: ident),*) => {
        impl<T $(, $args: Into<TupleOrValue<T>>)*> TupleArg<T> for ($($args,)*) {
            fn as_erased_vec(self) -> Vec<TupleOrValue<T>> {
                #[allow(non_snake_case)]
                let ($($args,)*) = self;
                vec![$($args.into(),)*]
            }
        }
    };
}

impl_tuple_arg!();
impl_tuple_arg!(A);
impl_tuple_arg!(A, B);
impl_tuple_arg!(A, B, C);
impl_tuple_arg!(A, B, C, D);
impl_tuple_arg!(A, B, C, D, E);
impl_tuple_arg!(A, B, C, D, E, F);
impl_tuple_arg!(A, B, C, D, E, F, G);
impl_tuple_arg!(A, B, C, D, E, F, G, H);
impl_tuple_arg!(A, B, C, D, E, F, G, H, I);
impl_tuple_arg!(A, B, C, D, E, F, G, H, I, J);

#[derive(Clone, PartialEq, Debug)]
pub struct Tuple<T>(Vec<TupleOrValue<T>>);

#[derive(Clone, PartialEq, Copy, Debug)]
#[non_exhaustive]
pub enum TupleWalkErrorReason {
    MismatchedLength(usize, usize),
    MismatchedTopology(usize, usize),
    Divisibility,
}
#[derive(Clone, PartialEq, Debug)]
pub struct TupleWalkError {
    pub(crate) reason: TupleWalkErrorReason,
    pub(crate) trace: Vec<usize>,
}

impl<T> Tuple<T> {
    #[allow(private_bounds)]
    pub fn append(&mut self, to_add: impl Into<TupleOrValue<T>>) {
        self.0.push(to_add.into());
    }

    #[allow(private_bounds)]
    pub fn new(args: impl TupleArg<T>) -> Self {
        Self(args.as_erased_vec())
    }

    pub fn walk(&self) -> impl Iterator<Item = &TupleOrValue<T>> {
        self.0.iter()
    }
    pub fn walk_mut(&mut self) -> impl Iterator<Item = &mut TupleOrValue<T>> {
        self.0.iter_mut()
    }
    pub fn into_iter(self) -> impl Iterator<Item = TupleOrValue<T>> {
        self.0.into_iter()
    }

    pub fn walk2<'a, U>(
        &'a self,
        other: &'a Tuple<U>,
    ) -> impl Iterator<Item = (&'a TupleOrValue<T>, &'a TupleOrValue<U>)> {
        self.walk().zip(other.walk())
    }
    pub fn walk3<'a, U, V>(
        &'a self,
        tup2: &'a Tuple<U>,
        tup3: &'a Tuple<V>,
    ) -> impl Iterator<
        Item = (
            &'a TupleOrValue<T>,
            &'a TupleOrValue<U>,
            &'a TupleOrValue<V>,
        ),
    > {
        self.walk2(tup2)
            .zip(tup3.walk())
            .map(|((a, b), c)| (a, b, c))
    }

    pub fn index(&self, idx: usize) -> Option<&TupleOrValue<T>> {
        if idx < self.len() {
            Some(&self.0[idx])
        } else {
            None
        }
    }
    pub fn index_mut(&mut self, idx: usize) -> Option<&mut TupleOrValue<T>> {
        if idx < self.len() {
            Some(&mut self.0[idx])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn apply<U>(&self, f: impl Fn(&T) -> U + Copy) -> Tuple<U> {
        let mut new = Tuple::new(());
        new.0.reserve(self.len());
        for item in self.walk() {
            match item {
                TupleOrValue::Tuple(tuple) => new.append(tuple.apply(f)),
                TupleOrValue::Value(v) => new.append(f(v)),
            }
        }
        new
    }

    pub fn apply_fallible<U: Copy>(
        &self,
        f: impl Fn(&T) -> Result<U, TupleWalkErrorReason> + Copy,
    ) -> Result<Tuple<U>, TupleWalkError>
    where
        T: Copy,
    {
        let mut new = Tuple::new(());
        new.0.reserve(self.len());
        for (i, item) in self.walk().enumerate() {
            match item {
                TupleOrValue::Tuple(tuple) => match tuple.apply_fallible(f) {
                    Ok(val) => {
                        new.append(val);
                    }
                    Err(mut e) => {
                        e.trace.push(i);
                        return Err(e);
                    }
                },
                TupleOrValue::Value(v) => match f(v) {
                    Ok(val) => new.append(val),
                    Err(e) => {
                        return Err(TupleWalkError {
                            reason: e,
                            trace: vec![i],
                        });
                    }
                },
            }
        }
        Ok(new)
    }

    pub fn apply2_fallible<U, W>(
        &self,
        rhs: &Tuple<U>,
        f: impl Fn(&T, &U) -> Result<W, TupleWalkErrorReason> + Copy,
    ) -> Result<Tuple<W>, TupleWalkError> {
        let mut new = Tuple::new(());
        new.0.reserve(self.len());
        let eret = |e, i| {
            Err(TupleWalkError {
                reason: e,
                trace: vec![i],
            })
        };
        for (i, (a, b)) in self.walk2(rhs).enumerate() {
            if !a.matches(b) {
                return eret(TupleWalkErrorReason::MismatchedTopology(0, 1), i);
            }
            if a.is_tuple() {
                let a = a.tuple();
                let b = b.tuple();
                if a.len() != b.len() {
                    return eret(TupleWalkErrorReason::MismatchedLength(0, 1), i);
                }
                match a.apply2_fallible(b, f) {
                    Ok(v) => new.append(v),
                    Err(mut e) => {
                        e.trace.push(i);
                        return Err(e);
                    }
                }
            } else {
                match f(a.value(), b.value()) {
                    Ok(v) => new.append(v),
                    Err(e) => return eret(e, i),
                }
            }
        }
        Ok(new)
    }

    pub fn apply3_fallible<U, V, W>(
        &self,
        tup2: &Tuple<U>,
        tup3: &Tuple<V>,
        f: impl Fn(&T, &U, &V) -> Result<W, TupleWalkErrorReason> + Copy,
    ) -> Result<Tuple<W>, TupleWalkError> {
        let mut new = Tuple::new(());
        new.0.reserve(self.len());
        let eret = |e, i| {
            Err(TupleWalkError {
                reason: e,
                trace: vec![i],
            })
        };
        for (i, (a, b, c)) in self.walk3(tup2, tup3).enumerate() {
            if !a.matches(b) {
                return eret(TupleWalkErrorReason::MismatchedTopology(0, 1), i);
            }
            if !a.matches(c) {
                return eret(TupleWalkErrorReason::MismatchedTopology(0, 2), i);
            }
            if a.is_tuple() {
                let a = a.tuple();
                let b = b.tuple();
                let c = c.tuple();
                if a.len() != b.len() {
                    return eret(TupleWalkErrorReason::MismatchedLength(0, 1), i);
                }
                if a.len() != c.len() {
                    return eret(TupleWalkErrorReason::MismatchedLength(0, 2), i);
                }
                match a.apply3_fallible(b, c, f) {
                    Ok(v) => new.append(v),
                    Err(mut e) => {
                        e.trace.push(i);
                        return Err(e);
                    }
                }
            } else {
                match f(a.value(), b.value(), c.value()) {
                    Ok(v) => new.append(v),
                    Err(e) => return eret(e, i),
                }
            }
        }
        Ok(new)
    }

    pub fn try_apply2<U, V>(
        &self,
        other: &Tuple<U>,
        f: impl Fn(&T, &U) -> V + Copy,
    ) -> Result<Tuple<V>, TupleWalkError> {
        self.apply2_fallible(other, |t, u| Ok(f(t, u)))
    }

    pub fn same_topology<U: Copy>(&self, other: &Tuple<U>) -> Result<(), TupleWalkError>
    where
        T: Copy,
    {
        self.apply2_fallible(other, |_, _| Ok(())).map(|_| ())
    }

    pub fn canonicalize(mut self) -> Self {
        self.canonicalize_();
        self
    }
    pub fn canonicalize_(&mut self) {
        for t in self.walk_mut() {
            t.canonicalize_();
        }
    }
}

impl<T> Tuple<Tuple<T>> {
    pub fn flatten(self) -> Tuple<T> {
        Tuple(
            self.0
                .into_iter()
                .map(|v| match v {
                    TupleOrValue::Tuple(tuple) => tuple.flatten(),
                    TupleOrValue::Value(t) => t,
                })
                .map(TupleOrValue::Tuple)
                .collect(),
        )
    }
    pub fn cat_flatten(self) -> Tuple<T> {
        let mut ret = Tuple::new(());
        for v in self.0.into_iter() {
            match v {
                TupleOrValue::Tuple(tuple) => ret.append(tuple.flatten()),
                TupleOrValue::Value(v) => {
                    for sv in v.0.into_iter() {
                        ret.append(sv);
                    }
                }
            }
        }
        ret
    }
}

impl<T: ToString> ToString for TupleOrValue<T> {
    fn to_string(&self) -> String {
        match self {
            TupleOrValue::Tuple(tuple) => tuple.to_string(),
            TupleOrValue::Value(val) => val.to_string(),
        }
    }
}

impl<T: ToString> ToString for Tuple<T> {
    fn to_string(&self) -> String {
        let inner = self
            .0
            .iter()
            .enumerate()
            .fold(String::new(), |mut acc, (i, x)| {
                if i > 0 {
                    acc.push_str(", ");
                }
                acc.push_str(&x.to_string());
                acc
            });
        format!("({inner})")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_constructor() {
        let t0 = Tuple::new((3, 4));
        let t1 = Tuple::new((5, 6));
        let mut t2 = Tuple::new((t0, t1));
        t2.append(3);
        assert!(
            t2 == Tuple(vec![
                TupleOrValue::Tuple(Tuple(vec![TupleOrValue::Value(3), TupleOrValue::Value(4)])),
                TupleOrValue::Tuple(Tuple(vec![TupleOrValue::Value(5), TupleOrValue::Value(6)])),
                TupleOrValue::Value(3)
            ])
        );
    }
    // TODO: test from string and to string
    // TODO: test from string failure.
}
