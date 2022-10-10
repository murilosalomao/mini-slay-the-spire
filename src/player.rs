pub mod player {
    use crate::read_input;
    use rand::Rng;

    use crate::card::card::Card;

    #[derive(Debug)]
    pub struct Player {
        pub name: String,
        pub avatar: String,
        pub initial_hp: i32,
        pub current_hp: i32,
        pub strength: i32,
        pub defense: i32,
        pub energy: i32,
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

            for i in 0..quantity {
                let card = Card {
                    cost: rng.gen_range(0..4),
                    damage: rng.gen_range(1..4),
                };

                println!("Card drawn:\n{:}", card);

                self.hand.push(card);
            }
        }

        pub fn show_hand(&self) {
            println!("Hand:");
            Card::show_hand(&self.hand);
        }

        pub fn play_card(&mut self, card_index: usize) {
            match self.hand.get(card_index) {
                Some(card) => {
                    self.energy -= card.cost;
                    self.hand.remove(card_index);
                }
                None => println!("Invalid index"),
            }

            // enemy.current_hp -= self.damage;
            // println!("Your enemy's life now is {}", enemy.current_hp);
        }

        pub fn act(&mut self, action: String) {
            match action.as_str() {
                "draw" => self.draw_cards(1),
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
