use crate::Categorical;
use num_traits::{NumAssignRef, NumRef};
use std::collections::{hash_map, HashMap};
use std::hash::Hash;

/// A [Categorical](crate::Categorical) that performs deduplication using a Hashtable
pub struct CategoricalHash<T: Hash + Eq, P>(HashMap<T, P>);

impl<T: Hash + Eq, P> Categorical<T, P> for CategoricalHash<T, P>
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

impl<T: Hash + Eq, P: NumAssignRef + NumRef + Clone> FromIterator<(T, P)>
    for CategoricalHash<T, P>
{
    fn from_iter<I: IntoIterator<Item = (T, P)>>(iter: I) -> Self {
        let mut out = HashMap::new();
        for (t, p) in iter {
            match out.entry(t) {
                hash_map::Entry::Vacant(x) => {
                    x.insert(p);
                }
                hash_map::Entry::Occupied(mut x) => {
                    *x.get_mut() += &p;
                }
            }
        }
        Self(out)
    }
}

impl<T: Hash + Eq, P> IntoIterator for CategoricalHash<T, P> {
    type Item = (T, P);
    type IntoIter = <HashMap<T, P> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
