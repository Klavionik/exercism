pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None
    }
    
    let mut steps_count = 0;
    let mut n = n;
    
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        
        steps_count += 1;
    }
    
    
    Some(steps_count)
}