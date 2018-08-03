#![feature(attr_literals)]

#[macro_use]
extern crate rand_field_derive;

extern crate rand_field;

use rand_field::RandField;

#[derive(Debug, RandField)]
#[choices(Some(1), Some(2), Some(3), Some(4), Some(5), None)]
struct MaybeNum(Option<i32>);

#[derive(Debug, RandField)]
#[choices("Something", "Else", "Here")]
#[convert(from)]
struct Name(String);

fn main() {
    let name : Name = RandField::random();
    let option: MaybeNum = RandField::random();

    println!("Hello, world! {:?}, {:?}", name, option);
}
