use std::{
    borrow::Borrow,
    collections::{HashSet, TryReserveError},
    hash::{BuildHasher, Hash, RandomState},
    marker::PhantomData,
};

pub trait ToPartial<P> {
    fn to_partial(&self) -> &P;
}

impl<V: ToPartial<P>, P> ToPartial<P> for &V {
    fn to_partial(&self) -> &P {
        (*self).to_partial()
    }
}

#[derive(Debug)]
pub struct Partial<V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    value: V,
    marker: PhantomData<P>,
}

impl<V, P> Partial<V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    pub fn into_value(self) -> V {
        self.value
    }
}

impl<V, P> Hash for Partial<V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.to_partial().hash(state);
    }
}

impl<V, P> PartialEq<P> for Partial<V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    fn eq(&self, other: &P) -> bool {
        self.value.to_partial() == other
    }
}
impl<V, P> Eq for Partial<V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
}

impl<V, P> PartialEq<Partial<V, P>> for Partial<V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    fn eq(&self, other: &Partial<V, P>) -> bool {
        self.value.to_partial() == other.value.to_partial()
    }
}

impl<V, P> From<V> for Partial<V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    fn from(value: V) -> Self {
        Partial {
            value,
            marker: PhantomData,
        }
    }
}

impl<V, P> Borrow<P> for Partial<V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    fn borrow(&self) -> &P {
        self.value.to_partial()
    }
}

pub struct PartialSet<V, P, S = RandomState>
where
    V: ToPartial<P>,
    P: Hash + Eq,
    S: BuildHasher,
{
    inner: HashSet<Partial<V, P>, S>,
}

pub struct Iter<'a, V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    inner: std::collections::hash_set::Iter<'a, Partial<V, P>>,
}

pub struct Drain<'a, V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    inner: std::collections::hash_set::Drain<'a, Partial<V, P>>,
}

impl<'a, V, P> Iterator for Drain<'a, V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|p| p.value)
    }
}

impl<'a, V, P> Iterator for Iter<'a, V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|p| &p.value)
    }
}

pub struct Difference<'a, V, P, S>
where
    V: ToPartial<P>,
    P: Hash + Eq,
    S: 'a,
{
    inner: std::collections::hash_set::Difference<'a, Partial<V, P>, S>,
}

impl<'a, V, P, S> Iterator for Difference<'a, V, P, S>
where
    V: ToPartial<P>,
    P: Hash + Eq,
    S: 'a + BuildHasher,
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|v| &v.value)
    }
}

pub struct IntoIter<V, P>
where
    V: Hash + Eq + ToPartial<P>,
    P: Hash + Eq,
{
    inner: std::collections::hash_set::IntoIter<Partial<V, P>>,
}

impl<V, P> Iterator for IntoIter<V, P>
where
    V: Hash + Eq + ToPartial<P>,
    P: Hash + Eq,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|p| p.value)
    }
}

impl<V, P> PartialSet<V, P>
where
    V: ToPartial<P>,
    P: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashSet::with_capacity(capacity),
        }
    }
}

impl<V, P, S> PartialSet<V, P, S>
where
    V: ToPartial<P>,
    P: Hash + Eq,
    S: BuildHasher,
{
    pub fn with_hasher(hasher: S) -> Self {
        Self {
            inner: HashSet::with_hasher(hasher),
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self {
            inner: HashSet::with_capacity_and_hasher(capacity, hasher),
        }
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn contains(&self, partial: &P) -> bool {
        self.inner.contains(partial)
    }

    pub fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, V, P, S> {
        Difference {
            inner: self.inner.difference(&other.inner),
        }
    }

    pub fn drain(&mut self) -> Drain<V, P> {
        Drain {
            inner: self.inner.drain(),
        }
    }

    pub fn get(&self, partial: &P) -> Option<&V> {
        self.inner.get(partial).map(|v| &v.value)
    }

    pub fn hasher(&self) -> &S {
        self.inner.hasher()
    }

    pub fn insert(&mut self, value: V) -> bool {
        self.inner.insert(Partial::from(value))
    }

    // ...

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    //...

    pub fn iter(&self) -> Iter<V, P> {
        Iter {
            inner: self.inner.iter(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn remove(&mut self, partial: &P) -> bool {
        self.inner.remove(partial)
    }

    pub fn replace(&mut self, value: V) -> Option<V> {
        self.inner.replace(Partial::from(value)).map(|v| v.value)
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&V) -> bool,
    {
        self.inner.retain(|partial| f(&partial.value));
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }

    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    // ...

    pub fn take<Q>(&mut self, value: &Q) -> Option<V>
    where
        Q: ?Sized + Hash + Eq,
        Partial<V, P>: Borrow<Q>,
    {
        self.inner.take(value).map(|v| v.value)
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    // ...
}

impl<V, P, S> IntoIterator for PartialSet<V, P, S>
where
    V: Hash + Eq + ToPartial<P>,
    S: BuildHasher,
    P: Hash + Eq,
{
    type Item = V;
    type IntoIter = IntoIter<V, P>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.inner.into_iter(),
        }
    }
}
