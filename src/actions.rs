use crate::game_objects::*;
use std::io::{self, Write, stdin, stdout};

pub fn prompt() -> io::Result<String> {
    print!("> ");
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase())
}

pub fn go_cave(player: &mut Player, cave: &mut CaveObj) -> io::Result<()> {
    println!("\nThe cave is dark and smells of wet dog.");
    println!("Something growls in the darkness.");

    while player.get_room() == &RoomId::Cave {
        match prompt()?.as_str() {
            "go west" | "west" | "w" => {
                println!(
                    "\nYou left the cave, and avoided the potential danger that lies in the depths of the cave."
                );
                println!("A thick forest line is in your vicinity.");
                player.change_room(RoomId::Forest);
                return Ok(());
            }
            "get closer" | "approach" | "attack" | "fight" => {
                println!(
                    "\nYou decide to become brave in the very instance of great uncertainty..."
                );
                println!(
                    "\n... but to no avail. The vile creature approaches you and fights back until you fall down, critically wounded."
                );
                println!(
                    "There is nobody to come rescue you. In a few minutes, you will be no more."
                );
                player.die();
                return Ok(());
            }
            "light up torch" | "torch on" => {
                if !player.check_in_inv("torch") {
                    println!("\nNo torch is found in your inventory.");
                    continue;
                }

                if !cave.is_illuminated() {
                    cave.toggle_illuminate();
                }
                println!(
                    "\nYou are lighting up the torch you found, and you can now see the cave's surroundings better."
                );

                if cave.get_monster().is_present() {
                    if !cave.get_monster().is_alert() {
                        cave.get_monster_mut().toggle_alert();
                    }
                    println!(
                        "\nThough the monster is in a way blind, it does sense a presence of some additional light."
                    );
                    println!(
                        "And now the monster is more alert and has grown more vicious to attack you... or anything in its vicinity."
                    );
                } else {
                    println!(
                        "Now that the monster is out of sight, you can see a shiny little object being hidden not far from you."
                    );
                }
            }
            "turn off torch" | "torch off" => {
                if !player.check_in_inv("torch") {
                    println!("\nNo torch is found in your inventory.");
                    continue;
                }

                if cave.is_illuminated() {
                    cave.toggle_illuminate();
                }

                if cave.get_monster().is_alert() {
                    println!(
                        "\nAs the light from your torch fades, the monster slowly fades back into its normal state, yet the danger remains."
                    );
                    if cave.get_monster().is_alert() {
                        cave.get_monster_mut().toggle_alert();
                    }
                }
            }
            "distract" | "throw meat" | "throw dead meat" => {
                if player.check_in_inv("dead meat") {
                    if cave.get_monster().is_alert() {
                        println!(
                            "\nCan't do that now... The monster is now paying more attention to you than to some decoy meat."
                        );
                        println!(
                            "Gotta make sure nothing alerts him before committing to such action."
                        );
                        continue;
                    }
                    player.spend_item("dead meat");
                    cave.get_monster_mut().get_distracted();
                    println!("\nYou threw the meat towards the further, darker side of the cave.");
                    println!("Now the monster does not pay attention to you.");
                }
            }
            "check shiny object" => {
                if !cave.get_monster().is_present() {
                    println!("\nThis is actually the treasure chest key!");
                }
            }
            "get chest key" | "chest key" => {
                if cave.get_monster().is_present() {
                    println!("\nCan't do that now.");
                    continue;
                }

                player.pick_item("chest key");
                cave.complete_location(player);
                println!("\nWelp, that's done! Off we go outta here!");
            }
            _ => {
                println!("\nI'm not exactly sure what you intended to do here.");
            }
        }
    }

    Ok(())
}

pub fn go_forest(player: &mut Player, forest: &mut ForestObj) -> io::Result<()> {
    println!(
        "\nThe forest is dense, thick and you can see through the masses of the trees and bushes surrounding you."
    );
    println!("You are looking around to see what's going on.");

    while player.get_room() == &RoomId::Forest {
        match prompt()?.as_str() {
            "look left" => {
                println!("\nYou notice a bunch of bushes you can cut through.");
            }
            "look right" => {
                println!(
                    "\nThough it gives you some chills, you do noticed an entrance to a cave being drawn to your attention..."
                );
                println!("Do you dare enter it?");
            }
            "cut bushes" | "cut" => {
                if !player.check_in_inv("cutters") {
                    println!("\nYou don't have the necessary items to go past the bushes.");
                    continue;
                }

                player.spend_item("cutters");
                forest.check_bushes();
                println!(
                    "\nYou cut through the bushes, and now you see a weird-looking hump sticking out of the ground."
                );
            }
            "dig hole" | "dig" => {
                if forest.has_bushes() {
                    println!("\nWhat to dig? You can barely see through these bushes...");
                    continue;
                }

                if !player.check_in_inv("shovel") {
                    println!("\nYou don't have the necessary items to go past the bushes.");
                    continue;
                }

                player.spend_item("shovel");
                forest.check_mold();
                println!("\nYou dug a hole that turned out to be not that deep.");
                println!("\nAnd you see there is a torch lying there.");
                println!(
                    "I wonder who left it? There actually might be one place where it could end up being hefty..."
                );
            }
            "pick up torch" | "torch" => {
                if forest.has_mold() {
                    println!("\nCan't do that now.");
                    continue;
                }

                player.get_torch(forest);
                println!("\nRight, got the torch now. I think I'm done waddling around here.");
            }
            "go south" | "south" => {
                player.change_room(RoomId::Road);
                break;
            }
            "go east" | "east" | "go cave" => {
                player.change_room(RoomId::Cave);
                break;
            }
            _ => {
                println!("\nI'm not exactly sure what you intended to do here.");
            }
        }
    }
    Ok(())
}

pub fn go_mansion(player: &mut Player, mansion: &mut MansionObj) -> io::Result<()> {
    println!("\nYou find yourself by the front entrance of the mansion.");

    while player.get_room() == &RoomId::Mansion {
        match prompt()?.as_str() {
            "open front door" | "front door" => {
                println!("\nThe front door is barred from the other side.");
            }
            "open back door" | "back door" => {
                mansion.mark_back_door_opened();
                println!("\nYou noticed a back entrance slightly opened...");
                println!(
                    "Perhaps, someone forgot to ensure it was fully shut when leaving the mansion from this side... Weird"
                );
                println!(
                    "Nonetheless, you enter the mansion and you see a small drawer next to you."
                );
            }
            "check drawer" | "drawer" => {
                println!("\nYou look into the drawer, and you see some keys lying down.");
                println!("And they also look like they are keys to a car?");
            }
            "get car key" | "car key" => {
                if !mansion.check_back_door() {
                    println!("\nCan't do that now.");
                    continue;
                }
                player.get_car_key(mansion);
                mansion.complete_location(player);
                println!("\nKeys obtained. Are there any cars around?");
            }
            "go north" | "north" | "go yard" => {
                player.change_room(RoomId::Yard);
                break;
            }
            "go east" | "east" => {
                player.change_room(RoomId::Road);
                break;
            }
            _ => {
                println!("\nI'm not exactly sure what you intended to do here.");
            }
        }
    }

    Ok(())
}

pub fn go_road(player: &mut Player, road: &mut RoadObj) -> io::Result<()> {
    println!(
        "\nYou are now standing on an empty, misty road that is encumbered by a thick fog and haunted void."
    );
    println!(
        "You can feel the depth of nothingness enfeebling your feet as you familiarize yourself with the surroundings of it."
    );
    println!("And yet you do noticed a few things that immediately grab your attention...");
    println!(
        "\n... I wonder who was driving here? And what in the Supreme Being name's heck happened here?!"
    );
    println!("Best to make haste and check it all out quickly before the sun sets.");

    while player.get_room() == &RoomId::Road {
        match prompt()?.as_str() {
            "look around" => {
                if road.car_checked() && road.dead_meat_checked() {
                    println!(
                        "\nI'm done looking around here. Gotta leave this road until it's too late."
                    );
                    continue;
                }

                println!(
                    "\nThe one and only thing that grabs your attention is a concentrated, and..."
                );
                println!("vile scene... Was there a car accident? A deadly hit and run?");
            }
            "check car" | "go to car" => {
                if road.car_checked() {
                    println!("\nNothing to see here.");
                    continue;
                }

                println!("\nYou approach the car. Obviously, the door is locked.");
                println!("You notice there are cutters lying down in the back seats.");
                road.check_car();
            }
            "open car" | "use car key" => {
                if !player.check_in_inv("car key") || !road.car_checked() {
                    println!("\nCan't do that right now.");
                    continue;
                }

                player.spend_item("car key");
                player.get_cutters(road);

                if !road.dead_meat_checked() {
                    println!("\nGot the door open, now let me get the... AW WTF DUDE?!!");
                    println!("oooh my God... I'm about to puke... Is that... a dead body?!");
                } else {
                    println!("\nAlrighty... I think I'm finally done here...");
                    println!("Best to get the hellz out of here before I lose my shit!");
                    road.mark_completed();
                }
            }
            "check dead body" | "dead body" => {
                println!("\noh for f**k's sake... it's still rotting...");

                if road.is_some_meat() {
                    player.get_dead_meat(road);
                    println!(
                        "I can't believe I actually got the audacity stuff like this... it better be useful, or else..."
                    );
                } else {
                    println!(
                        "There's nothing left of it... at least anything useful for me... thankfully..."
                    );
                }
                road.check_dead();
            }
            "go south" | "south" => {
                player.change_room(RoomId::Town);
                break;
            }
            "go north" | "north" => {
                player.change_room(RoomId::Forest);
                break;
            }
            "go west" | "west" => {
                player.change_room(RoomId::Mansion);
                break;
            }
            _ => {
                println!("\nI'm not exactly sure what you intended to do here.");
            }
        }
    }

    Ok(())
}

pub fn go_yard(player: &mut Player, yard: &mut YardObj) -> io::Result<()> {
    println!(
        "\nYou went just a little further from the north-eastern corner of the mansion, where you spotted a small yet nostalgia-inducing yard."
    );

    while player.get_room() == &RoomId::Yard {
        match prompt()?.as_str() {
            "go south" | "south" => {
                player.change_room(RoomId::Mansion);
                break;
            }
            "check yard" | "look around" => {
                println!("\nYou see a separated double door that might lead to a basement.");
                println!("But why is it located separately like this?");

                if yard.is_shovel_some() {
                    println!("\nYou see a shovel stashed in the corner of a little garden.");
                    println!("Might be useful, actually.");
                } else {
                    println!("Nothing special here. I got everything I needed.");
                }
            }
            "enter basement" | "go basement" | "basement" => {
                println!("\nYou have entered a basement.");
                println!(
                    "It smells here like England's middle ages, old wine, and... rusted wood and metal..."
                );
                println!("Could that be... it? The very treasure chest?");
                yard.check_basement();
            }
            "chest" | "open chest" => {
                if !yard.basement_checked() || !player.check_in_inv("chest key") {
                    println!("\nCan't do that now.");
                    continue;
                }

                player.open_chest(yard);
                player.spend_item("chest key");

                println!(
                    "\nSUCCESS! At last, I have managed to get to the very relic ny uncle Sir Benedict Clobberbanks told me of!"
                );
                println!("I see... A note... and a... ring...");
                println!("Let me read the note, see what it says.");
                break;
            }
            "get shovel" | "shovel" | "pick up shovel" => {
                if yard.is_shovel_some() {
                    player.get_shovel(yard);
                    println!("\nNice! Got a shovel, now let's see where I could use it...");
                } else {
                    println!("No shovel to be found.");
                }
            }
            _ => {
                println!("\nI'm not exactly sure what you intended to do here.");
            }
        }
    }

    Ok(())
}

pub fn go_town(player: &mut Player, town: &mut TownObj) -> io::Result<()> {
    println!("\nThis town used to be the most lively point in the entire area... And now...");
    println!(
        "It's nothing but a Silent Hill of a place... Not even a wandering soul can be ever felt... Except..."
    );
    println!("I wonder if there is anyone around here left?");

    while player.get_room() == &RoomId::Town {
        match prompt()?.as_str() {
            "go north" | "north" => {
                player.change_room(RoomId::Road);
                break;
            }
            _ => {
                println!("\nI'm not exactly sure what you intended to do here.");
            }
        }
    }

    Ok(())
}
