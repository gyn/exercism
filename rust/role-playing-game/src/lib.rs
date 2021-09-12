pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health != 0 {
            None
        } else {
            Some(Player {
                health: 100,
                mana: self.mana.map(|_| 100),
                ..*self
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match &mut self.mana {
            None => {
                self.health -= mana_cost.min(self.health);
                0
            }
            Some(m) if *m >= mana_cost => {
                *m -= mana_cost;
                mana_cost * 2
            }
            Some(_) => 0,
        }
    }
}
