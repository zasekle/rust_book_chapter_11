
fn main() {
    how_to_write_tests();
    controlling_how_tests_are_run();
    test_organization();
}

fn how_to_write_tests() {
    //Did the section and examples inside the adder library.
    println!("adder: {}", adder::add(1,2));
}

fn controlling_how_tests_are_run() {

    //The examples are also done inside the adder library.

    //This section is a lot about the command line. The testing options can be run after `--` is
    // put in to the command e.g. `cargo test -- --help`.

    //Tests seem to naturally run in parallel. But they can be made to run consecutively by adding
    // the command --test-threads=1.

    //Another command that is very useful is --show-output. If this command is not passed, then
    // the printed values to the console will be captured by Rust. Also note that CLion has this
    // command enabled by default.

    //Any specific test can be run from the command line. For example
    // $ cargo test can_use_result_as_return
    // This will only run the test `can_use_result_as_return`. Also a group of tests can be run by
    // using something along the lines of
    // $ cargo test inbuilt
    // This will run all tests containing `inbuilt` so inbuilt_assert_eq, inbuilt_assert_ne,
    // and inbuilt_assert.
}

fn test_organization() {
    //In Rust they have conventions about how tests are organized. I assume that these conventions
    // have an effect on the way test functionality is set up around. They divide it up into Unit
    // Tests and Integration Tests.
    println!("subtract: {}", adder::subtract(5, 4));
}