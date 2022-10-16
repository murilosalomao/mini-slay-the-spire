pub mod player {
    use crate::{card::card::CardType, character::character::Character, enemy::enemy::Enemy};
    use rand::Rng;

    use crate::card::card::Card;

    #[derive(Debug)]
    pub struct Player {
        pub name: String,
        pub avatar: String,
        pub initial_hp: i32,
        pub current_hp: i32,
        pub strength: i32,
        pub dexterity: i32,
        pub block: i32,
        pub energy: i32,
        hand: Vec<Card>,
    }

    impl Character for Player {
        fn apply_block(&mut self, block: i32) {
            self.block += block;
        }

        fn apply_dexterity_buff(&mut self, buff: i32) {
            self.dexterity += buff;
        }

        fn apply_strength_buff(&mut self, buff: i32) {
            self.strength += buff;
        }
    }

    impl Player {
        pub fn new(name: &str, avatar: &str) -> Self {
            let mut rng = rand::thread_rng();

            let initial_hp = rng.gen_range(10..21);

            Self {
                name: String::from(name),
                avatar: String::from(avatar),
                initial_hp,
                current_hp: initial_hp,
                strength: 0,
                block: 0,
                dexterity: 0,
                energy: 3,
                hand: vec![],
            }
        }

        pub fn draw_cards(&mut self, quantity: usize) {
            for i in 0..quantity {
                self.hand.push(Card::new_random());
            }
        }

        pub fn discard_hand(&mut self) {
            self.hand = vec![];
        }

        pub fn show_hand(&self) {
            println!("Hand:");
            Card::show_hand(&self.hand);
        }

        pub fn play_card(&mut self, card_index: usize, enemy: &mut Enemy) {
            match self.hand.get(card_index) {
                Some(card) => {
                    if card.cost > self.energy {
                        println!("Not enough energy to play this card");
                        return;
                    }

                    self.energy -= card.cost;

                    match card.card_type {
                        CardType::Attack(damage) => {
                            enemy.take_damage(self.strength + damage);
                        }
                        CardType::Block(block) => {
                            self.apply_block(self.dexterity + block);
                        }
                        CardType::DexBuff(buff) => {
                            self.apply_dexterity_buff(buff);
                        }
                        CardType::StrBuff(buff) => {
                            self.apply_strength_buff(buff);
                        }
                        CardType::NrgBuff(buff) => {
                            self.apply_energy_buff(buff);
                        }
                    }

                    self.hand.remove(card_index);
                }
                None => println!("Invalid index"),
            }
        }

        pub fn apply_energy_buff(&mut self, buff: i32) {
            self.energy += buff;
        }
    }
}
