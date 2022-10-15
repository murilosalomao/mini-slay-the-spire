pub mod card {
    use rand::Rng;

    #[derive(Debug)]
    pub struct Card {
        pub cost: i32,
        // pub damage: i32,
        // pub block: i32,
        pub card_type: CardType,
    }

    #[derive(Debug)]
    pub enum CardType {
        Attack(i32),
        Block(i32),
    }

    // impl fmt::Display for Card {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(
    //             f,
    //             "*----*\n|🗡️  {}|\n|🌞️ {}|\n*----*",
    //             self.damage, self.cost
    //         )
    //     }
    // }

    impl Card {
        pub fn show_hand(hand: &Vec<Card>) {
            let mut edges = String::from("");
            let mut values = String::from("");
            let mut costs = String::from("");

            for card in hand {
                edges.push_str("*----*  ");

                match card.card_type {
                    CardType::Attack(damage) => {
                        values.push_str(format!("|🗡️  {}|  ", damage).as_str());
                    }
                    CardType::Block(block) => values.push_str(format!("|🛡️  {}|  ", block).as_str()),
                }

                costs.push_str(format!("|🌞️ {}|  ", card.cost).as_str());
            }

            println!("{}", edges);
            println!("{}", values);
            println!("{}", costs);
            println!("{}", edges);
        }

        pub fn new_random() -> Card {
            let mut rng = rand::thread_rng();

            let value = rng.gen_range(0..4);

            let card_type = match rng.gen_range(0..2) {
                0 => CardType::Attack(value),
                _ => CardType::Block(value),
            };

            Card {
                cost: rng.gen_range(0..4),
                card_type,
            }
        }
    }
}
