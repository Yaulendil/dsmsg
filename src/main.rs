extern crate rand;

mod data;
mod msg;
#[cfg(test)]
mod test;


use rand::thread_rng;

use msg::Message;


fn main() {
    println!("{}", Message::random(&mut thread_rng()));
}
