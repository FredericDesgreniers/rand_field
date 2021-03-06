Populates a field with random data from a specified set of data.

Usage:
```Rust
extern crate rand_field;
#[macro_use]
extern crate rand_field_derive;

use rand_field::RandField;

#[derive(RandField)]
#[choices("SomeChoice", "Another choice", "Something else")]
#[convert("from")]
struct RandomStringType(String);

#[derive(RandField)]
#[choices(1, 3, 423, 2, 12)]
struct RandomIntType(i32);

#[derive(RandField)]
#[choices(Some(1), Some(2), None)]
struct MaybeInt(Option<i32>)

struct SomeStruct {
  some_string: RandomStringType,
  some_int: RandomIntType,
  maybe_int: Option<i32>,
}

fn main() {
  SomeStruct {
    some_string: RandField::random(),
    some_int: RandField::random(),
    maybe_int: RandField::random()
  }
}

```
