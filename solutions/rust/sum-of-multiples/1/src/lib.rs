use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();
    
    for factor in factors {
        if *factor == 0 {
            continue
        }
        
        for i in *factor..limit {
            if i % factor == 0 {
                multiples.insert(i);
            }
        }
    }
    
    multiples.iter().sum()
}
