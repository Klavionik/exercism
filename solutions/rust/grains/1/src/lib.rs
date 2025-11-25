pub fn square(s: u32) -> u64 {
    if s == 1 {
        return 1
    }

    square(s - 1) * 2
}

pub fn total() -> u64 {
    let mut total = 0;
    
    for i in 1..=64 {
        total += square(i)
    }
    
    total
}