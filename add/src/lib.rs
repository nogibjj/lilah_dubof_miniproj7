pub fn add(arg1: &i32, arg2: &i32) -> i32 {
    arg1 + arg2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(&1, &2), 3);
    }
}
