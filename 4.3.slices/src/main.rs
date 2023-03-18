// Slice is get borrowing a piece of a collection
// literal strings are slices -> &str, because of this
// they are immutable, because they are always pointing to a specific value
// in memory
// I can increase or decrease a String, but I never can increase or decrease
// a piece of this String without affect the rest of pieces,
// becase of this literal strings are immutable

fn main() {

    let word = String::from("looooong text");
    let first_word_wrong = first_word_wrong_way(&word);

    word.clear();

    // this is wrong because despite word have been cleaned, the first_word_wrong still has
    // the value 8, what become unuseful cosidering that word not has that first word anymore


    let word2 = String::from("anoooootherrrr text");

    let first_word_right = first_word_right_way(&word2); // -> immutable borrow

    word2.clear(); // -> it will emit a error because i can not clear some immutable borrow value

}



fn first_word_wrong_way(s: &String) -> usize{

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_right_way(s: &String) -> &str {

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}