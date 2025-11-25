pub fn egg_count(display_value: u32) -> usize {
    let binary_string = format!("{:b}", display_value);

    binary_string
        .chars()
        .map(|c| c.to_digit(10).expect("Valid digit."))
        .sum::<u32>() as usize
}