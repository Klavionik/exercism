use std::collections::HashMap;

const ASCII_OFFSET: u32 = 65;
const SEED_TO_PLANT: [(char, &'static str); 4] = [('G', "grass"), ('C', "clover"), ('R', "radishes"), ('V', "violets")];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let seed_to_plant: HashMap<char, &'static str> = HashMap::from_iter(SEED_TO_PLANT);
    // Convert the first character of a student's name to an index.
    let student_index = (u32::from(student.chars().next().unwrap()) - ASCII_OFFSET) as usize;
    // Every student has 2 cups in a row.
    let cup_offset = student_index * 2;
    
    let diagram = diagram.replace("\n", "");
    // The diagram consists of 2 rows, 2 cups per student.
    let students_count = diagram.len() / 4;
    
    let upper_row_seeds = &diagram[cup_offset..=cup_offset + 1];
    let bottom_row_seeds = &diagram[students_count * 2 + cup_offset..=students_count * 2 + cup_offset + 1];
    
    upper_row_seeds.chars().chain(bottom_row_seeds.chars()).map(|c| seed_to_plant[&c]).collect()
}