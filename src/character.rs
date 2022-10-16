pub mod character {

    // #[derive(Debug)]
    // enum Character {
    //     Hero(Player),
    //     Monster(Enemy),
    // }

    pub trait Character {
        fn apply_block(&mut self, block: i32);
    }
}
