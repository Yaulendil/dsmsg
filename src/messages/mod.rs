#[cfg(not(any(
    feature="bloodborne",
    feature="ds1", feature="ds2", feature="ds3",
)))]
compile_error!("Cannot compile without any Message Sets enabled.");

#[cfg(feature="bloodborne")]
mod bb;
#[cfg(feature="ds1")]
mod ds1;
#[cfg(feature="ds2")]
mod ds2;
#[cfg(feature="ds3")]
mod ds3;

#[cfg(feature="bloodborne")]
pub use bb::Message as MessageBlood;
#[cfg(feature="ds1")]
pub use ds1::Message as MessageDS1;
#[cfg(feature="ds2")]
pub use ds2::Message as MessageDS2;
#[cfg(feature="ds3")]
pub use ds3::Message as MessageDS3;

use rand::prelude::{SliceRandom, ThreadRng};


pub const GENERATORS: &[fn(&mut ThreadRng) -> String] = &[
    #[cfg(feature="bloodborne")]
    |r| MessageBlood::random(r).to_string(),
    #[cfg(feature="ds1")]
    |r| MessageDS1::random(r).to_string(),
    #[cfg(feature="ds2")]
    |r| MessageDS2::random(r).to_string(),
    #[cfg(feature="ds3")]
    |r| MessageDS3::random(r).to_string(),
];


pub fn random_message(rng: &mut ThreadRng) -> String {
    GENERATORS.choose(rng).unwrap()(rng)
}
