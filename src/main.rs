use console::{style, Color, Term};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Key {
    name: String,
}

impl Key {
    fn new(name: String) -> Key {
        Key { name }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, Clone)]
struct Door {
    name: String,
    description: String,
    locked: bool,
    key: Key,

    associated_room: Room,
}

impl Door {
    fn new(name: String, description: String, locked: bool, key: Key, associated_room: Room) -> Door {
        Door {
            name,
            description,
            locked,
            key,
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

#[derive(Debug, Clone)]
struct Room {
    name: String,
    description: String,
    doors: Vec<Door>,
    items: Vec<Item>,
    keys: Vec<Key>,
}

impl Room {
    fn new(name: String, description: String, doors: Vec<Door>, items: Vec<Item>, keys: Vec<Key>) -> Room {
        Room {
            name,
            description,
            doors,
            items,
            keys
        }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    rooms_cleared: Vec<Room>,
    items_held: Vec<Item>,
    keys_held: Vec<Key>,
    health: i32,
    attack: i32,

    current_room: Room,
}

impl Player {

    fn move_through_door(&mut self, door_name: String) {
        let mut found = false;
        for door in &self.current_room.clone().doors {
            if door.name == door_name {
                found = true;
                if door.locked { // todo: check user inventory for key
                    write("The door is locked.", "red");
                } else {
                    self.current_room = door.associated_room.clone();
                }
            }
        }

        if !found {
            write("I find no such door", "red");
        } else {
            write(format!("You are in the {}", self.current_room.name).as_str(), "blue");
        }
    }

    fn take_item(&mut self, item_name: String) {
        let mut found = false;
        for item in &self.current_room.items {
            if item.name == item_name {
                self.items_held.push(item.clone());
                write(format!("You took the {}", item.name).as_str(), "green");
                found = true;
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

        write(format!("You used the {}, your health is now {} and your attack {}", item.name, self.health, self.attack).as_str(), "green");

        self.items_held.retain(|i| i != &item);
    }

    fn take_key(&mut self, key_name: String) {
        let mut found = false;
        for key in &self.current_room.keys {
            if key.name == key_name {
                self.keys_held.push(key.clone());
                write(format!("You took the {}", key.name).as_str(), "green");
                found = true;
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
                "Kitchen".to_string(), "a door to the kitchen".to_string(), true, Key::new("kitchen key".to_string()), 
                Room::new("Kitchen".to_string(), "a room with a stove and a fridge".to_string(), vec![], vec![], vec![])
            ),
        ],
        items: vec![],
        keys: vec![
            Key::new("kitchen key".to_string()),
        ],
    };

    let mut player = Player {
        name: "Player".to_string(),
        rooms_cleared: vec![],
        items_held: vec![],
        keys_held: vec![],
        health: 100,
        attack: 10,
        current_room: room,
    };

    term.write_line(&format!("You are in the {}", player.current_room.name)).unwrap();

    loop {
        let input = term.read_line().unwrap();

        let input = input.trim();

        let commands: Vec<&str> = input.splitn(2, " ").collect();

        match commands[0] {
            "quit" => {
                write("Goodbye!", "green");
                break;
            },

            "look" => {
                out!(player.current_room.description.as_str());
            },

            "go" => {
                if commands.len() < 2 {
                    write("Go where?", "red");
                } else {
                    player.move_through_door(commands[1].to_string());
                }
            },

            "take" => {
                if commands.len() < 2 {
                    write("Take what?", "red");
                } else {
                    player.take_item(commands[1].to_string());
                }
            },

            "takekey" => {
                if commands.len() < 2 {
                    write("Take what?", "red");
                } else {
                    player.take_key(commands[1].to_string());
                }
            },

            "use" => {
                if commands.len() < 2 {
                    write("Use what?", "red");
                } else {
                    let item_from_inventory = player.items_held.iter().find(|i| i.name == commands[1]);

                    match item_from_inventory {
                        Some(item) => {
                            player.use_item(item.clone());
                        },
                        None => {
                            write("You don't have that item.", "red");
                        }
                    }
                }
            },

            "search" => {
                out!("You search the room.", "green");

                println!("{:?}", player.current_room);
            },

            _ => {
                write("I don't understand that command.", "red");
            }
        }
    }
}



