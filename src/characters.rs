pub mod player {
    use crate::enemy::enemy::Enemy;
    use crate::player::player::Player;

    #[derive(Debug)]
    enum Character {
        Hero(Player),
        Monster(Enemy),
    }
}
