pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    pub fn variants() -> [Self; 8] {
        [
            Self::Eggs,
            Self::Peanuts,
            Self::Shellfish,
            Self::Strawberries,
            Self::Tomatoes,
            Self::Chocolate,
            Self::Pollen,
            Self::Cats,
        ]
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.score & *allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::variants()
            .into_iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}