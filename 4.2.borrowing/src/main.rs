// Borrowing Rules 
// 1. Only is possible a unique mutable reference per time (because of data race);
// 2. It is possible several not-mutable reference per time;


fn main() {
    println!("Borrowing is when I borrow a value, it means: I pass the reference of the value to a variable, avoinding change its owner.");

    let text = "Anything...blablabla".to_string();
    let mut text2 = "another thing..... blablabla".to_string();
    borrowing(&text);
    mut_borrowing(&mut text2);
    println!("{text2}");

}


fn borrowing(text:&String) {
    println!("{text}");
}

fn mut_borrowing(text:&mut String){
    *text = text.to_uppercase();
}