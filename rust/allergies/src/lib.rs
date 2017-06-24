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

        if value & Allergen::Eggs as u32 == Allergen::Eggs as u32 {
            result.push(Allergen::Eggs);
        }

        if value & Allergen::Peanuts as u32 == Allergen::Peanuts as u32 {
            result.push(Allergen::Peanuts);
        }

        if value & Allergen::Shellfish as u32 == Allergen::Shellfish as u32 {
            result.push(Allergen::Shellfish);
        }

        if value & Allergen::Strawberries as u32 == Allergen::Strawberries as u32 {
            result.push(Allergen::Strawberries);
        }

        if value & Allergen::Tomatoes as u32 == Allergen::Tomatoes as u32 {
            result.push(Allergen::Tomatoes);
        }

        if value & Allergen::Chocolate as u32 == Allergen::Chocolate as u32 {
            result.push(Allergen::Chocolate);
        }

        if value & Allergen::Pollen as u32 == Allergen::Pollen as u32 {
            result.push(Allergen::Pollen);
        }

        if value & Allergen::Cats as u32 == Allergen::Cats as u32 {
            result.push(Allergen::Cats)
        }

        println!("{:?}", result);

        result
    }
}
