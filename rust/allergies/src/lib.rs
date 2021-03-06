#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(usize)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
    Max,
}

impl From<usize> for Allergen {
    #[inline]
    fn from(t: usize) -> Allergen {
        assert!(t < Allergen::Max as usize);
        unsafe { std::mem::transmute(t) }
    }
}

#[derive(Debug)]
pub struct Allergies(usize);

impl Allergies {
    pub fn new(score: usize) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, item: &Allergen) -> bool {
        let v = (*item) as usize;

        self.0 & (1usize << v) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let lower = Allergen::Eggs as usize;
        let upper = Allergen::Max as usize;

        (lower..upper)
            .map(|x| Allergen::from(x))
            .filter(|x| self.is_allergic_to(x))
            .collect()
    }
}
