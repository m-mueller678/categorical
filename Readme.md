# Categorical

This crate provides a type representing a categorical probability distribution: `Categorical<T,P>`.
A `Categorical` is a collection of objects of type `T`, each associated with a probability of type `P`.
You can combine two `Categoricals` and compute the probability of each combination (assuming the two distributions are sampled independently).

```rust
use categorical::Categorical;
let die_roll = Categorical::new_uniform(vec![1,2,3,4,5,6]);
// roll two dice and pick the higher number
let max_of_two = die_roll.combine_hash(&die_roll,|&a,&b|a.max(b)); 
let double_wins:f64 = max_of_two.combine_hash(&die_roll,|double,single|double>single)
    .probability_of(&true);
println!("player rolling two dice rolls higher with probability of {double_wins}");
 ```