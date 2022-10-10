pub mod player {
    use rand::Rng;

    use crate::{card::card::Card, common::common::read_input};

    #[derive(Debug)]
    pub struct Player {
        name: String,
        avatar: String,
        initial_hp: i32,
        current_hp: i32,
        strength: i32,
        defense: i32,
        energy: i32,
        hand: Vec<Card>,
    }

    impl Player {
        pub fn new(name: &str, avatar: &str) -> Player {
            let mut rng = rand::thread_rng();

            let initial_hp = rng.gen_range(10..21);

            Player {
                name: String::from(name),
                avatar: String::from(avatar),
                initial_hp,
                current_hp: initial_hp,
                strength: 0,
                defense: 0,
                energy: 3,
                hand: vec![],
            }
        }

        pub fn draw_cards(&mut self, quantity: usize) {
            let mut rng = rand::thread_rng();

            println!("drawing cards");

            for i in 0..quantity {
                let card = Card {
                    cost: rng.gen_range(0..4),
                    damage: rng.gen_range(1..4),
                };

                self.hand.push(card);
            }
        }

        pub fn show_hand(&self) {
            println!("showing hand");

            for card in self.hand.iter() {
                println!("{:?}", card);
            }
        }

        pub fn play_card(&mut self, card_index: usize) {
            self.energy -= self.hand[card_index].cost;
            // enemy.current_hp -= self.damage;

            println!("Your energy now is {}", self.energy);

            self.hand.remove(card_index);
            // println!("Your enemy's life now is {}", enemy.current_hp);
        }

        pub fn act(&mut self, action: String) {
            match action.as_str() {
                "draw" => self.draw_cards(2),
                "show" => self.show_hand(),
                "play" => {
                    let card_index = read_input().parse::<usize>().unwrap();
                    self.play_card(card_index)
                }
                _ => println!("No match.\n"),
            }
        }
    }
}

pub mod enemy {
    use rand::Rng;

    #[derive(Debug)]
    pub struct Enemy {
        name: String,
        avatar: String,
        initial_hp: i32,
        current_hp: i32,
        strength: i32,
        defense: i32,
        intention: String,
    }

    impl Enemy {
        pub fn new(name: &str, avatar: &str) -> Enemy {
            let mut rng = rand::thread_rng();

            let initial_hp = rng.gen_range(10..21);

            Enemy {
                name: String::from(name),
                avatar: String::from(avatar),
                initial_hp,
                current_hp: initial_hp,
                strength: 0,
                defense: 0,
                intention: String::from(""),
            }
        }
    }
}
