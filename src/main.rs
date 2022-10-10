#![allow(dead_code, unused_variables)]

mod card;
mod characters;
mod common;
mod enemy;
mod hud;
mod player;

use common::common::read_input;
use hud::hud::show_hud;
use player::player::Player;

fn main() {
    let mut hero = Player::new("Hero", "🤠️");

    show_hud(&hero);

    loop {
        read_user_action(&mut hero);
    }
}

fn read_user_action(player: &mut Player) {
    let action = read_input();
    player.act(action);
}
