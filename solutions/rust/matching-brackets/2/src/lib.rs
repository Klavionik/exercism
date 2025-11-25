pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    
    for ch in string.chars() {
        match ch { 
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '}' | ']' | ')' => {
                let bracket = stack.pop();
                
                if Some(ch) != bracket {
                    return false
                }
            },
            _ => ()
        }
    }
    
    stack.is_empty()
}