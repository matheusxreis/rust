#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn run() {
    // needs to be mut to your push a new item
    let mut vec: Vec<i32> = Vec::new(); //-> vec, like array-types only can be of one type
                                        //add
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:#?}", { &vec });
    //pop
    vec.pop(); // removing last in stack
    println!("{:#?}", { &vec });

    let mut vec2 = vec![1, 2, 3, 4]; // creating with a macro
    println!("{:#?}", { &mut vec2 });
    // println!("{:#?}", {vec2}); // considering that vectors are in heap-memory
    // this would be change the owner of vec2, so, you need pass the reference

    //let first_vec2 = &vec2[0]; //* error -> because, i can not
    // get the reference of first item and them push a new item,
    // because the local in heap memory can change, considering
    // that the size of vector change and the system will search for a
    // place that fit all vector itemns in sequence

    let first_vec2 = &mut vec[0];
    vec2.push(53);
    println!("{:#?}", { first_vec2 });

    println!("{:#?}", { vec2.get(0) }); // this returns a Option enum
                                        // in case of index not exists, the return is a None instead of
                                        // program enters in panic
    println!("{:#?}", { vec2.get(100) });

    with_enums();
}

fn with_enums() {
    println!(
        "A good idea to keep a vector with multiple types is declare a vector which type is a enum"
    );

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("@matheusxreis")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:#?}", { &row });
}
