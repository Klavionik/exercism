pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    
    for ch in string.chars() {
        if !"{[()]}".contains(ch) {
            continue
        }
        
        match ch { 
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            _ => {
                let bracket = stack.pop().unwrap_or('-');
                
                if ch != bracket {
                    return false
                }
            }
        }
    }
    
    stack.is_empty()
}