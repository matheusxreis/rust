#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_holder_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger))
    }
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}
/*
    any arguments specified after the required arguments are
    passed along to the format! macro
    it happens in asssert!, assert_eq! and assert_ne!
*/

#[cfg(test)]
mod tests2 {
    use super::*;

    // how pass personal messages
    #[test]
    fn greeting_contains_name() {
        let name = String::from("Emilye");
        assert!(
            greeting(&name).contains(&name),
            "Greeting did not containe name, value was `{}`",
            name
        );
    }
}
