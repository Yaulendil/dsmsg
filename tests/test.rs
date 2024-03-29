use dsmsg::*;
use rand::thread_rng;


const ITERS: usize = 300;
const DIGIT: usize = 3;


#[test]
fn test_rng() {
    for i in 1..=ITERS {
        println!(
            "?-{i:0d$}: {txt}",
            i = i, d = DIGIT, txt = random_message(),
        );
    }
}


#[cfg(feature = "demons")]
#[test]
fn test_des() {
    for i in 1..=ITERS {
        println!(
            "0-{i:0d$}: {txt}",
            i = i, d = DIGIT, txt = MessageDeS::random(&mut thread_rng()),
        );
    }
}


#[cfg(feature = "ds1")]
#[test]
fn test_ds1() {
    for i in 1..=ITERS {
        println!(
            "1-{i:0d$}: {txt}",
            i = i, d = DIGIT, txt = MessageDkS1::random(&mut thread_rng()),
        );
    }
}


#[cfg(feature = "ds2")]
#[test]
fn test_ds2() {
    for i in 1..=ITERS {
        println!(
            "2-{i:0d$}: {txt}",
            i = i, d = DIGIT, txt = MessageDkS2::random(&mut thread_rng()),
        );
    }
}


#[cfg(feature = "ds3")]
#[test]
fn test_ds3() {
    for i in 1..=ITERS {
        println!(
            "3-{i:0d$}: {txt}",
            i = i, d = DIGIT, txt = MessageDkS3::random(&mut thread_rng()),
        );
    }
}


#[cfg(feature = "bloodborne")]
#[test]
fn test_bb() {
    for i in 1..=ITERS {
        println!(
            "B-{i:0d$}: {txt}",
            i = i, d = DIGIT, txt = MessageBB::random(&mut thread_rng()),
        );
    }
}


#[cfg(feature = "eldenring")]
#[test]
fn test_er1() {
    for i in 1..=ITERS {
        println!(
            "E-{i:0d$}: {txt}",
            i = i, d = DIGIT, txt = MessageEr1::random(&mut thread_rng()),
        );
    }
}


#[cfg(feature = "sekiro")]
#[test]
fn test_sek() {
    for i in 1..=ITERS {
        println!(
            "S-{i:0d$}: {txt}",
            i = i, d = DIGIT, txt = MessageSek::random(&mut thread_rng()),
        );
    }
}
