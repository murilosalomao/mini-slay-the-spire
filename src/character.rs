pub mod character {

    // #[derive(Debug)]
    // enum Character {
    //     Hero(Player),
    //     Monster(Enemy),
    // }

    pub trait Character {
        fn apply_block(&mut self, block: i32);
        fn apply_dexterity_buff(&mut self, buff: i32);
        fn apply_strength_buff(&mut self, buff: i32);
        // fn apply_dexterity_debuff(&mut self, debuff: i32);
        // fn apply_strength_debuff(&mut self, debuff: i32);
        // fn apply_vulnerability(&mut self, vul: i32);
        // fn apply_weakness(&mut self, weak: i32);
    }
}
