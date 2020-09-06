//! DSMSG: Random generator for Messages that would be found in the Dark Souls
//!     series.

#[macro_use]
extern crate lazy_static;
extern crate rand;

mod messages;
#[cfg(test)]
mod test;
mod util;

use messages::*;
use rand::thread_rng;


fn main() {
    println!("{}", random_message(&mut thread_rng()));
}
