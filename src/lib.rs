//! DSMSG: Random generator for Messages that would be found in the Dark Souls
//!     series.

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod messages;
#[cfg(test)]
mod test;
mod util;

pub use messages::*;
