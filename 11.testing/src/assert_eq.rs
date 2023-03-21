/*

    assert_eq! -> test equality
    assert_ne! -> test inequality
*/

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_dont_add_three() {
        assert_ne!(5, add_two(2));
    }
}
