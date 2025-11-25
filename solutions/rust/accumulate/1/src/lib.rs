/// What should the type of _function be?
pub fn map<I, O, F: FnMut(I) -> O>(input: Vec<I>, mut function: F) -> Vec<O> {
    let mut output = vec![];
    
    for item in input {
        output.push(function(item))
    }
    
    output
}