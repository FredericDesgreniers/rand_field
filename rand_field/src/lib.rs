extern crate rand;
use rand::Rng;

pub trait RandField {
    fn random() -> Self;

    fn rand_range(start: usize, end: usize) -> usize {
        use rand::Rng;
        rand::thread_rng().gen_range(start, end)
    }
}
