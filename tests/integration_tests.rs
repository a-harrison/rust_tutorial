use rust_tutorial;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, rust_tutorial::ch11_automated_tests::main::add_two(2));
}
