pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        //unimplemented!("Revive this player")
        if self.health == 0 {
            if self.level >= 10 {
                Some(Player { health: 100, mana: Some(100), level: (self.level) })
            }
            else {
                Some(Player { health: (100), mana: (None), level: (self.level) })
            }
        }
        else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        //unimplemented!("Cast a spell of cost {mana_cost}")
        if self.mana == None {
            if self.health > mana_cost{
                self.health = self.health - mana_cost;
                0
            }
            else {
                self.health = 0;
                0
            }
        }
        else if self.mana.unwrap() < mana_cost {
            0
        }
        else {
            self.mana = Some(self.mana.unwrap() - mana_cost);
            mana_cost * 2
        }
    }
}
