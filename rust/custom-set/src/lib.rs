use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: Hash + Eq> {
    store: HashSet<T>,
}

impl<T> CustomSet<T>
where
    T: Hash + Eq,
{
    pub fn new(list: Vec<T>) -> CustomSet<T>
    where
        T: Clone,
    {
        CustomSet {
            store: list.iter().cloned().collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        let h = &self.store;
        h.is_empty()
    }

    pub fn contains(&self, v: &T) -> bool {
        let h = &self.store;
        h.contains(v)
    }

    pub fn add(&mut self, v: T) -> bool {
        let h = &mut self.store;
        h.insert(v)
    }

    pub fn is_subset(&self, set: &CustomSet<T>) -> bool {
        let h = &self.store;
        h.is_subset(&set.store)
    }

    pub fn is_disjoint(&self, set: &CustomSet<T>) -> bool {
        let h = &self.store;
        h.is_disjoint(&set.store)
    }

    pub fn intersection(&self, set: &CustomSet<T>) -> CustomSet<T>
    where
        T: Clone,
    {
        let h = &self.store;
        CustomSet {
            store: h.intersection(&set.store).cloned().collect(),
        }
    }

    pub fn difference(&self, set: &CustomSet<T>) -> CustomSet<T>
    where
        T: Clone,
    {
        let h = &self.store;
        CustomSet {
            store: h.difference(&set.store).cloned().collect(),
        }
    }

    pub fn union(&self, set: &CustomSet<T>) -> CustomSet<T>
    where
        T: Clone,
    {
        let h = &self.store;
        CustomSet {
            store: h.union(&set.store).cloned().collect(),
        }
    }
}
