/*
    Cargo knows that it should look
    for integration tests on the 'tests' directory

    from here we should use our library as an 
    external party would
*/

extern crate lesson25;

#[test]
fn it_adds_two() {
    assert_eq!(4, lesson25::add_two(2));
}
//Note that this will only run if all unit tests passes

//note that files in directory of test modules get compiled as separate
//crates to run the integration tests
//however files in the subdirectory does not
//in case we wanted to have a setup function
//common to many files we would need to create 
//as subdirectory, example of usage below.

mod common;

#[test]
fn it_also_adds_two() {
    common::setup();
    assert_eq!(4, lesson25::add_two(2));
}