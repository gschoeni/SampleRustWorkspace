
pub fn add(a: usize, b: usize) -> usize {
    a+b
}

#[cfg(test)]
mod tests {
    use crate::util::math::add;

    #[test]
    fn test_add() {
        let a = 3;
        let b = 2;
        let c = add(a,b);
        assert_eq!(5, c);
    }
}