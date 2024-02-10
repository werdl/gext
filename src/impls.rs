use rand::{self, Rng};
use std::fs::File;
use std::io::Write;

use std::collections::HashMap;

use console::Term;

use crate::structs::{BattleResult, Class, Door, Item, Key, Player, Room, RoomRequirements};

use crate::write;

impl Key {
    pub fn new(name: String) -> Key {
        Key { name }
    }
}

impl Item {
    pub fn new(name: String, description: String, health: i32, attack: i32, defense: i32) -> Item {
        Item {
            name,
            description,
            health,
            attack,
            defense,
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
        requirements: Option<RoomRequirements>,
    ) -> Door {
        Door {
            name,
            description,
            locked,
            key,
            enemy,
            associated_room_name,
            requirements,
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
    pub fn new(
        winner: bool,
        player_health: i32,
        enemy_health: i32,
        enemy_name: String,
    ) -> BattleResult {
        BattleResult {
            winner,
            player_health,
            enemy_health,
            enemy_name,
        }
    }
}

impl Class {
    pub fn default() -> Class {
        Class {
            name: "Default".to_string(),
            description: "Looks like somebody didn't choose a class...".to_string(),
            health: 100,
            attack: 10,
            defense: 10,
            starting_items: vec![],
            starting_keys: vec![],
            won_battle_attack_bonus: 5,
            won_battle_defense_bonus: 5,
            won_battle_health_bonus: 5,
        }
    }
    pub fn new(
        name: String,
        description: String,
        health: i32,
        attack: i32,
        defense: i32,
        starting_items: Vec<Item>,
        starting_keys: Vec<Key>,
        won_battle_attack_bonus: i32,
        won_battle_defense_bonus: i32,
        won_battle_health_bonus: i32,
    ) -> Class {
        Class {
            name,
            description,
            health,
            attack,
            defense,
            starting_items,
            starting_keys,
            won_battle_attack_bonus,
            won_battle_defense_bonus,
            won_battle_health_bonus,
        }
    }
}

impl Player {
    pub fn init(
        name: String,
        map: HashMap<String, Room>,
        game_name: String,
        starting_room: String,
    ) -> Player {
        let classes: Vec<Class> = vec![
            Class::new(
                "Warrior".to_string(),
                "A strong and brave warrior.".to_string(),
                100,
                40,
                20,
                vec![Item::new(
                    "sword".to_string(),
                    "A sharp sword.".to_string(),
                    0,
                    10,
                    0,
                )],
                vec![Key::new("key".to_string())],
                5,
                5,
                5,
            ),
            Class::new(
                "Mage".to_string(),
                "A wise and powerful mage.".to_string(),
                150,
                15,
                5,
                vec![Item::new(
                    "staff".to_string(),
                    "A powerful staff.".to_string(),
                    0,
                    15,
                    0,
                )],
                vec![],
                10,
                0,
                10,
            ),
            Class::new(
                "Rogue".to_string(),
                "A sneaky and agile rogue.".to_string(),
                90,
                5,
                25,
                vec![Item::new(
                    "dagger".to_string(),
                    "A sharp dagger.".to_string(),
                    0,
                    5,
                    0,
                )],
                vec![],
                0,
                10,
                5,
            ),
            Class::new(
                "Monk".to_string(),
                "A peaceful and strong monk.".to_string(),
                175,
                5,
                10,
                vec![Item::new(
                    "strong will".to_string(),
                    "A strong mind trumps any weapon.".to_string(),
                    100,
                    0,
                    0,
                )],
                vec![],
                5,
                5,
                5,
            ),
            Class::new(
                "Dark Mage".to_string(),
                "A mage with no buffs - but has a second shot at every battle.".to_string(),
                100,
                20,
                10,
                vec![],
                vec![],
                0,
                0,
                0,
            )
        ];

        write("Choose a class:", "yellow");

        for class in &classes {
            write(
                format!("{}: {} -  {}❤️, {}🪓, {}🛡️", class.name, class.description, class.health, class.attack, class.defense).as_str(),
                "yellow",
            );
        }

        let input = Term::stdout().read_line().unwrap();

        let input = input.trim().to_lowercase();

        let class = classes.iter().find(|c| c.name.to_lowercase() == input).unwrap_or(&classes[0]);

        write(
            format!(
                "You chose the {} class, which has {}❤️, {}🪓 and {}🛡️.",
                class.name, class.health, class.attack, class.defense
            )
            .as_str(),
            "green",
        );

        let player = Player {
            name,
            map: map.clone(),
            items_held: class.starting_items.clone(),
            keys_held: class.starting_keys.clone(),
            health: class.health,
            attack: class.attack,
            defense: class.defense,
            battles: vec![],
            current_room: map.clone().get(&starting_room).unwrap().clone(),
            game_name,
            class: class.clone(),
        };

        player
    }
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
        defense: i32,
        class: Class
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
            defense,
            class
        }
    }
    pub fn fight(&mut self, enemy: &mut Player) -> BattleResult {
        let initial_enemy = enemy.clone();

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
            "You are in a fight! You have to fight the enemy! You have {}❤️, {}🪓 and {}🛡️, the enemy {}❤️, {}🪓 and {}🛡️. Your class is {}.",
            self.health, player_attack, self.defense, enemy_health, enemy_attack, enemy.defense, self.class.name
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
                    "You have {}❤️, {}🪓 and {}🛡️, the enemy has {}❤️, {}🪓 and {}🛡️. Do you wish to use an item? (y/n)",
                    self.health, player_attack, self.defense, enemy_health, enemy_attack, enemy.defense
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
                                "You have the {}, which buffs you {}❤️, {}🪓 and {}🛡️",
                                item.name, item.health, item.attack, item.defense
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
                            write(
                                format!(
                                    "You use the {}, which buffs you {}❤️, {}🪓 and {}🛡️",
                                    item.name, item.health, item.attack, item.defense
                                )
                                .as_str(),
                                "green",
                            );
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

            if enemy.items_held.len() > 0 && rng.gen_bool(0.5) {
                let item = enemy.items_held[rng.gen_range(0..enemy.items_held.len())].clone();

                write(
                    format!(
                        "The enemy used the {}, which buffs them {}❤️, {}🪓 and {}🛡️.",
                        item.name, item.health, item.attack, item.defense
                    )
                    .as_str(),
                    "green",
                );

                enemy.use_item(item);
            }

            self.health -= (enemy_attack - self.defense).max(0);

            if self.health <= 0 {
                self.battles.push(BattleResult::new(
                    false,
                    0,
                    enemy_health,
                    enemy.name.clone(),
                ));

                self.health = initial_health;
                self.attack = initial_attack;

                if self.class.name == "Dark Mage" {
                    write("You lost the fight, but you have a second chance! However, you now reassign your class to a different one.", "red");
                    self.class = Player::init(self.name.clone(), self.map.clone(), self.game_name.clone(), self.current_room.name.clone()).class;
                    return self.fight(&mut initial_enemy.clone());
                }

                return BattleResult::new(false, 0, enemy_health, enemy.name.clone());
            }

            enemy_health -= (player_attack - enemy.defense).max(0);

            if enemy_health <= 0 {
                self.battles
                    .push(BattleResult::new(true, self.health, 0, enemy.name.clone()));

                self.health = initial_health;
                self.attack = initial_attack;

                return BattleResult::new(true, self.health, 0, enemy.name.clone());
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
            "Mississippi has two 'p's and four 's's. Now, without using 'p' or 's', spell it."
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
                    if let Some(requirements) = &door.requirements {
                        if self.health < requirements.health || self.attack < requirements.attack {
                            write(format!("You don't meet the requirements to go through this door. It needs {} attack and {} health, whereas you only have {} attack and {} health.", requirements.attack, requirements.health, self.attack, self.health).as_str(), "red");
                            return;
                        }
                    }
                    if door.enemy.is_some()
                        && !self
                            .battles
                            .iter()
                            .any(|b| b.enemy_name == door.enemy.clone().unwrap().name)
                    {
                        let enemy = &mut door.enemy.clone().unwrap();
                        let result = self.fight(enemy);

                        if result.winner {
                            write(
                                "You won the fight! You gain the enemy's stats they had at the start of the fight.",
                                "green",
                            );
                            self.health += enemy.health + (self.class.won_battle_health_bonus);
                            self.attack += enemy.attack + (self.class.won_battle_attack_bonus);
                            self.defense += enemy.defense + (self.class.won_battle_defense_bonus);

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
                            None,
                        ));
                    }

                    for room in self.map.clone().values() {
                        for door in &room.doors {
                            if door.associated_room_name == self.current_room.name
                                && !self.current_room.doors.iter().any(|d| d.name == room.name)
                            {
                                self.current_room.doors.push(Door::new(
                                    room.name.clone(),
                                    format!("a door to the {}", room.name),
                                    false,
                                    Key::new("".to_string()),
                                    None,
                                    room.name.clone(),
                                    None,
                                ));
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
            File::create(format!("savegames/{}.save.json", self.game_name)).unwrap_or_else(|err| {
                write(
                    format!(
                        "Error creating the file: {} (ensure savegames dir exists)",
                        err.to_string()
                    )
                    .as_str(),
                    "red",
                );

                File::create("gext.save.json").unwrap()
            });

        let json = serde_json::to_string(&self).unwrap();

        file.write_all(json.as_bytes()).unwrap();
    }
}
