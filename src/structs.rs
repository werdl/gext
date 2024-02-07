use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Door {
    pub name: String,
    pub description: String,
    pub locked: bool,
    pub key: Key,

    pub enemy: Option<Player>,

    pub associated_room_name: String,
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
    pub name: String,

    pub map: HashMap<String, Room>,

    pub items_held: Vec<Item>,
    pub keys_held: Vec<Key>,
    pub health: i32,
    pub attack: i32,

    pub battles: Vec<BattleResult>,

    pub current_room: Room,

    pub game_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleResult {
    pub winner: bool,
    pub player_health: i32,
    pub enemy_health: i32,
}
