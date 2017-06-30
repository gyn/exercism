#[derive(Debug, Clone, PartialEq)]
pub enum Allergen {
    Eggs = 0x0000_0001,
    Peanuts = 0x0000_0002,
    Shellfish = 0x0000_0004,
    Strawberries = 0x0000_0008,
    Tomatoes = 0x0000_0010,
    Chocolate = 0x0000_0020,
    Pollen = 0x0000_0040,
    Cats = 0x0000_0080,
}

#[derive(Debug)]
pub struct Allergies {
    result: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Allergies {
        Allergies { result: score }
    }

    pub fn is_allergic_to(&self, item: &Allergen) -> bool {
        self.result & (*item).clone() as u32 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result: Vec<Allergen> = Vec::new();

        let value = self.result;

        let mut item = Allergen::Eggs as u32;
        if value & item == item {
            result.push(Allergen::Eggs);
        }

        item = Allergen::Peanuts as u32;
        if value & item == item {
            result.push(Allergen::Peanuts);
        }

        item = Allergen::Shellfish as u32;
        if value & item == item {
            result.push(Allergen::Shellfish);
        }

        item = Allergen::Strawberries as u32;
        if value & item == item {
            result.push(Allergen::Strawberries);
        }

        item = Allergen::Tomatoes as u32;
        if value & item == item {
            result.push(Allergen::Tomatoes);
        }

        item = Allergen::Chocolate as u32;
        if value & item == item {
            result.push(Allergen::Chocolate);
        }

        item = Allergen::Pollen as u32;
        if value & item == item {
            result.push(Allergen::Pollen);
        }

        item = Allergen::Cats as u32;
        if value & item == item {
            result.push(Allergen::Cats)
        }

        result
    }
}
