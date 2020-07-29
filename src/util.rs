use rand::prelude::*;


/// Percentage chance for a randomly-generated `Message` to contain two
///     `Segment`s, separated by a Conjunction.
const COMPOUND_CHANCE: f64 = 0.5;


/// Given a `String`, replace the first character with its uppercase equivalent
///     in-place. Do not affect any other characters.
pub fn capitalize(s: &mut String) {
    unsafe {
        let v: &mut Vec<u8> = s.as_mut_vec();
        if v[0].is_ascii() {
            v[0] = v[0].to_ascii_uppercase();
        }
    }
}


/// Randomly return a Reference to a `&str` in a Slice.
pub fn choose<'a>(slice: &[&'a str], rng: &mut ThreadRng) -> &'a str {
    slice[rng.gen_range(0, slice.len())]
}


/// Randomly choose whether a Message should have multiple components.
pub fn compound(rng: &mut ThreadRng) -> bool {
    rng.gen_bool(COMPOUND_CHANCE)
}
