use std::collections::{btree_map, hash_map, BTreeMap, HashMap};
use std::hash::Hash;
use std::ops::AddAssign;

pub struct Categorical<T, P> {
    options: Vec<T>,
    probabilities: Vec<P>,
}

impl<T, P> Categorical<T, P>
where
    for<'a> &'a P: std::ops::Mul<Output = P>,
    for<'a> P: AddAssign<&'a P>,
{
    pub fn new(options: Vec<T>, probabilities: Vec<P>) -> Self {
        assert_eq!(options.len(), probabilities.len());
        Self {
            options,
            probabilities,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &P)> {
        self.options.iter().zip(self.probabilities.iter())
    }

    pub fn combine_injective<U, O>(
        &self,
        other: &Categorical<U, P>,
        mut f: impl FnMut(&T, &U) -> O,
    ) {
        let mut probabilities =
            Vec::with_capacity(self.probabilities.len() * other.probabilities.len());
        let mut options = Vec::with_capacity(probabilities.capacity());
        for (a, pa) in self.iter() {
            for (b, pb) in other.iter() {
                probabilities.push(pa * pb);
                options.push(f(a, b));
            }
        }
    }

    pub fn combine_hash<U, O: Hash + Eq>(
        &self,
        other: &Categorical<U, P>,
        mut f: impl FnMut(&T, &U) -> O,
    ) {
        let mut out = HashMap::new();
        for (a, pa) in self.iter() {
            for (b, pb) in other.iter() {
                match out.entry(f(a, b)) {
                    hash_map::Entry::Vacant(x) => {
                        x.insert(pa * pb);
                    }
                    hash_map::Entry::Occupied(mut x) => {
                        *x.get_mut() += &(pa * pb);
                    }
                }
            }
        }
    }

    pub fn combine_ord<U, O: Ord + Eq>(
        &self,
        other: &Categorical<U, P>,
        mut f: impl FnMut(&T, &U) -> O,
    ) {
        let mut out = BTreeMap::new();
        for (a, pa) in self.iter() {
            for (b, pb) in other.iter() {
                match out.entry(f(a, b)) {
                    btree_map::Entry::Vacant(x) => {
                        x.insert(pa * pb);
                    }
                    btree_map::Entry::Occupied(mut x) => {
                        *x.get_mut() += &(pa * pb);
                    }
                }
            }
        }
    }
}

impl<T, P> FromIterator<(T, P)> for Categorical<T, P> {
    fn from_iter<IT: IntoIterator<Item = (T, P)>>(iter: IT) -> Self {
        let (options, probabilities) = iter.into_iter().unzip();
        Categorical {
            options,
            probabilities,
        }
    }
}
