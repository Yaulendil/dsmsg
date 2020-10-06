//! Generate Messages from Bloodborne.

mod data;

use rand::prelude::{SliceRandom, ThreadRng};

use crate::util::compound;
use data::{CONJUNCTIONS, TEMPLATES, WORDS};
use super::{DsMsg, DsMulti};


/// A template, combined with a phrase to be inserted into it. Represents a
///     single complete thought.
struct Segment { main: &'static str, word: &'static str }

impl Segment {
    /// Create a new `Segment`, with a random Template and a random Word.
    pub fn random(rng: &mut ThreadRng) -> Self {
        Self {
            main: TEMPLATES.choose(rng).unwrap(),
            word: WORDS.choose(rng).unwrap(),
        }
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.main.find('\x1F') {
            Some(i) => write!(f, "{}{}{}", &self.main[..i], &self.word, &self.main[i + 1..]),
            None => write!(f, "{}", &self.main),
        }
    }
}


/// A complete Hint Message that could be found in Bloodborne. Consists of
///     either one or two parts. If there are two, there will also be a
///     Conjunction string to join them.
pub struct Message {
    p1: Segment,
    p2: Option<(&'static str, Segment)>,
}

impl DsMulti for Message {
    /// Create a `Message` with two parts.
    fn double(rng: &mut ThreadRng) -> Self {
        Self {
            p1: Segment::random(rng),
            p2: Some((CONJUNCTIONS.choose(rng).unwrap(), Segment::random(rng))),
        }
    }

    /// Create a `Message` with one part.
    fn single(rng: &mut ThreadRng) -> Self {
        Self {
            p1: Segment::random(rng),
            p2: None,
        }
    }
}

impl DsMsg for Message {
    /// Create a new `Message`, with at least one randomized string. There is a
    ///     chance it will also contain a second part, joined to the first by a
    ///     Conjunction.
    fn random(rng: &mut ThreadRng) -> Self {
        if compound(rng) {
            Self::double(rng)
        } else {
            Self::single(rng)
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.p2 {
            Some((conj, second)) => write!(f, "{}{}{}", &self.p1, conj, second),
            None => write!(f, "{}", &self.p1),
        }
    }
}
