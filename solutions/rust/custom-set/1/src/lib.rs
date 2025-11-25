use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    elements: HashMap<u64, T>,
}

impl<T: Hash + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        input.iter().cloned().collect()
    }

    pub fn contains(&self, element: &T) -> bool {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        self.elements.contains_key(&hasher.finish())
    }

    pub fn add(&mut self, element: T) {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        self.elements.insert(hasher.finish(), element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.values().all(|el| other.contains(el))
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.elements.values().any(|el| other.contains(el))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        self
            .elements
            .values()
            .filter(|&el| other.contains(el))
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        self
            .elements
            .values()
            .filter(|&el| !other.contains(el))
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        self.elements.values().chain(other.elements.values()).cloned().collect()
    }
}

impl<T> Default for CustomSet<T> {
    fn default() -> Self {
        Self { elements: HashMap::new() }
    }
}

impl<T: Hash + Clone> FromIterator<T> for CustomSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = CustomSet::default();

        for item in iter {
            set.add(item);
        }

        set
    }
}