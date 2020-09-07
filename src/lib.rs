//! DSMSG: Random generator for Messages that would be found in the Dark Souls
//!     series.

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod messages;
mod util;

pub use messages::*;
use rand::prelude::{Rng, ThreadRng};


pub enum Message<'m> {
    #[cfg(feature = "bloodborne")]
    BB(MessageBlood<'m>),

    #[cfg(feature = "demons")]
    DeS(MessageDemons<'m>),

    #[cfg(feature = "ds1")]
    DkS1(MessageDkS1<'m>),

    #[cfg(feature = "ds2")]
    DkS2(MessageDkS2<'m>),

    #[cfg(feature = "ds3")]
    DkS3(MessageDkS3<'m>),
}

impl Message<'_> {
    fn random(rng: &mut ThreadRng) -> Self {
        //  We cannot know what variants will be available at runtime, so just
        //      pretend that they all will be. Pick one at random, and if it is
        //      not available, try again until we pick one that is.
        //  This SHOULD not spin forever, because compile time checks elsewhere
        //      ensure that at least one of these will be enabled; However, if
        //      we are incredibly unlucky, it may spin for a while.
        //  FIXME: This is...REALLY horrible. Find a better way or just delete
        //      the function entirely.
        loop {
            match rng.gen_range(0, 5) {
                #[cfg(feature = "bloodborne")]
                0 => return Self::BB(MessageBlood::random(rng)),

                #[cfg(feature = "demons")]
                1 => return Self::DeS(MessageDemons::random(rng)),

                #[cfg(feature = "ds1")]
                2 => return Self::DkS1(MessageDkS1::random(rng)),

                #[cfg(feature = "ds2")]
                3 => return Self::DkS2(MessageDkS2::random(rng)),

                #[cfg(feature = "ds3")]
                4 => return Self::DkS3(MessageDkS3::random(rng)),

                _ => continue,
            }
        }
    }
}

impl std::fmt::Display for Message<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "bloodborne")]
            Self::BB(m) => m.fmt(f),

            #[cfg(feature = "demons")]
            Self::DeS(m) => m.fmt(f),

            #[cfg(feature = "ds1")]
            Self::DkS1(m) => m.fmt(f),

            #[cfg(feature = "ds2")]
            Self::DkS2(m) => m.fmt(f),

            #[cfg(feature = "ds3")]
            Self::DkS3(m) => m.fmt(f),
        }
    }
}
