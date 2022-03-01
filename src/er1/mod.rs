//! Generate Messages from Elden Ring.

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
            self.main.replace('\x1F', self.word).fmt(f)
        }
    }
}


/// A complete Hint Message that could be found in Elden Ring. Consists of
///     either one or two parts. If there are two, there will also be a
///     Conjunction string to join them.
pub struct MessageEr1 {
    p1: Segment,
    p2: Option<(&'static str, Segment)>,
}

impl DsMulti for MessageEr1 {
    /// Create a `MessageEr1` with two parts.
    fn double(rng: &mut ThreadRng) -> Self {
        Self {
            p1: Segment::random(rng),
            p2: Some((CONJUNCTIONS.choose(rng).unwrap(), Segment::random(rng))),
        }
    }

    /// Create a `MessageEr1` with one part.
    fn single(rng: &mut ThreadRng) -> Self {
        Self {
            p1: Segment::random(rng),
            p2: None,
        }
    }
}

impl Display for MessageEr1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.p2 {
            Some((conj, second)) => write!(f, "{}{}{}", &self.p1, conj, second),
            None => self.p1.fmt(f),
        }
    }
}
