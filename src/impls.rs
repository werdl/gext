use rand::{self, Rng};
use std::fs::File;
use std::io::Write;

use std::collections::HashMap;

use console::Term;

use crate::structs::{BattleResult, Door, Item, Key, Player, Room};

use crate::write;

impl Key {
    pub fn new(name: String) -> Key {
        Key { name }
    }
}

impl Item {
    pub fn new(name: String, description: String, health: i32, attack: i32) -> Item {
        Item {
            name,
            description,
            health,
            attack,
        }
    }
}

impl Door {
    pub fn new(
        name: String,
        description: String,
        locked: bool,
        key: Key,
        enemy: Option<Player>,
        associated_room_name: String,
    ) -> Door {
        Door {
            name,
            description,
            locked,
            key,
            enemy,
            associated_room_name,
        }
    }
}

impl Room {
    pub fn new(
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

impl BattleResult {
    pub fn new(winner: bool, player_health: i32, enemy_health: i32) -> BattleResult {
        BattleResult {
            winner,
            player_health,
            enemy_health,
        }
    }
}

impl Player {
    pub fn new(
        name: String,
        map: HashMap<String, Room>,
        items_held: Vec<Item>,
        keys_held: Vec<Key>,
        health: i32,
        attack: i32,
        battles: Vec<BattleResult>,
        current_room: Room,
        game_name: String,
    ) -> Player {
        Player {
            name,
            map,
            items_held,
            keys_held,
            health,
            attack,
            battles,
            current_room,
            game_name,
        }
    }
    pub fn fight(&mut self, enemy: &Player) -> BattleResult {
        let initial_health = self.health;
        let initial_attack = self.attack;

        let mut rng = rand::thread_rng();

        let mut player_attack = self.attack + rng.gen_range(-(self.attack / 4)..(self.attack / 4));
        let mut enemy_attack =
            enemy.attack + rng.gen_range(-(enemy.attack / 4)..(enemy.attack / 4));

        let enemy_health = enemy.health;

        let mut enemy_health = enemy_health;

        write(
            format!(
            "You are in a fight! You have to fight the enemy! You have {} health and {} attack, the enemy {} health and {} attack.",
            self.health, player_attack, enemy_health, enemy_attack
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
                    "You have {} health and {} attack, the enemy has {} health and {} attack. Do you wish to use an item? (y/n)",
                    self.health, player_attack, enemy_health, enemy_attack
                )
                .as_str(),
                "magenta"
            );

            let input = Term::stdout().read_line().unwrap();

            match input.as_str() {
                "y" => {
                    write("Which item do you want to use?", "magenta");

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

            self.health -= enemy_attack;

            if self.health <= 0 {
                self.battles.push(BattleResult::new(false, 0, enemy_health));

                self.health = initial_health;
                self.attack = initial_attack;

                return BattleResult::new(false, 0, enemy_health);
            }

            enemy_health -= player_attack;

            if enemy_health <= 0 {
                self.battles.push(BattleResult::new(true, self.health, 0));

                self.health = initial_health;
                self.attack = initial_attack;

                return BattleResult::new(true, self.health, 0);
            }

            player_attack = self.attack + rng.gen_range(-(self.attack / 4)..(self.attack / 4));
            enemy_attack = enemy.attack + rng.gen_range(-(enemy.attack / 4)..(enemy.attack / 4));
        }
    }

    pub fn ask_question(&self) -> bool {
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
        questions.insert(
            "What has a bark but no bite?".to_string(),
            vec!["tree".to_string()],
        );
        questions.insert(
            "What has a bed but never sleeps?".to_string(),
            vec!["river".to_string()],
        );
        questions.insert(
            "What has a face and two hands but no arms or legs?".to_string(),
            vec!["clock".to_string()],
        );
        questions.insert(
            "What has a head and a tail but no body?".to_string(),
            vec!["coin".to_string()],
        );
        questions.insert(
            "What has a tongue but cannot talk?".to_string(),
            vec!["shoe".to_string()],
        );
        questions.insert(
            "What has a ring but no finger?".to_string(),
            vec!["telephone".to_string()],
        );
        questions.insert(
            "Mississippi has three 'i's and four 's's. Now, without using 'i' or 's', spell it."
                .to_string(),
            vec!["it".to_string()],
        );
        questions.insert(
            "A man in a car saw a golden door, a silver door, and a bronze door. What door did he open first?".to_string(),
            vec!["car door".to_string(), "car".to_string()],
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

    pub fn move_through_door(&mut self, door_name: String) {
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
                            write(
                                format!(
                                    "The enemy had {} health left (you died with {}).",
                                    result.enemy_health, result.player_health
                                )
                                .as_str(),
                                "red",
                            );
                            return;
                        }
                    }



                    let old_room = self.current_room.clone();

                    self.current_room = self.map.get(&door.associated_room_name).unwrap().clone();

                    if !self
                        .current_room
                        .doors
                        .iter()
                        .any(|d| d.associated_room_name == old_room.name)
                    {
                        self.current_room.doors.push(Door::new(
                            old_room.name.clone(),
                            format!("a door to the {}", old_room.name),
                            false,
                            Key::new("".to_string()),
                            None,
                            old_room.name.clone(),
                        ));
                    }

                    for room in self.map.clone().values() {
                        for door in &room.doors {
                            if door.associated_room_name == self.current_room.name && !self.current_room.doors.iter().any(|d| d.name == room.name){
                                self.current_room.doors.push(
                                    Door::new(
                                        room.name.clone(),
                                        format!("a door to the {}", room.name),
                                        false,
                                        Key::new("".to_string()),
                                        None,
                                        room.name.clone(),
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }

        if !found {
            write("I find no such door", "red");
        } else {
            write(
                format!("You are in the {}", self.current_room.name).as_str(),
                "blue",
            );
        }
    }

    pub fn take_item(&mut self, item_name: String) {
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
        self.map
            .get_mut(&self.current_room.name)
            .unwrap()
            .items
            .retain(|i| i.name != item_name);
    }

    pub fn use_item(&mut self, item: Item) {
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

    pub fn take_key(&mut self, key_name: String) {
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
        self.map
            .get_mut(&self.current_room.name)
            .unwrap()
            .keys
            .retain(|k| k.name != key_name);
    }

    pub fn save(&self) {
        if std::env::args().any(|arg| arg == "--no-save" || arg == "-n") {
            write(
                "Not saving the game, this was ran on development mode.",
                "red",
            );
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

        let mut file =
            File::create(format!("savegames/{}.json", self.game_name)).unwrap_or_else(|err| {
                write(
                    format!(
                        "Error creating the file: {} (ensure savegames dir exists)",
                        err.to_string()
                    )
                    .as_str(),
                    "red",
                );
                std::process::exit(1);
            });

        let json = serde_json::to_string(&self).unwrap();

        file.write_all(json.as_bytes()).unwrap();
    }
}
