pub extern crate rand;
use rand::Rng;

pub trait RandField {
    fn random() -> Self;
}
