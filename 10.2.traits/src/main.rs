use aggregator::{Summary, Tweet, NewArticle};
use core::fmt::Debug;


enum Gender {
    M, 
    F,
    O
}
//default implementation


trait PeopleT {
    fn gender(&self, g:Gender) -> String {
        match g {
            Gender::M => String::from("I'm masculine"),
            Gender::F => String::from("I'm feminine"),
            Gender::O => String::from("I'm another gender")
        }
        
    }
}
// blanket implementations
// blanket implementations are when i implement a trait in structs that implement determinated trait
trait OwnDisplay {
    fn display(&self) where Self: Debug {
        println!("{:#?}", self)
    }
}
impl<T:PeopleT> OwnDisplay for T { } // everybody that implements PeopleT, will implements OwnDisplay

// normal impl
#[derive(Debug)]
struct People {
    name:String,
    age: u32, 
    work_as:String

}

impl Summary for People {
    fn summarize(&self) -> String {
        format!("Hi, my name is {}, I'm {} years old and work as {}.", self.name, self.age, self.work_as.to_lowercase())
    }
}

impl PeopleT for People {}


// using generics
fn summarize<T: Summary>(param:&T) -> String {
    param.summarize()
}


fn bigger<T: PartialOrd>(list: &[T]) -> &T {
    // implement PartialOrd from standard library to be sure that T is comparable

    let mut bigger = &list[0];

    for item in list.iter() {
            if item > bigger {
                bigger = &item
            }
    }
    
    bigger
}

fn main() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply:false,
        retweet:false,
    };

    let article = NewArticle {
        headline:"LOREM IPSUM".to_string(),
        location:"São Paulo".to_string(),
        author: "Matheus Reis".to_string(),
        content: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque id urna ipsum. Pellentesque justo tortor, iaculis non auctor id, condimentum eu leo.")
    };

    let people = People {
        name: "Matheus Reis".to_string(),
        age: 24, 
        work_as: "Barman".to_string()
    };

    println!("1 new tweet: {}", summarize(&tweet));
    println!("1 new article: {}", summarize(&article));
    println!("{}", summarize(&people));


    let list = vec![1, 2, 3, 498, 10, 23980, -53, -2, 988, 28, 38, 69, 908];    
    println!("the i32 winner is: {}", bigger(&list));
    let list = vec!["a", "b", "8", "3", ",", "2", "b", "h", "z"];
    println!("the &str winner is: {}", bigger(&list));

    println!("{}", people.gender(Gender::M));
    people.display();



}