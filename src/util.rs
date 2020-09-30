use rand::prelude::*;


/// Percentage chance for a randomly-generated `Message` to contain two
///     `Segment`s, separated by a Conjunction.
#[cfg(any(feature="ds2", feature="ds3", feature="bloodborne"))]
const COMPOUND_CHANCE: f64 = 0.5;


/// Given a `String`, replace the first character with its uppercase equivalent
///     in-place. Do not affect any other characters.
#[cfg(feature="ds1")]
pub fn capitalize(s: &mut String) {
    if !s.is_empty() { unsafe { s.as_bytes_mut()[0].make_ascii_uppercase(); } }
}


/// Randomly choose whether a Message should have multiple components.
#[cfg(any(feature="ds2", feature="ds3", feature="bloodborne"))]
pub fn compound(rng: &mut ThreadRng) -> bool {
    rng.gen_bool(COMPOUND_CHANCE)
}
