//! Provider of [`OwnDokStrage`].

use std::collections::BTreeMap;
use std::collections::btree_map::{Entry, Iter, IterMut, OccupiedEntry};
use crate::aliases::{Pos, Size};
use crate::parts::Scalar;

/// Self owning matrix strage with dictionary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnDokStrage<T> {
    pub(crate) map: BTreeMap<Pos, T>,
    pub(crate) size: Size,
}

impl<T> OwnDokStrage<T>
where
    T: Scalar,
{
    /// Creates a new value.
    pub fn new(size: Size) -> Self {
        let map = BTreeMap::new();
        Self { map, size }
    }

    /// Returns iterator over the components.
    pub fn iter(&self) -> Iter<Pos, T> {
        self.map.iter()
    }

    /// Returns entry at specified position.
    pub fn at_mut(&mut self, pos: Pos) -> OccupiedEntry<'_, Pos, T> {
        /* Waiting new feature `BTreeMap::insert_entry`.
        Entry search after entry insert is inefficient.
        https://github.com/rust-lang/rust/issues/65225
        */
        self.map.entry(pos).or_insert(*T::zero());
        let Entry::Occupied(entry) = self.map.entry(pos) else {
            panic!()
        };
        entry
    }

    /// Returns mutable iterator over the components.
    pub fn iter_mut(&mut self) -> IterMut<Pos, T> {
        self.map.iter_mut()
    }
}
