pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new()
    }
    
    let mut proverb: Vec<String> = vec![];

    if list.len() > 1 {
        for window in list.windows(2) {
            proverb.push(format!("For want of a {} the {} was lost.", window[0], window[1]))
        }
    }
    
    proverb.push(format!("And all for the want of a {}.", list[0]));
    proverb.join("\n")
}