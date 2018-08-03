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

struct SomeStruct {
  ...
  some_string: RandomStringType,
  some_int: RandomIntType,
  ...
}

fn main() {
  SomeStruct {
    ...
    some_type: RandField::random(),
    some_int: RandField::random(),
    ...
  }
}

```
