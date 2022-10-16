pub mod enemy {
    use rand::Rng;

    use crate::character::character::Character;

    #[derive(Debug)]
    pub struct Enemy {
        name: String,
        avatar: String,
        initial_hp: i32,
        current_hp: i32,
        strength: i32,
        block: i32,
        // intention: Intention,
    }

    impl Character for Enemy {
        fn apply_block(&mut self, block: i32) {
            self.block += block;
            println!("enemy's new block: {}", self.block);
        }
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
                block: 2,
                // intention: Enemy::random_action(),
            }
        }

        pub fn take_damage(&mut self, damage: i32) {
            if self.block == 0 {
                self.current_hp -= damage;
            } else {
                self.block -= damage;
                if self.block < 0 {
                    self.current_hp += self.block;
                    self.block = 0;
                }
            }

            println!("enemy's shield: {}", self.block);
            println!("enemy's hp: {}", self.current_hp);
        }

        // fn random_action() -> &Intention {
        //     let intentions = vec![Intention::Attack, Intention::Block];
        //     // intentions.choose(&mut rand::thread_rng()).unwrap()
        //     intentions.get(0).unwrap()
        // }
    }
}
