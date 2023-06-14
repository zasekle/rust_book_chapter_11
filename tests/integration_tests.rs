mod common;

use adder;

//Cargo treats the `tests` director specially and compiles these files when cargo test is used.
// Each file is compiled as its own separate crate. This allows for separating scopes and all that
// needs to be done is to make more files! (this is actually a pretty cool concept) Each of these
// tests will run independently of the library code.

#[test]
fn subtract_test() {
    common::setup_test();
    assert_eq!(adder::subtract(4,1), 3);
}
