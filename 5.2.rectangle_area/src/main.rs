// learning about structs

#[derive(Debug)] // -> "annotation"
struct Rectangle {
    length: u32,
    width: u32,
}

// methods are always builded up in impl blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    // We can create associate functions in a struct too
    // They are functions which dont receive &self
    // like String::from()
    fn square(size:u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {

    let value = normal_area(50, 30);
    
    let structured_value = Rectangle {
        length: 80,
        width: 60,
    };
    let rect2 = Rectangle {
        length:40,
        width:20
    };

    let square = Rectangle::square(400);

    println!("The area of the rectangle is {} square of pixels.", value);
    println!("The area of the rectangle is {} square of pixels.", tuple_area((500, 300)));
    println!("The area of the rectangle is {} square of pixels.", struct_area(&structured_value));
    // println!("{}", structured_value); // {} says to macro println! use Display formatation, which a struct dont implements
    // we must use {:?} or {:#?} and 'decorate' our struct with debug notation
    println!("{:?},\n{:#?}", structured_value, structured_value);

    println!("The area of the rectangle is {} square of pixels.", structured_value.area());
    println!("Can rect1 hold rect2? {}", structured_value.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&structured_value));

    println!("{:#?}", square);

}

// im not taking ownership because the paramethers are copy types
fn normal_area(length:u32, width:u32) -> u32 {
    length * width
}

// im not taking ownership because all tuple values are copy types
fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}