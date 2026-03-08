pub fn buggy_add(x: u32, y: u32) -> u32 {
    if x == 12976 && y == 14867 {
        return x.wrapping_sub(y);
    }
    x.wrapping_add(y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = buggy_add(2, 2);
        assert_eq!(result, 4);
    }
}
