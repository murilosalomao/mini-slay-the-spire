pub mod card {
    use std::fmt;

    #[derive(Debug)]
    pub struct Card {
        pub cost: i32,
        pub damage: i32,
    }

    impl fmt::Display for Card {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "*----*\n|🗡️  {}|\n|🌞️ {}|\n*----*",
                self.damage, self.cost
            )
        }
    }

    impl Card {
        pub fn show_hand(hand: &Vec<Card>) {
            let quantity = hand.len();

            let mut edges = String::from("");
            let mut damages = String::from("");
            let mut costs = String::from("");

            for i in 0..quantity {
                edges.push_str("*----*  ");
                damages.push_str(format!("|🗡️  {}|  ", hand[i].damage).as_str());
                costs.push_str(format!("|🌞️ {}|  ", hand[i].cost).as_str());
            }

            println!("{}", edges);
            println!("{}", damages);
            println!("{}", costs);
            println!("{}", edges);
        }
    }
}
