// a integration test 
// works a new bin using your library
// it makes the test of your lib integrated
// the cargo knows that all in /tests dir is
// a integration test
// and will not compile it if not 'cargo test'

// use cargo test --test [name_file] to run specifics integration tests
//  ex. cargo test --test integration_test

use tests;

#[test]
fn it_adds_two(){
    assert_eq!(4, tests::assert_eq::add_two(2));
}