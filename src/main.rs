use std::collections::{hash_map, HashMap};

use console::{style, Color, Term};

use rand::{self, Rng};

use serde::{Deserialize, Serialize};
use serde_json;

use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
struct Key {
    name: String,
}

impl Key {
    fn new(name: String) -> Key {
        Key { name }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
struct Item {
    name: String,
    description: String,
    health: i32,
    attack: i32,
}

impl Item {
    fn new(name: String, description: String, health: i32, attack: i32) -> Item {
        Item {
            name,
            description,
            health,
            attack,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Door {
    name: String,
    description: String,
    locked: bool,
    key: Key,

    enemy: Option<Player>,

    associated_room: Room,
}

impl Door {
    fn new(
        name: String,
        description: String,
        locked: bool,
        key: Key,
        enemy: Option<Player>,
        associated_room: Room,
    ) -> Door {
        Door {
            name,
            description,
            locked,
            key,
            enemy,
            associated_room,
        }
    }

    fn unlock(&mut self, key: Key) -> bool {
        if key == self.key {
            self.locked = false;
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Room {
    name: String,
    description: String,
    doors: Vec<Door>,
    items: Vec<Item>,
    keys: Vec<Key>,
}

impl Room {
    fn new(
        name: String,
        description: String,
        doors: Vec<Door>,
        items: Vec<Item>,
        keys: Vec<Key>,
    ) -> Room {
        Room {
            name,
            description,
            doors,
            items,
            keys,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Player {
    name: String,
    rooms_cleared: Vec<Room>,
    items_held: Vec<Item>,
    keys_held: Vec<Key>,
    health: i32,
    attack: i32,
    is_dead: bool,

    current_room: Room,
}

struct BattleResult {
    winner: bool,
    player_health: i32,
    enemy_health: i32,
}

impl BattleResult {
    fn new(winner: bool, player_health: i32, enemy_health: i32) -> BattleResult {
        BattleResult {
            winner,
            player_health,
            enemy_health,
        }
    }
}

impl Player {
    fn fight(&mut self, enemy: &Player) -> BattleResult {
        let mut rng = rand::thread_rng();

        let player_attack = self.attack + rng.gen_range(0..(self.attack / 4));
        let enemy_attack = enemy.attack + rng.gen_range(0..(enemy.attack / 4));

        let player_health = self.health;
        let enemy_health = enemy.health;

        let mut player_health = player_health;
        let mut enemy_health = enemy_health;

        write(
            format!(
            "You are in a fight! You have to fight the enemy! You have {} health and {} attack, the enemy {} health and {} attack.",
            player_health, player_attack, enemy_health, enemy_attack
            ).as_str(),
            "magenta"
        );

        for item in &self.items_held {
            write(
                format!(
                    "You have the {}, which buffs you {} health and {} attack",
                    item.name, item.health, item.attack
                )
                .as_str(),
                "magenta",
            );
        }

        write("Fight!", "magenta");

        loop {
            write(
                format!(
                    "You have {} health, the enemy has {} health. Do you wish to use an item? (y/n)",
                    player_health, enemy_health
                )
                .as_str(),
                "magenta"
            );

            let input = Term::stdout().read_line().unwrap();

            match input.as_str() {
                "y" => {
                    write("Which item do you want to use?", "magenta");

                    for item in &self.items_held {
                        write(format!("You have the {}", item.name).as_str(), "magenta");
                    }

                    let input = Term::stdout().read_line().unwrap();

                    let input = input.trim();

                    let item_from_inventory = self.items_held.iter().find(|i| i.name == input);

                    match item_from_inventory {
                        Some(item) => {
                            self.use_item(item.clone());
                        }
                        None => {
                            write("You don't have that item.", "red");
                        }
                    }
                }

                _ => {
                    write("You chose not to use an item.", "magenta");
                }
            }

            player_health -= enemy_attack;
            enemy_health -= player_attack;

            if player_health <= 0 {
                return BattleResult::new(false, player_health, enemy_health);
            }

            if enemy_health <= 0 {
                return BattleResult::new(true, player_health, enemy_health);
            }
        }
    }

    fn ask_question(&self) -> bool {
        let mut questions = HashMap::new();

        questions.insert("There are two ducks in front of a duck, two ducks behind a duck and a duck in the middle. How many ducks are there?".to_string(), vec!["3".to_string(), "three".to_string()]);
        questions.insert(
            "What has keys but can't open locks?".to_string(),
            vec!["piano".to_string()],
        );
        questions.insert(
            "What has a head, a tail, is brown, and has no legs?".to_string(),
            vec!["penny".to_string()],
        );
        questions.insert(
            "What has a neck but no head?".to_string(),
            vec!["bottle".to_string()],
        );
        questions.insert(
            "What has a thumb and four fingers but is not alive?".to_string(),
            vec!["glove".to_string()],
        );
        questions.insert(
            "What has a heart that doesn't beat?".to_string(),
            vec!["artichoke".to_string()],
        );
        questions.insert(
            "What has a foot but no legs?".to_string(),
            vec!["snail".to_string()],
        );

        let mut rng = rand::thread_rng();

        let questions_potential: Vec<&String> = questions.keys().collect::<Vec<&String>>();

        let question = questions_potential[rng.gen_range(0..questions_potential.len())];

        let answers = questions.get(question).unwrap();

        write(question, "yellow");

        let input = Term::stdout().read_line().unwrap();

        let input = input.trim();

        if answers.contains(&input.to_string()) {
            write("Correct!", "green");
            return true;
        } else {
            write("Incorrect!", "red");
            return false;
        }
    }

    fn move_through_door(&mut self, door_name: String) {
        let mut found = false;
        for door in &self.current_room.clone().doors {
            if door.name == door_name {
                found = true;
                if door.locked && !self.keys_held.contains(&door.key) {
                    write("The door is locked.", "red");
                    return;
                } else {
                    if let Some(enemy) = &door.enemy {
                        let result = self.fight(enemy);

                        if result.winner {
                            write("You won the fight!", "green");
                        } else {
                            write("You lost the fight, your adventure ends here. :-(", "red");
                            self.is_dead = true;
                            return;
                        }
                    }

                    let old_room = self.current_room.clone();
                    self.current_room = door.associated_room.clone();

                    if self
                        .current_room
                        .doors
                        .iter()
                        .find(|d| d.name == old_room.name)
                        .is_none()
                    {
                        self.current_room.doors.push(Door::new(
                            old_room.name.clone(),
                            format!("a door to the {}", old_room.name),
                            false,
                            Key::new("".to_string()),
                            None,
                            old_room,
                        ));
                    }
                }
            }
        }

        if !found {
            write("I find no such door", "red");
        } else {
            let term = Term::stdout();
            term.clear_screen().unwrap();
            write(
                format!("You are in the {}", self.current_room.name).as_str(),
                "blue",
            );
        }
    }

    fn take_item(&mut self, item_name: String) {
        let mut found = false;
        for item in &self.current_room.items {
            if item.name == item_name {
                found = true;
                if self.ask_question() {
                    self.items_held.push(item.clone());
                    write(
                        format!("You took the {} (item)", item.name).as_str(),
                        "green",
                    );
                } else {
                    write("You got it wrong, the item despawns.", "red");
                }
            }
        }

        if !found {
            write("I find no such item", "red");
        }

        self.current_room.items.retain(|i| i.name != item_name);
    }

    fn use_item(&mut self, item: Item) {
        self.health += item.health;
        self.attack += item.attack;

        write(
            format!(
                "You used the {}, your health is now {} and your attack {}",
                item.name, self.health, self.attack
            )
            .as_str(),
            "green",
        );

        self.items_held.retain(|i| i != &item);
    }

    fn take_key(&mut self, key_name: String) {
        let mut found = false;
        for key in &self.current_room.keys {
            if key.name == key_name {
                found = true;
                if self.ask_question() {
                    self.keys_held.push(key.clone());
                    write(format!("You took the {} (key)", key.name).as_str(), "green");
                } else {
                    write("You got it wrong, the key despawns.", "red");
                }
            }
        }

        if !found {
            write("I find no such key", "red");
        }

        self.current_room.keys.retain(|k| k.name != key_name);
    }

    fn use_key(&mut self, key: Key, door_name: String) {
        for door in &mut self.current_room.doors {
            if door.name == door_name {
                door.unlock(key.clone());
            }
        }

        self.keys_held.retain(|k: &Key| k != &key);
    }

    fn save(&self) {
        if std::env::args().any(|arg| arg == "--no-save" || arg == "-n") {
            write("Not saving the game, this was ran on development mode.", "red");
            write("Manual override (y/n)?", "yellow");

            let input = Term::stdout().read_line().unwrap();

            match input.as_str() {
                "y" => {
                    write("Saving the game.", "green");
                }
                _ => {
                    write("Not saving the game.", "red");
                    return;
                }
            }
        }

        let mut file = File::create("savegame.json").unwrap();

        let json = serde_json::to_string(&self).unwrap();

        file.write_all(json.as_bytes()).unwrap();
    }
}

fn write(text: &str, color: &str) {
    let term = Term::stdout();

    let rgb = match color {
        "red" => Color::Color256(196),
        "green" => Color::Color256(46),
        "blue" => Color::Color256(21),
        "yellow" => Color::Color256(226),
        "magenta" => Color::Color256(201),
        "cyan" => Color::Color256(51),
        "white" => Color::Color256(231),
        _ => Color::Color256(21),
    };

    let styled_text = style(text).fg(rgb);

    term.write_line(&styled_text.to_string()).unwrap();
}

macro_rules! out {
    ($text:expr, $color:expr) => {
        write($text, $color);
    };

    ($text:expr) => {
        write($text, "blue");
    };
}

fn main() {
    let term = Term::stdout();

    let empty_room = Room {
        name: "Empty Room".to_string(),
        description: "a room with nothing in it".to_string(),
        doors: vec![],
        items: vec![],
        keys: vec![],
    };

    let room = Room {
        name: "Entrance Hall".to_string(),
        description: "the first room - the entrance hall".to_string(),
        doors: vec![
            Door::new(
                "Kitchen".to_string(),
                "a door to the kitchen".to_string(),
                true,
                Key::new("kitchen key".to_string()),
                None,
                Room::new(
                    "Kitchen".to_string(),
                    "a room with a stove and a fridge".to_string(),
                    vec![],
                    vec![
                        Item::new("apple".to_string(), "a red apple".to_string(), 10, 0),
                        Item::new("sword".to_string(), "a sharp sword".to_string(), 0, 10),
                    ],
                    vec![],
                ),
            ),
            Door::new(
                "Armoury".to_string(),
                "a door to the armoury".to_string(),
                true,
                Key::new("armoury".to_string()),
                None,
                Room::new(
                    "Armoury".to_string(),
                    "a room with a lot of weapons".to_string(),
                    vec![Door::new(
                        "Trophy Cupboard".to_string(),
                        "a door to the trophy cupboard".to_string(),
                        true,
                        Key::new("trophy cupboard".to_string()),
                        Some(Player {
                            name: "Enemy".to_string(),
                            rooms_cleared: vec![],
                            items_held: vec![],
                            keys_held: vec![],
                            health: 100,
                            attack: 10,
                            is_dead: false,
                            current_room: empty_room.clone(),
                        }),
                        Room::new(
                            "Trophy Cupboard".to_string(),
                            "a room with a lot of trophies".to_string(),
                            vec![],
                            vec![Item::new(
                                "trophy".to_string(),
                                "a shiny trophy showcasing your victory!".to_string(),
                                0,
                                0,
                            )],
                            vec![],
                        ),
                    )],
                    vec![Item::new(
                        "shield".to_string(),
                        "a strong shield".to_string(),
                        0,
                        20,
                    )],
                    vec![Key::new("trophy cupboard".to_string())],
                ),
            ),
        ],
        items: vec![
            Item::new("potion".to_string(), "a red potion".to_string(), 20, 0),
            Item::new(
                "poison vial".to_string(),
                "a poisonous liquid that can be used to throw at your enemy".to_string(),
                0,
                20,
            ),
        ],
        keys: vec![
            Key::new("kitchen key".to_string()),
            Key::new("armoury".to_string()),
        ],
    };

    let mut player = Player {
        name: "Player".to_string(),
        rooms_cleared: vec![],
        items_held: vec![],
        keys_held: vec![],
        health: 100,
        attack: 10,
        is_dead: false,
        current_room: room,
    };

    if let Ok(file) = File::open("savegame.json") {
        player = serde_json::from_reader(file).unwrap();

        let term = Term::stdout();
        term.clear_screen().unwrap();
        write(
            format!("You are in the {}", player.current_room.name).as_str(),
            "blue",
        );
    } else {
        out!("What is your name?", "yellow");

        player.name = term.read_line().unwrap().trim().to_string();

        out!(format!(
            "Welcome to the game, {}! You are in the {}",
            player.name, player.current_room.name
        )
        .as_str());
    }

    loop {
        let input = term.read_line().unwrap();

        let input = input.trim();

        let commands: Vec<&str> = input.splitn(2, " ").collect();

        match commands[0] {
            "quit" => {
                write("Goodbye!", "green");
                player.save();
                break;
            }

            "look" => {
                out!(player.current_room.description.as_str());
            }

            "go" => {
                if commands.len() < 2 {
                    write("Go where?", "red");
                } else {
                    player.move_through_door(commands[1].to_string());
                    if player.is_dead {
                        out!("Game over!", "red");
                        break;
                    }
                }
            }

            "take" => {
                if commands.len() < 2 {
                    write("Take what?", "red");
                } else {
                    player.take_item(commands[1].to_string());
                }
            }

            "takekey" => {
                if commands.len() < 2 {
                    write("Take what?", "red");
                } else {
                    player.take_key(commands[1].to_string());
                }
            }

            "use" => {
                if commands.len() < 2 {
                    write("Use what?", "red");
                } else {
                    let item_from_inventory =
                        player.items_held.iter().find(|i| i.name == commands[1]);

                    match item_from_inventory {
                        Some(item) => {
                            player.use_item(item.clone());
                        }
                        None => {
                            write("You don't have that item.", "red");
                        }
                    }
                }
            }

            "debug" => {
                println!("{:?}", player.current_room);
            }

            "search" => {
                for item in &player.current_room.items {
                    write(
                        format!("You see \"{}\" (item)", item.name).as_str(),
                        "green",
                    );
                }

                for key in &player.current_room.keys {
                    write(format!("You see \"{}\" (key)", key.name).as_str(), "green");
                }

                for door in &player.current_room.doors {
                    write(
                        format!(
                            "You see \"{}\" ({} door)",
                            door.name,
                            if door.locked && !player.keys_held.contains(&door.key) {
                                "locked"
                            } else {
                                "unlocked"
                            }
                        )
                        .as_str(),
                        "green",
                    );
                }
            }

            "help" => {
                out!(format!("").as_str())
            }

            "inventory" => {
                for item in &player.items_held {
                    write(format!("You have the {}", item.name).as_str(), "green");
                }

                for key in &player.keys_held {
                    write(format!("You have the {}", key.name).as_str(), "green");
                }

                write(
                    format!(
                        "You have {} health and {} attack",
                        player.health, player.attack
                    )
                    .as_str(),
                    "green",
                );

                if player.is_dead {
                    out!("You are dead!", "red");
                }
                if player.keys_held.len() == 0 && player.items_held.len() == 0 {
                    out!("You have nothing in your inventory.", "red");
                }
            }

            _ => {
                write("I don't understand that command.", "red");
            }
        }
    }
}