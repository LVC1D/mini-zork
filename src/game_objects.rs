use crate::game_objects::{
    RoomId::*,
    State::{Alive, Dead},
};

#[derive(Debug, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

#[derive(Debug, PartialEq)]
pub enum RoomId {
    Mansion,
    Forest,
    Cave,
    Road,
    Yard,
    Town,
}

#[derive(Debug, PartialEq)]
pub struct Player {
    name: String,
    state: State,
    room: RoomId,
    inventory: Vec<String>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Alive,
            room: Mansion,
            inventory: Vec::new(),
        }
    }

    pub fn die(&mut self) {
        self.inventory.clear();
        self.state = Dead;
    }

    pub fn change_room(&mut self, room: RoomId) {
        self.room = room;
    }

    pub fn pick_item(&mut self, item: &str) {
        self.inventory.push(item.to_string());
    }

    pub fn spend_item(&mut self, item: &str) {
        self.inventory.retain(|x| x != item);
    }

    pub fn get_room(&self) -> &RoomId {
        &self.room
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn check_in_inv(&self, item: &str) -> bool {
        self.inventory.iter().any(|x| x == item)
    }

    pub fn get_chest_key(&mut self, cave: &mut CaveObj) {
        if let Some(grabbed) = cave.chest_key.take() {
            self.pick_item(grabbed.as_str());
        } else {
            println!("Already picked the chest key up.");
        }
    }

    pub fn get_car_key(&mut self, mansion: &mut MansionObj) {
        if let Some(grabbed) = mansion.back_drawer.take() {
            self.pick_item(grabbed.as_str());
        } else {
            println!("Already picked the car key up.");
        }
    }

    pub fn get_torch(&mut self, forest: &mut ForestObj) {
        if forest.has_mold || !self.inventory.contains(&String::from("shovel")) {
            println!("Can't have the torch yet.");
            return;
        }

        if let Some(grabbed) = forest.torch.take() {
            self.pick_item(grabbed.as_str());
            forest.is_checked = true;
        } else {
            println!("Already got the item");
        }
    }

    pub fn get_shovel(&mut self, yard: &mut YardObj) {
        if let Some(grabbed) = yard.shovel.take() {
            self.pick_item(grabbed.as_str());
        } else {
            println!("Already got the item");
        }
    }

    pub fn open_chest(&self, yard: &mut YardObj) {
        if self.inventory.contains(&String::from("chest key")) {
            yard.chest_unlocked = true;
        }
    }

    pub fn get_dead_meat(&mut self, road: &mut RoadObj) {
        if let Some(meat) = road.dead_meat.take() {
            self.inventory.push(meat);
        }
    }

    pub fn get_cutters(&mut self, road: &mut RoadObj) {
        if let Some(cutters) = road.car_inventory.take() {
            self.inventory.push(cutters);
        }
    }
}

#[derive(Debug)]
pub struct Monster {
    is_present: bool,
    is_alert: bool,
}

impl Monster {
    pub fn new() -> Self {
        Self {
            is_present: true,
            is_alert: false,
        }
    }

    pub fn get_distracted(&mut self) {
        self.is_present = false;
    }

    pub fn toggle_alert(&mut self) {
        if !self.is_present {
            self.is_present = true;
        }
        self.is_alert = !self.is_alert;
    }

    pub fn is_alert(&self) -> bool {
        self.is_alert
    }

    pub fn is_present(&self) -> bool {
        self.is_present
    }
}

// room objects
#[derive(Debug)]
pub struct MansionObj {
    back_door_opened: bool,
    is_checked: bool,
    back_drawer: Option<String>,
}

impl MansionObj {
    pub fn new() -> Self {
        Self {
            back_door_opened: false,
            is_checked: false,
            back_drawer: Some(String::from("car key")),
        }
    }

    pub fn mark_back_door_opened(&mut self) {
        self.back_door_opened = true;
    }

    pub fn check_back_door(&self) -> bool {
        self.back_door_opened
    }

    pub fn complete_location(&mut self, player: &mut Player) {
        if player.inventory.contains(&String::from("car key")) {
            self.is_checked = true;
        }
    }
}

#[derive(Debug)]
pub struct CaveObj {
    monster: Monster,
    chest_key: Option<String>,
    is_illuminated: bool,
    is_checked: bool,
}

impl CaveObj {
    pub fn new() -> Self {
        Self {
            monster: Monster::new(),
            chest_key: Some("chest key".to_string()),
            is_illuminated: false,
            is_checked: false,
        }
    }

    pub fn get_monster(&self) -> &Monster {
        &self.monster
    }

    pub fn get_monster_mut(&mut self) -> &mut Monster {
        &mut self.monster
    }

    pub fn complete_location(&mut self, player: &Player) {
        if player.inventory.contains(&String::from("chest key")) {
            self.is_checked = true;
        }
    }

    pub fn toggle_illuminate(&mut self) {
        self.is_illuminated = !self.is_illuminated;
    }

    pub fn is_illuminated(&self) -> bool {
        self.is_illuminated
    }
}

#[derive(Debug)]
pub struct ForestObj {
    torch: Option<String>,
    has_bushes: bool,
    has_mold: bool,
    is_checked: bool,
}

impl ForestObj {
    pub fn new() -> Self {
        Self {
            torch: Some("torch".to_string()),
            has_bushes: true,
            has_mold: true,
            is_checked: false,
        }
    }

    pub fn check_bushes(&mut self) {
        self.has_bushes = false;
    }

    pub fn check_mold(&mut self) {
        self.has_mold = false;
    }

    pub fn has_bushes(&self) -> bool {
        self.has_bushes
    }

    pub fn has_mold(&self) -> bool {
        self.has_mold
    }

    pub fn mark_completed(&mut self) {
        self.is_checked = true;
    }
}

#[derive(Debug)]
pub struct YardObj {
    shovel: Option<String>,
    basement_checked: bool,
    chest_unlocked: bool,
}

impl YardObj {
    pub fn new() -> Self {
        Self {
            shovel: Some("shovel".to_string()),
            basement_checked: false,
            chest_unlocked: false,
        }
    }

    pub fn is_shovel_some(&self) -> bool {
        self.shovel.is_some()
    }

    pub fn chest_unlocked(&self) -> bool {
        self.chest_unlocked
    }

    pub fn check_basement(&mut self) {
        self.basement_checked = true;
    }

    pub fn basement_checked(&self) -> bool {
        self.basement_checked
    }
}

#[derive(Debug)]
pub struct RoadObj {
    car_checked: bool,
    dead_meat_checked: bool,
    car_inventory: Option<String>,
    dead_meat: Option<String>,
    is_checked: bool,
}

impl RoadObj {
    pub fn new() -> Self {
        Self {
            car_checked: false,
            dead_meat_checked: false,
            car_inventory: Some("cutters".to_string()),
            dead_meat: Some("dead meat".to_string()),
            is_checked: false,
        }
    }

    pub fn check_car(&mut self) {
        self.car_checked = true;
    }

    pub fn car_checked(&self) -> bool {
        self.car_checked
    }

    pub fn check_dead(&mut self) {
        self.dead_meat_checked = true;
    }

    pub fn is_some_meat(&self) -> bool {
        self.dead_meat.is_some()
    }

    pub fn dead_meat_checked(&self) -> bool {
        self.dead_meat_checked
    }

    pub fn mark_completed(&mut self) {
        if self.car_inventory.is_none() && self.dead_meat.is_none() {
            self.is_checked = true;
        }
    }
}

#[derive(Debug)]
pub struct TownObj {
    store_owner: String,
    hints_count: u8,
}

impl TownObj {
    pub fn new() -> Self {
        Self {
            store_owner: "Isaiah Creeks".to_string(),
            hints_count: 3,
        }
    }

    pub fn hints_left(&self) -> u8 {
        self.hints_count
    }

    pub fn request_hint(&mut self) {
        if self.hints_count == 0 {
            return;
        }

        self.hints_count -= 1;
    }
}

pub trait RoomHelper {
    fn give_hint(&self);
    fn if_is_completed(&self) -> bool;
}
