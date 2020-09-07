#[cfg(feature="bloodborne")]
mod bb;
mod ds1;
mod ds2;
mod ds3;

#[cfg(feature="bloodborne")]
pub use bb::Message as MessageBlood;
pub use ds1::Message as MessageDS1;
pub use ds2::Message as MessageDS2;
pub use ds3::Message as MessageDS3;
use rand::prelude::{SliceRandom, ThreadRng};


pub const GENERATORS: &[fn(&mut ThreadRng) -> String] = &[
    #[cfg(feature="bloodborne")]
    |r| MessageBlood::random(r).to_string(),
    |r| MessageDS1::random(r).to_string(),
    |r| MessageDS2::random(r).to_string(),
    |r| MessageDS3::random(r).to_string(),
];


pub fn random_message(rng: &mut ThreadRng) -> String {
    GENERATORS.choose(rng).unwrap()(rng)
}
