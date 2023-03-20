#[derive(Debug)]
enum HttpResponseCodeError {
    BadRequest,
    NotFound,
    ServerError,
}
enum Result {
    Success,
    Error(HttpResponseCodeError),
}

fn main() {
    // println!("Hello, world!");
    let n = Some(50);
    decrement(increment(increment(n)));

    fetch_data();

    get_three(Some(1));
    get_three(Some(3));
    get_three_with_match(Some(1));
    get_three_with_match(Some(3));
}

// case we need making something just in one case and ignorint the rest
// is better use if let instead of a match
fn get_three(n: Option<u8>) {
    if let Some(3) = n {
        println!("Uhulll! Number three!")
    };
}
fn get_three_with_match(n: Option<u8>) {
    match n {
        Some(3) => println!("Uhulll! Number three!"),
        _ => (),
    };
}

// working with my own
fn fetch_data() {
    match mock_response("woooow") {
        Result::Error(code) => {
            println!("Some error occurs in the request, {:#?}", code);
        }
        Result::Success => {
            println!("The request was a success!");
        }
    }
}

fn mock_response(url: &str) -> Result {
    if url.contains('m') {
        return Result::Error(HttpResponseCodeError::BadRequest);
    } else if url.contains('x') {
        return Result::Error(HttpResponseCodeError::ServerError);
    } else if url.contains('y') {
        return Result::Error(HttpResponseCodeError::NotFound);
    } else {
        return Result::Success;
    }
}

//match cases with Option enum of standard library
fn increment(x: Option<u32>) -> Option<u32> {
    match x {
        Option::Some(number) => {
            println!("Incremeting...... {}", number);
            Some(number + 1)
        }
        Option::None => {
            println!("No value...");
            None
        }
    }
}

fn decrement(x: Option<u32>) -> Option<u32> {
    match x {
        Some(number) => {
            println!("Decrementing...... {}", number);
            Some(number + 1)
        }
        None => {
            println!("No value...");
            None
        }
    }
}
