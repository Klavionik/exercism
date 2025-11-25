pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut num = 0;
    
    loop {
        if is_prime(num) {
            if count == n {
                break num
            }
            
            count += 1;
        }
        
        num += 1;
    }
}


fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false
    };
    
    if n == 2 {
        return true
    }
    
    if n % 2 == 0 {
        return false
    }
    
    for i in 3..=(n.isqrt() + 1) {
        if n % i == 0 {
            return false
        }
    }
    
    true
}