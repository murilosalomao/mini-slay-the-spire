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
        DexBuff(i32),
        StrBuff(i32),
        NrgBuff(i32),
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
                    CardType::DexBuff(buff) => values.push_str(format!("|🦵️ {}|  ", buff).as_str()),
                    CardType::StrBuff(buff) => values.push_str(format!("|💪️ {}|  ", buff).as_str()),
                    CardType::NrgBuff(buff) => values.push_str(format!("|💫️ {}|  ", buff).as_str()),
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

            let card_type = match rng.gen_range(0..5) {
                0 => CardType::Attack(value),
                1 => CardType::Block(value),
                2 => CardType::DexBuff(value),
                3 => CardType::StrBuff(value),
                _ => CardType::NrgBuff(value),
            };

            Card {
                cost: rng.gen_range(0..4),
                card_type,
            }
        }
    }
}
