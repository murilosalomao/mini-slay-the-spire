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
