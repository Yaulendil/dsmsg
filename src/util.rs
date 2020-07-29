use rand::prelude::*;
pub use rand::prelude::ThreadRng;


/// Percentage chance for a randomly-generated `Message` to contain two
///     `Segment`s, separated by a Conjunction.
pub const COMPOUND_CHANCE: f64 = 0.5;


/// Randomly return a Reference to a `&str` in a Slice.
pub fn choose<'a>(slice: &[&'a str], rng: &mut ThreadRng) -> &'a str {
    slice[rng.gen_range(0, slice.len())]
}
