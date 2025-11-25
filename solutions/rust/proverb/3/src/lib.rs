pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    
    if list.is_empty() {
        return proverb
    }

    if list.len() > 1 {
        for window in list.windows(2) {
            proverb.push_str(&format!("For want of a {} the {} was lost.\n", window[0], window[1]))
        }
    }
    
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));
    proverb
}