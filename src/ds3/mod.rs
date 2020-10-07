//! Generate Messages from Dark Souls III.

mod data;

use data::{CONJUNCTIONS, TEMPLATES, WORDS};
use rand::prelude::{SliceRandom, ThreadRng};
use std::fmt::{Display, Formatter, Result};
use super::DsMulti;


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

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.main == "\x1F" {
            self.word.fmt(f)
        } else {
            match self.main.find('\x1F') {
                Some(i) => write!(
                    f, "{}{}{}",
                    &self.main[..i], &self.word, &self.main[i + 1..],
                ),
                None => self.main.fmt(f),
            }
        }
    }
}


/// A complete Hint Message that could be found in Dark Souls III. Consists of
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

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.p2 {
            Some((conj, second)) => write!(f, "{}{}{}", &self.p1, conj, second),
            None => self.p1.fmt(f),
        }
    }
}
