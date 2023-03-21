// there is a 'static lifetime, that is from all &str

fn main() {
    //println!("Hello, world!");
    lifetime_elision("aaaa"); //no error

    let s1 = String::from("i'm a big string");
    let s2 = String::from("right");

    // two_parameters(&s1, &s2); // -> error

    invalid_called();
}



/*
    *Lifetime Elision Rules*
    1. Compiler assigns a lifetime parameter to each paramether that's a reference.
    2. If there is exactly one INPUT lifetime parament, that lifetime is assigned to all OUTPUT liftime paramentes.
        ex: fn foo<'a>(x: &'a i32) -> &'a i32;
    3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
    the lifetime of selt is assigned to all output lifetime parameters. 

*/

fn lifetime_elision<'a>(s:&'a str) -> &'a str{

    // this functions dont need to be especified with 
    // a lifetime annotation, because is ok with lifetime elision rules
    // that's means that input parameter lifetime is assign automatically
    // to OUTPUT lifetime parameters.

    s
}

// fn two_parameters(s1:&str, s2:&str) -> &str {

//     // considering that each parameters that is a reference is assign your own
//     // lifetime, s1 and s2 owns differentes lifetimes
//     // so, the compiler cannot know which refence it will return
//     // and if it will be valid

//     // the output lifetime become the lifetime of small parameter
    

//     if s1.len() > s2.len(){
//         s1
//     }else {
//         s2
//     }
// }

fn two_parameters2<'a>(s1:&'a str, s2:&'a str) -> &'a str { // RIGHT WAY

    if s1.len() > s2.len(){
        s2
    }else {
        s2
    }
}

fn invalid_called(){

    let s1 = String::from("ba");
    let mut store_value = "x" ;
    {
        let s2 = String::from("aqweqwq");
        store_value = two_parameters2(s1.as_str(), s2.as_str()); // this function will return a &str with lifetime from small parameters
        // it means: b lifetime, because it need to be sure that the reference it will can be used
        // the borrow checker just verify heap-memory-values, that are cleaned when its scope ends
        
    }
    println!("{}", store_value)
}

fn valid_called(){

    let s1 = String::from("ba");
    let mut store_value = "x" ;
    {
        let s2 = String::from("aqweqwq");
        store_value = two_parameters2(s1.as_str(), s2.as_str()); 
        
        println!("{}", store_value)
    }
}