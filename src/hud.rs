pub mod hud {

    use crate::Player;

    pub fn show_hud(character: &Player) {
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{} {}", character.avatar, character.name);
        println!(
            "🩸️ [{}] {}/{}",
            create_hp_hud(character),
            character.current_hp,
            character.initial_hp
        );
        println!(
            "💪️ {}  🛡️ {}  🦵️ {}  🌞️ {}\n",
            character.strength, character.block, character.dexterity, character.energy
        );
    }

    pub fn create_hp_hud(character: &Player) -> String {
        let hp_percentage = character.current_hp as f32 / character.initial_hp as f32;
        let mut hp_drawing = "#".repeat((hp_percentage * 10.0) as usize);
        let lost_hp_drawing = " ".repeat(10 - (hp_percentage * 10.0) as usize);

        hp_drawing.push_str(lost_hp_drawing.as_str());

        hp_drawing
    }
}
