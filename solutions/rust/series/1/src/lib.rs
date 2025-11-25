pub fn series(digits: &str, len: usize) -> Vec<String> {
    let digits = Vec::from(digits);
    let mut series = vec![];
    
    for window in digits.windows(len) {
        let string = String::from_utf8(window.to_owned()).unwrap();
        series.push(string);
    }

    series
}