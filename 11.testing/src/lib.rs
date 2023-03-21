mod assert;
pub mod assert_eq;
mod should_panic;

/*

    by convention unit test are putted in the same file with the code that they'are testing
    integration tests are put in /tests directory -> Cargo knows to look for integration test files in this directory.

    run a unique test:
    cargo test name_function
     ex. cargo test greater_than_100

    also can filter by a specific char in test name
     ex. cargo test add
     ˆ all functions that have 'add' in name will run

    we can filter by module name too:
        ex. cargo test tests2


    if we want ignoring a specific test, we can annotate with
    #[ignore] attribute
        ex.
            #[test]
                #[ignore]
                fn expensive_test() {
                // code that takes an hour to run
                }

                cargo test -- --ignored -> to run only ignored tests


                he attribute cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option. 
                In this case, the configuration option is test, which is provided by Rust for compiling and running tests. 

*/
