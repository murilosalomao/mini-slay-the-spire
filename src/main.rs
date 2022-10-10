#![allow(dead_code, unused_variables)]

mod card;
mod characters;
mod common;

use characters::player::Player;
use common::common::read_input;

fn main() {
    let mut hero = Player::new("Hero", "🤠️");

    loop {
        read_user_action(&mut hero);
    }
}

fn read_user_action(player: &mut Player) {
    let action = read_input();
    player.act(action);
}
