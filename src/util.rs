#![cfg(any(
    feature = "ds1", feature = "ds2", feature = "ds3",
    feature = "bloodborne",
))]


/// Given a `String`, replace the first character with its uppercase equivalent
///     in-place. Do not affect any other characters.
#[cfg(feature = "ds1")]
pub fn capitalize(s: &mut String) {
    if !s.is_empty() { unsafe { s.as_bytes_mut()[0].make_ascii_uppercase(); } }
}


/// Randomly choose whether a Message should have multiple components.
#[cfg(any(feature = "ds2", feature = "ds3", feature = "bloodborne"))]
pub fn compound(rng: &mut rand::prelude::ThreadRng) -> bool {
    use rand::prelude::Rng;

    rng.gen_bool(0.5)
}
