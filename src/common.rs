pub mod common {
    pub fn read_input() -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        input.pop();

        input
    }
}
