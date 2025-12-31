use super::*;
use crate::entry::ExpirableEntry;

#[allow(clippy::enum_variant_names)]
pub(crate) enum GenericMapIter<'a, K, V> {
    BTreeMap(btree_map::Iter<'a, K, V>),
    #[cfg(feature = "std")]
    HashMap(hash_map::Iter<'a, K, V>),
    #[cfg(all(feature = "std", feature = "rustc-hash"))]
    FxHashMap(hash_map::Iter<'a, K, V>),
}

#[allow(clippy::enum_variant_names)]
pub(crate) enum GenericMapIterMut<'a, K, V> {
    BTreeMap(btree_map::IterMut<'a, K, V>),
    #[cfg(feature = "std")]
    HashMap(hash_map::IterMut<'a, K, V>),
    #[cfg(all(feature = "std", feature = "rustc-hash"))]
    FxHashMap(hash_map::IterMut<'a, K, V>),
}

#[allow(clippy::enum_variant_names)]
pub(crate) enum GenericMapIntoIter<K, V> {
    BTreeMap(btree_map::IntoIter<K, V>),
    #[cfg(feature = "std")]
    HashMap(hash_map::IntoIter<K, V>),
    #[cfg(all(feature = "std", feature = "rustc-hash"))]
    FxHashMap(hash_map::IntoIter<K, V>),
}

impl<'a, K, V> Iterator for GenericMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BTreeMap(iter) => iter.next(),
            #[cfg(feature = "std")]
            Self::HashMap(iter) => iter.next(),
            #[cfg(all(feature = "std", feature = "rustc-hash"))]
            Self::FxHashMap(iter) => iter.next(),
        }
    }
}

impl<'a, K, V> Iterator for GenericMapIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BTreeMap(iter) => iter.next(),
            #[cfg(feature = "std")]
            Self::HashMap(iter) => iter.next(),
            #[cfg(all(feature = "std", feature = "rustc-hash"))]
            Self::FxHashMap(iter) => iter.next(),
        }
    }
}

impl<K, V> Iterator for GenericMapIntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BTreeMap(iter) => iter.next(),
            #[cfg(feature = "std")]
            Self::HashMap(iter) => iter.next(),
            #[cfg(all(feature = "std", feature = "rustc-hash"))]
            Self::FxHashMap(iter) => iter.next(),
        }
    }
}

pub struct Iter<'a, K, V> {
    pub(crate) inner: GenericMapIter<'a, K, ExpirableEntry<V>>,
    pub(crate) now: u64,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (k, v) = self.inner.next()?;
            if !v.is_expired(self.now) {
                return Some((k, v.value()));
            }
        }
    }
}

pub struct IterMut<'a, K, V> {
    pub(crate) inner: GenericMapIterMut<'a, K, ExpirableEntry<V>>,
    pub(crate) now: u64,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (k, v) = self.inner.next()?;
            if !v.is_expired(self.now) {
                return Some((k, v.value_mut()));
            }
        }
    }
}

pub struct IntoIter<K, V> {
    pub(crate) inner: GenericMapIntoIter<K, ExpirableEntry<V>>,
    pub(crate) now: u64,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (k, v) = self.inner.next()?;
            if !v.is_expired(self.now) {
                return Some((k, v.owned_value()));
            }
        }
    }
}
