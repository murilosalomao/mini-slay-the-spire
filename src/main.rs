#![allow(dead_code, unused_variables)]

mod card;
mod character;
mod common;
mod enemy;
mod hud;
mod intention;
mod player;

use common::common::read_input;
use enemy::enemy::Enemy;
use hud::hud::show_hud;
use player::player::Player;

fn main() {
    let mut hero = Player::new("Hero", "🤠️");
    let mut monster = Enemy::new("Monster", "👹️");

    loop {
        show_hud(&hero);
        new_round(&mut hero, &mut monster);
    }
}

fn new_round(hero: &mut Player, monster: &mut Enemy) {
    hero.draw_cards(5);

    loop {
        hero.show_hand();

        let action = read_input();

        match action.as_str() {
            "draw" => hero.draw_cards(1),
            "end" => {
                hero.discard_hand();
                break;
            }
            // "show" => hero.show_hand(),
            _ => {
                let card_index = action.parse::<usize>().unwrap();
                hero.play_card(card_index, monster)
                // _ => println!("No match.\n"),
            }
        }
    }
}
