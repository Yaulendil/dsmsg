#![cfg(feature = "ds1")]


/// Given a `String`, replace the first character with its uppercase equivalent
///     in-place. Do not affect any other characters.
#[cfg(feature = "ds1")]
pub fn capitalize(s: &mut String) {
    if !s.is_empty() { unsafe { s.as_bytes_mut()[0].make_ascii_uppercase(); } }
}
