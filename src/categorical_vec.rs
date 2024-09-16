use crate::Categorical;
use num_traits::{NumAssignRef, NumRef};
use std::iter::Zip;

/// A [Categorical](crate::Categorical) that performs no deduplication
///
/// If this is constructed from an iterator that yields multiple instances of equal items, they will be retained as is.
/// If duplicate items are expected and items implement either [Hash] or [Ord], consider using [CategoricalHash](crate::CategoricalHash) or [CategoricalOrd](crate::CategoricalOrd).
pub struct CategoricalVec<T, P> {
    categories: Vec<T>,
    probabilities: Vec<P>,
}

impl<T, P> Categorical<T, P> for CategoricalVec<T, P>
where
    T: PartialEq,
    P: NumAssignRef + NumRef + Clone,
{
    fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (&'a T, &'a P)>
    where
        T: 'a,
        P: 'a,
    {
        self.categories.iter().zip(self.probabilities.iter())
    }

    fn probabilities_mut<'a>(&'a mut self) -> impl 'a + Iterator<Item = &'a mut P>
    where
        P: 'a,
    {
        self.probabilities.iter_mut()
    }

    fn probability_of(&self, x: &T) -> P {
        self.iter()
            .filter(|a| a.0 == x)
            .fold(P::zero(), |a, b| a + b.1)
    }
}

impl<T, P> FromIterator<(T, P)> for CategoricalVec<T, P> {
    fn from_iter<I: IntoIterator<Item = (T, P)>>(iter: I) -> Self {
        let (categories, probabilities) = iter.into_iter().unzip();
        Self {
            categories,
            probabilities,
        }
    }
}

impl<T, P> IntoIterator for CategoricalVec<T, P> {
    type Item = (T, P);
    type IntoIter = Zip<<Vec<T> as IntoIterator>::IntoIter, <Vec<P> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.categories.into_iter().zip(self.probabilities)
    }
}
