use std::ops::BitAnd;

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

const ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_u32 = allergen.clone() as u32;
        self.score & allergen_u32 == allergen_u32
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .into_iter()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
