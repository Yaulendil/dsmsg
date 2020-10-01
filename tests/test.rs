use rand::thread_rng;
use dsmsg::*;


const ITERS: usize = 300;
const DIGIT: usize = 3;


#[cfg(feature="demons")]
#[test]
fn test_des() {
    for i in 1..=ITERS {
        println!(
            "0-{i:0d$}: {txt}",
            i=i, d=DIGIT, txt=MessageDemons::random(&mut thread_rng()),
        );
    }
}


#[cfg(feature="ds1")]
#[test]
fn test_ds1() {
    for i in 1..=ITERS {
        println!(
            "1-{i:0d$}: {txt}",
            i=i, d=DIGIT, txt=MessageDkS1::random(&mut thread_rng()),
        );
    }
}


#[cfg(feature="ds2")]
#[test]
fn test_ds2() {
    for i in 1..=ITERS {
        println!(
            "2-{i:0d$}: {txt}",
            i=i, d=DIGIT, txt=MessageDkS2::random(&mut thread_rng()),
        );
    }
}


#[cfg(feature="ds3")]
#[test]
fn test_ds3() {
    for i in 1..=ITERS {
        println!(
            "3-{i:0d$}: {txt}",
            i=i, d=DIGIT, txt=MessageDkS3::random(&mut thread_rng()),
        );
    }
}


#[cfg(feature="bloodborne")]
#[test]
fn test_bb() {
    for i in 1..=ITERS {
        println!(
            "B-{i:0d$}: {txt}",
            i=i, d=DIGIT, txt=MessageBlood::random(&mut thread_rng()),
        );
    }
}