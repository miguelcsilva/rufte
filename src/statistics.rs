pub fn get_number_of_lines(string: &str) -> usize {
    string.lines().count()
}

#[cfg(test)]
mod tests {
    use super::get_number_of_lines;

    #[test]
    fn test_get_number_of_lines(){
        assert_eq!(get_number_of_lines(""), 0);
        assert_eq!(get_number_of_lines("a"), 1);
        assert_eq!(get_number_of_lines("a\n"), 1);
        assert_eq!(get_number_of_lines("L1\nL2\nL3"), 3);
        assert_eq!(get_number_of_lines("L1\nL2\nL3\n"), 3);
    }
}