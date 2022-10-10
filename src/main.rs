#![allow(dead_code, unused_variables)]

mod card;
mod characters;

use characters::player::Player;

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.pop();

    input
}

fn main() {
    let mut hero = Player::new("Hero", "🤠️");

    loop {
        read_user_action(&mut hero);
    }
}

fn read_user_action(player: &mut Player) {
    let action = read_input();
    act(player, action);
}

fn act(player: &mut Player, action: String) {
    match action.as_str() {
        "draw" => player.draw_cards(2),
        "show" => player.show_hand(),
        "play" => {
            let card_index = read_input().parse::<usize>().unwrap();
            player.play_card(card_index)
        }
        _ => println!("No match.\n"),
    }
}
