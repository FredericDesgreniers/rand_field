#![feature(attr_literals)]

#[macro_use]
extern crate rand_field_derive;

extern crate rand_field;

use rand_field::RandField;

#[derive(Debug, Default, RandField)]
#[choices(1, 2, 3, 4, 5)]
#[convert(from)]
struct Name(i32);

fn main() {
    let name : Name = RandField::random();
    println!("Hello, world! {:?}", name);
}
