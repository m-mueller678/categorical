//! This crate provides a type representing a categorical probability distribution: `Categorical<T,P>`.
//! A `Categorical` is a collection of objects of type `T`, each associated with a probability of type `P`.
//! You can combine two `Categoricals` and compute the probability of each combination (assuming the two distributions are sampled independently).
//!
//! ```rust
//! use categorical::{Categorical, CategoricalHash};
//! let die_roll = CategoricalHash::new_uniform(vec![1, 2, 3, 4, 5, 6].into_iter());
//! // roll two dice and pick the higher number
//! let max_of_two = CategoricalHash::combined(&die_roll, &die_roll, |&a, &b| a.max(b));
//! let double_wins: f64 =
//!     CategoricalHash::combined(&max_of_two, &die_roll, |double, single| double > single)
//!         .probability_of(&true);
//! println!("player rolling two dice rolls higher with probability of {double_wins}");
//! ```

use num_traits::{NumAssignRef, NumRef};

mod categorical_hash;
mod categorical_ord;
mod categorical_vec;

pub use categorical_hash::CategoricalHash;
pub use categorical_ord::CategoricalOrd;
pub use categorical_vec::CategoricalVec;

/// Describes a categorical distribution over values of `T`.
///
/// Ideally, the sum of probabilities should be 1, however this is not enforced.
/// You may use [normalize_in_place](Self::normalize_in_place) to rescale probabilities such that they add up to 1.
pub trait Categorical<T, P>: FromIterator<(T, P)> + IntoIterator<Item = (T, P)>
where
    P: NumAssignRef + NumRef + Clone,
{
    fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (&'a T, &'a P)>
    where
        T: 'a,
        P: 'a;
    fn probabilities_mut<'a>(&'a mut self) -> impl 'a + Iterator<Item = &'a mut P>
    where
        P: 'a;

    /// Construct a `Categorical` with same probability for each category.
    fn new_uniform(it: impl Iterator<Item = T>) -> Self {
        let mut ret: Self = it.zip(std::iter::repeat_with(P::one)).collect();
        ret.normalize_in_place();
        ret
    }

    /// Returns the probability of some category.
    fn probability_of(&self, x: &T) -> P;

    /// Rescale probabilities such that they sum up to 1.
    ///
    /// This function does not take numerical issues like floating point inaccuracy into consideration.
    /// If the probabilities sum to zero, this may panic or produce other undesirable effects.
    /// A reference to self is returned for convenient chaining.
    fn normalize_in_place(&mut self) -> &mut Self {
        let reciprocal_total = P::one() / self.iter().fold(P::zero(), |a, b| a + b.1);
        for p in self.probabilities_mut() {
            *p *= &reciprocal_total;
        }
        self
    }

    /// Combine with a different `Categorical` using a function that combines pairs of categories.
    ///
    /// Output probabilities are computed assuming the two distributions are independent.
    fn combined<T1, C1, T2, C2, F>(c1: &C1, other: &C2, mut f: F) -> Self
    where
        C1: Categorical<T1, P>,
        C2: Categorical<T2, P>,
        F: FnMut(&T1, &T2) -> T,
    {
        c1.iter()
            .flat_map(move |(t1, p1)| {
                other
                    .iter()
                    .map(move |(t2, p2)| ((t1, t2), p1.clone() * p2))
            })
            .map(|((t1, t2), p)| (f(t1, t2), p))
            .collect()
    }
}

/// Builds Categorical with a single category: () with probability 1.
pub fn unit_categorical<C: Categorical<(), P>, P: NumAssignRef + NumRef + Clone>() -> C {
    std::iter::once(((), P::one())).collect()
}
