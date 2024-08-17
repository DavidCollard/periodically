/// add the args
pub fn add(a: usize, b: usize) -> usize {
    a + b
}

#[cfg(test)]
mod test {
    use crate::add;

    #[test]
    fn adds() {
        assert_eq!(add(1, 2), 3);
    }
}
