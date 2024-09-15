use num_traits::{NumAssignRef, NumRef, One};
use std::collections::{btree_map, hash_map, BTreeMap, HashMap};
use std::hash::Hash;

/// Describes a categorical distribution over values of `T`.
///
/// Ideally, the sum of probabilities should be 1, however this is not enforced.
/// You may use [normalize_in_place] to rescale probabilities such that they add up to 1.
///
/// It is possible but probably not useful to construct a `Categorical` with duplicate items, that is multiple pairs with the same `T`.
pub struct Categorical<T, P> {
    categories: Vec<T>,
    probabilities: Vec<P>,
}

impl<P: One> Default for Categorical<(), P> {
    /// A Categorical with a single category: () with probability 1.
    fn default() -> Self {
        Self {
            categories: vec![()],
            probabilities: vec![P::one()],
        }
    }
}

impl<T, P: NumAssignRef + NumRef + Clone> Categorical<T, P> {
    /// Construct a distribution with given categories and probabilities.
    /// # Panics
    /// Panics if the vectors have different length.
    pub fn new(categories: Vec<T>, probabilities: Vec<P>) -> Self {
        assert_eq!(categories.len(), probabilities.len());
        Self {
            categories,
            probabilities,
        }
    }

    /// Rescale probabilities such that they sum up to 1.
    ///
    /// This function does not take numerical issues like floating point inaccuracy into consideration.
    /// If the probabilities sum to zero, this may panic or produce other undesirable effects.
    /// A reference to self is returned for convenient chaining.
    pub fn normalize_in_place(&mut self) -> &mut Self {
        let reciprocal_total = P::one() / self.probabilities.iter().fold(P::zero(), |a, b| a + b);
        for p in &mut self.probabilities {
            *p *= &reciprocal_total;
        }
        self
    }

    /// Iterate pairs of categories and probabilities by reference
    pub fn iter(&self) -> impl Iterator<Item = (&T, &P)> {
        self.categories.iter().zip(self.probabilities.iter())
    }

    /// Combine with a different `Categorical`.
    ///
    /// Output probabilities are computed assuming the two distributions are independent.
    /// This does not combine equal output from different invocations to `f`.
    /// If `f` is not injective, this may cause duplicate items.
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
                probabilities.push(pa.clone() * pb);
                options.push(f(a, b));
            }
        }
    }

    /// Like [combine_injective](Self::combine_injective), but combines equal items.
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
                        x.insert(pa.clone() * pb);
                    }
                    hash_map::Entry::Occupied(mut x) => {
                        *x.get_mut() += &(pa.clone() * pb);
                    }
                }
            }
        }
    }

    /// Like [combine_injective](Self::combine_injective), but combines equal items.
    /// If T implements [Hash], you probably should use [combine_hash](Self::combine_hash) instead.
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
                        x.insert(pa.clone() * pb);
                    }
                    btree_map::Entry::Occupied(mut x) => {
                        *x.get_mut() += &(pa.clone() * pb);
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
            categories: options,
            probabilities,
        }
    }
}

impl<T, P> IntoIterator for Categorical<T, P> {
    type Item = (T, P);
    type IntoIter =
        std::iter::Zip<<Vec<T> as IntoIterator>::IntoIter, <Vec<P> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.categories.into_iter().zip(self.probabilities)
    }
}
