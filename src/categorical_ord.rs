use crate::Categorical;
use num_traits::{NumAssignRef, NumRef};
use std::collections::{btree_map, BTreeMap};

/// A [Categorical](crate::Categorical) that performs deduplication using an ordering
///
/// Currently, this is implemented using a B-Tree.
pub struct CategoricalOrd<T: Ord, P>(BTreeMap<T, P>);

impl<T: Ord, P> Categorical<T, P> for CategoricalOrd<T, P>
where
    T: Eq,
    P: NumAssignRef + NumRef + Clone,
{
    fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (&'a T, &'a P)>
    where
        T: 'a,
        P: 'a,
    {
        self.0.iter()
    }

    fn probabilities_mut<'a>(&'a mut self) -> impl 'a + Iterator<Item = &'a mut P>
    where
        T: 'a,
        P: 'a,
    {
        self.0.iter_mut().map(|x| x.1)
    }

    fn probability_of(&self, x: &T) -> P {
        self.0[x].clone()
    }
}

impl<T: Ord, P: NumAssignRef + NumRef + Clone> FromIterator<(T, P)> for CategoricalOrd<T, P> {
    fn from_iter<I: IntoIterator<Item = (T, P)>>(iter: I) -> Self {
        let mut out = BTreeMap::new();
        for (t, p) in iter {
            match out.entry(t) {
                btree_map::Entry::Vacant(x) => {
                    x.insert(p);
                }
                btree_map::Entry::Occupied(mut x) => {
                    *x.get_mut() += &p;
                }
            }
        }
        Self(out)
    }
}

impl<T: Ord, P> IntoIterator for CategoricalOrd<T, P> {
    type Item = (T, P);
    type IntoIter = <BTreeMap<T, P> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
