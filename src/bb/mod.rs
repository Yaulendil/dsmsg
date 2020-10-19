//! Generate Messages from Bloodborne.

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
        let main: &str = TEMPLATES.choose(rng).unwrap();
        let word: &str = if main.contains('\x1F') {
            WORDS.choose(rng).unwrap()
        } else {
            ""
        };

        Self { main, word }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        //  This implementation can be slimmer than the ones found in the DkS2
        //      and DkS3 Segments, because in Bloodborne, the "musing" style
        //      message fragments are implemented as Templates, rather than as
        //      fillers for otherwise-empty Templates.
        match self.main.find('\x1F') {
            Some(i) => write!(
                f, "{}{}{}",
                &self.main[..i], &self.word, &self.main[i + 1..],
            ),
            None => self.main.fmt(f),
        }
    }
}


/// A complete Hint Message that could be found in Bloodborne. Consists of
///     either one or two parts. If there are two, there will also be a
///     Conjunction string to join them.
pub struct MessageBB {
    p1: Segment,
    p2: Option<(&'static str, Segment)>,
}

impl DsMulti for MessageBB {
    /// Create a `MessageBB` with two parts.
    fn double(rng: &mut ThreadRng) -> Self {
        Self {
            p1: Segment::random(rng),
            p2: Some((CONJUNCTIONS.choose(rng).unwrap(), Segment::random(rng))),
        }
    }

    /// Create a `MessageBB` with one part.
    fn single(rng: &mut ThreadRng) -> Self {
        Self {
            p1: Segment::random(rng),
            p2: None,
        }
    }
}

impl Display for MessageBB {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.p2 {
            Some((conj, second)) => write!(f, "{}{}{}", &self.p1, conj, second),
            None => self.p1.fmt(f),
        }
    }
}
