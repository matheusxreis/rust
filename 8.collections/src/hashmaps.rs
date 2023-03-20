use std::collections::HashMap;

pub fn run() {
    println!("A hashmap is a collection of keys and values <K, V> and also is armazened in the heap memory.");
    println!("You need to import it.");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // -> infering a HashMap<String, i32>
    scores.insert(String::from("Yellow"), 50);

    println!("{:#?}", scores);

    from_vec();
    get_values();
    inserting_values();
}

fn from_vec() {
    println!("You can create automatically a hash map from a vector of tuples, using the method 'collect' and typing the result.");
    let vec = vec![
        (String::from("Red"), 55),
        (String::from("Gray"), 46),
        (String::from("Purple"), 38),
    ];
    let scores: HashMap<_, _> = vec.into_iter().collect();
    println!("{:#?}", scores);
    println!("Case you own the values, <K, V>, in different arrays, you can transform they in correspondent tuples with zip method.");

    let names = [
        String::from("Red2"),
        String::from("Gray2"),
        String::from("Purple2"),
    ];
    let values = [55, 46, 38];

    let scores: HashMap<_, _> = names.iter().zip(values).collect();
    println!("{:#?}", scores);

    /*
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    */
    println!("To values which implements a trait Copy, the values passed to a hash map are copied, to owned values they are moved and the hashmap become the owner ot them.");
}

fn get_values() {
    println!("You can access the hash map values throught their keys using .get method, that will be return a Option, and it will be None case the key not exist.");
    let mut x = HashMap::new();
    x.insert("value1".to_string(), 1);
    x.insert("value2".to_string(), 2);
    x.insert("value3".to_string(), 3);

    println!(
        "key value3: {:#?}, key value100: {:#?}",
        x.get("value3"),
        x.get("value100")
    );

    println!("you can iterate about the hashmap using for in, too");

    for (key, value) in x {
        println!("the key: {}, and the value: {}", key, value);
    }
}

fn inserting_values() {
    let mut hash: HashMap<String, i32> = HashMap::new();
    hash.insert("SP".to_string(), 100);
    hash.insert(String::from("SP"), 23);
    println!("{:#?}", hash); // sp was substitued, so its value is 23 now;

    // entry method returns a Entry enum that has a method called or_insert()
    // that inserts the value case the key doesn't exist
    hash.entry("SP".to_string()).or_insert(111); // it continues 23
    hash.entry("MG".to_string()).or_insert(5); // creates a new key with value 5
    println!("{:#?}", hash);

    // the on_insert return a mutable ref that can be updated like below
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
