# categorical

This crate provides a type representing a categorical probability distribution: `Categorical<T,P>`.
A `Categorical` is a collection of objects of type `T`, each associated with a probability of type `P`.
You can combine two `Categoricals` and compute the probability of each combination (assuming the two distributions are sampled independently).

```rust
use categorical::{Categorical, CategoricalHash};
let die_roll = CategoricalHash::new_uniform(1..=6);
// roll two dice and pick the higher number
let max_of_two = CategoricalHash::combined(&die_roll, &die_roll, |&a, &b| a.max(b));
let double_wins: f64 =
    CategoricalHash::combined(&max_of_two, &die_roll, |double, single| double > single)
        .probability_of(&true);
println!("player rolling two dice rolls higher with probability of {double_wins}");
```

License: MIT OR Apache-2.0
