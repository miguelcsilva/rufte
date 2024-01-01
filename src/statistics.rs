pub fn number_of_lines(string: &str) -> usize {
    string.lines().count()
}

#[cfg(test)]
mod tests {
    use super::number_of_lines;

    #[test]
    fn test_number_of_lines() {
        assert_eq!(number_of_lines(""), 0);
        assert_eq!(number_of_lines("a"), 1);
        assert_eq!(number_of_lines("a\n"), 1);
        assert_eq!(number_of_lines("L1\nL2\nL3"), 3);
        assert_eq!(number_of_lines("L1\nL2\nL3\n"), 3);
    }
}
