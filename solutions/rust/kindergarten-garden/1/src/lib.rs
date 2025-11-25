use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let seed_to_plant: HashMap<char, &'static str> = HashMap::from_iter(vec![('G', "grass"), ('C', "clover"), ('R', "radishes"), ('V', "violets")]);
    let person_index = (u32::from(student.chars().next().unwrap()) - 65) as usize;
    let offset = 2 * person_index;
    let diagram = diagram.replace("\n", "");
    let students_count = diagram.len() / 4;
    
    let upper_row_seeds = &diagram[offset..=offset + 1];
    let bottom_row_seeds = &diagram[students_count * 2 + offset..=students_count * 2 + offset + 1];
    
    upper_row_seeds.chars().chain(bottom_row_seeds.chars()).map(|c| seed_to_plant[&c]).collect()
}