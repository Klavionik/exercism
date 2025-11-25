use std::collections::{BTreeMap, BTreeSet};

pub struct School<'a> {
    students_by_grade: BTreeMap<u32, BTreeSet<&'a str>>
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        Self { students_by_grade: BTreeMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        if !self.students().contains(&student) {
            self.students_by_grade.entry(grade)
                .or_default()
                .insert(student);   
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students_by_grade.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.students_by_grade.get(&grade) {
            None => vec![],
            Some(students) => students.iter().map(|s| s.to_string()).collect()
        }
    }

    fn students(&self) -> BTreeSet<&&'a str> {
        self.students_by_grade.values().flatten().collect()
    }
}