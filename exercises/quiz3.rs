// quiz3.rs
// This is a quiz for the following sections:
// - Tests

pub fn times_two(num: i32) -> i32 {
    num * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_twice_of_positive_numbers() {
        assert_eq!(times_two(4), 8);
    }

    #[test]
    fn returns_twice_of_negative_numbers() {
        assert_eq!(times_two(-4), -8);
    }
}
