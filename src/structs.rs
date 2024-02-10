use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct RoomRequirements {
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Key {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Door {
    pub name: String,
    pub description: String,
    pub locked: bool,
    pub key: Key,

    pub enemy: Option<Player>,

    pub associated_room_name: String,

    pub requirements: Option<RoomRequirements>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub doors: Vec<Door>,
    pub items: Vec<Item>,
    pub keys: Vec<Key>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub class: Class,
    pub name: String,

    pub map: HashMap<String, Room>,

    pub items_held: Vec<Item>,
    pub keys_held: Vec<Key>,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,

    pub battles: Vec<BattleResult>,

    pub current_room: Room,

    pub game_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleResult {
    pub winner: bool,
    pub player_health: i32,
    pub enemy_health: i32,

    pub enemy_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub description: String,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,

    pub starting_items: Vec<Item>,
    pub starting_keys: Vec<Key>,

    pub won_battle_attack_bonus: i32,
    pub won_battle_defense_bonus: i32,
    pub won_battle_health_bonus: i32,
}
