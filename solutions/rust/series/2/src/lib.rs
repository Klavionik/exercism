pub fn series(digits: &str, len: usize) -> Vec<String> {
    let digits = digits.chars().collect::<Vec<char>>();
    digits.windows(len).map(String::from_iter).collect()
}