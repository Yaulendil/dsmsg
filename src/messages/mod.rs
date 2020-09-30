#[cfg(not(any(
    feature="bloodborne", feature="demons",
    feature="ds1", feature="ds2", feature="ds3",
)))]
compile_error!("Cannot compile without any Message Sets enabled.");

#[cfg(feature="bloodborne")]
mod bb;
#[cfg(feature="demons")]
mod des;
#[cfg(feature="ds1")]
mod ds1;
#[cfg(feature="ds2")]
mod ds2;
#[cfg(feature="ds3")]
mod ds3;

#[cfg(feature="bloodborne")]
pub use bb::Message as MessageBlood;
#[cfg(feature="demons")]
pub use des::Message as MessageDemons;
#[cfg(feature="ds1")]
pub use ds1::Message as MessageDkS1;
#[cfg(feature="ds2")]
pub use ds2::Message as MessageDkS2;
#[cfg(feature="ds3")]
pub use ds3::Message as MessageDkS3;

use rand::prelude::{SliceRandom, thread_rng, ThreadRng};


/// Generators: A constant slice of small closures that each return the `String`
///     form of a random generation of their respective messages.
pub const GENERATORS: &[fn(&mut ThreadRng) -> String] = &[
    #[cfg(feature="bloodborne")]
    |r| MessageBlood::random(r).to_string(),
    #[cfg(feature="demons")]
    |r| MessageDemons::random(r).to_string(),
    #[cfg(feature="ds1")]
    |r| MessageDkS1::random(r).to_string(),
    #[cfg(feature="ds2")]
    |r| MessageDkS2::random(r).to_string(),
    #[cfg(feature="ds3")]
    |r| MessageDkS3::random(r).to_string(),
];


/// Randomly select from the `GENERATORS` slice, and run it, producing a random
///     message from a random source.
pub fn random_message() -> String {
    let mut rng: ThreadRng = thread_rng();

    GENERATORS.choose(&mut rng).unwrap()(&mut rng)
}
