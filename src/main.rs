use mini_zork::{actions::*, game_objects::*};
use std::io;

fn main() -> io::Result<()> {
    println!("=== MINI ZORK ===");
    println!("A tiny text-based adventure");

    let mut player = Player::new("Hector");
    let mut mansion = MansionObj::new();
    let mut cave = CaveObj::new();
    let mut forest = ForestObj::new();
    let mut road = RoadObj::new();
    let mut town = TownObj::new();
    let mut yard = YardObj::new();

    println!("\nThis mansion used to belong to your uncle Sir Benedict Clobberbanks.");
    println!(
        "You have recently inherited it due to his critical health complications, of which you were informed by an associate that has known him and you for a while."
    );
    println!(
        "That associate passed you a small note where your uncle told you what you have to do in order to secure the future of your fmaily's generations."
    );
    println!(
        "The note reads: \"Find the sacred treasure, Hector... And remember that our family's legacy heavily relies on your well-renowned wittiness and dignity. Best of luck!\""
    );

    while player.get_state() == &State::Alive {
        if yard.chest_unlocked() {
            println!("Dear Hector... My beloved nephew... If you are reading this note...");
            println!(
                "That means you have made it. You are indeed the lawful and all-righteous heir to the mansion in the territory your feet are on."
            );
            println!(
                "The ring that you saw enxt to this note - wear it. Keep it. let it remind you of the power and the great influence of the Clobberbanks legacy."
            );
            println!(
                "\nYour quest concludes here. But fear not - do ventureth off overseas, and who knows?"
            );
            println!(
                "You may find more explorations to tackle on. One more timne - congratulations!"
            );
            break;
        }

        match player.get_room() {
            &RoomId::Cave => go_cave(&mut player, &mut cave)?,
            &RoomId::Mansion => go_mansion(&mut player, &mut mansion)?,
            &RoomId::Forest => go_forest(&mut player, &mut forest)?,
            &RoomId::Road => go_road(&mut player, &mut road)?,
            &RoomId::Yard => go_yard(&mut player, &mut yard)?, // todo!
            &RoomId::Town => go_town(&mut player, &mut town)?, // todo!
        };
    }

    if player.get_state() == &State::Dead {
        println!("\nWelp... You tried... Better luck next time.");
    }

    Ok(())
}
