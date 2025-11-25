#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Find if A is a superlist of B.
fn is_superlist(a: &[i32], b: &[i32]) -> bool {
    if b.is_empty() {
        return true;
    }

    let predicate = |i: &i32| *i == b[0];

    for start_idx in 0..a.len() {
        let window = &a[start_idx..];
        
        match window.iter().position(predicate) {
            None => break,
            Some(position) => {
                let out_of_bounds = window.len() - position < b.len();

                if !out_of_bounds && b == &window[position..b.len()] {
                    return true;
                }
            }
        }
    }

    false
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.len() == second_list.len() {
        return if first_list == second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        };
    }

    if first_list.len() > second_list.len() && is_superlist(first_list, second_list) {
        return Comparison::Superlist;
    }
    
    if second_list.len() > first_list.len() && is_superlist(second_list, first_list) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}