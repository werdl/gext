use std::collections::HashMap;

use console::{style, Color, Term};
use serde_json;

use std::fs::File;

mod impls;
mod structs;

use structs::{Door, Item, Key, Player, Room, RoomRequirements};

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
        write($text, $color)
    };

    ($text:expr) => {
        write($text, "blue")
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

    let empty_map: HashMap<String, Room> = HashMap::new();

    let mut rooms = HashMap::new();

    rooms.insert(
        "Kitchen".to_string(),
        Room::new(
            "Kitchen".to_string(),
            "a room with a stove and a fridge".to_string(),
            vec![
                Door::new(
                    "Pantry".to_string(),
                    "a room with a lot of food".to_string(),
                    true,
                    Key::new("pantry".to_string()),
                    None,
                    "Pantry".to_string(),
                    None,
                ),
                Door::new(
                    "Dining Room".to_string(),
                    "a room with a table and chairs".to_string(),
                    false,
                    Key::new("dining room".to_string()),
                    None,
                    "Dining Room".to_string(),
                    None,
                ),
            ],
            vec![
                Item::new("apple".to_string(), "a red apple".to_string(), 10, 0),
                Item::new("sword".to_string(), "a sharp sword".to_string(), 0, 10),
            ],
            vec![Key::new("elf basement".to_string())],
        ),
    );

    rooms.insert(
        "Entrance Hall".to_string(),
        Room::new(
            "Entrance Hall".to_string(),
            "the first room - the entrance hall".to_string(),
            vec![
                Door::new(
                    "Kitchen".to_string(),
                    "a room with a stove and a fridge".to_string(),
                    true,
                    Key::new("kitchen".to_string()),
                    None,
                    "Kitchen".to_string(),
                    None,
                ),
                Door::new(
                    "Armory".to_string(),
                    "a room with a lot of weapons".to_string(),
                    false,
                    Key::new("".to_string()),
                    None,
                    "Armory".to_string(),
                    None,
                ),
                Door::new(
                    "Concert Hall".to_string(),
                    "a room with a stage and a lot of seats".to_string(),
                    true,
                    Key::new("ticket to the concert hall".to_string()),
                    None,
                    "Concert Hall".to_string(),
                    None,
                ),
                Door::new(
                    "Forest".to_string(),
                    "a room with a lot of trees".to_string(),
                    false,
                    Key::new("".to_string()),
                    None,
                    "Forest".to_string(),
                    None,
                ),
                Door::new(
                    "Downstairs Staircase".to_string(),
                    "a staircase leading to the basement".to_string(),
                    false,
                    Key::new("".to_string()),
                    None,
                    "Downstairs Staircase".to_string(),
                    None,
                ),
            ],
            vec![
                Item::new("potion".to_string(), "a red potion".to_string(), 20, 0),
                Item::new(
                    "poison vial".to_string(),
                    "a poisonous liquid that can be used to throw at your enemy".to_string(),
                    0,
                    20,
                ),
            ],
            vec![
                Key::new("kitchen".to_string()),
                Key::new("concert hall ticket".to_string()),
            ],
        ),
    );

    rooms.insert(
        "North Dungeon".to_string(),
        Room::new(
            "North Dungeon".to_string(),
            "a dark dungeon".to_string(),
            vec![],
            vec![Item::new(
                "boulder".to_string(),
                "a boulder".to_string(),
                50,
                0,
            )],
            vec![],
        ),
    );

    rooms.insert(
        "East Dungeon".to_string(),
        Room::new(
            "East Dungeon".to_string(),
            "a dark dungeon".to_string(),
            vec![Door::new(
                "East Dungeon Cell".to_string(),
                "a dark dungeon cell".to_string(),
                false,
                Key::new("".to_string()),
                None,
                "Dungeon".to_string(),
                None,
            )],
            vec![Item::new("stick".to_string(), "a stick".to_string(), 0, 1)],
            vec![],
        ),
    );

    rooms.insert(
        "West Dungeon".to_string(),
        Room::new(
            "West Dungeon".to_string(),
            "a dark dungeon".to_string(),
            vec![],
            vec![Item::new(
                "stale bread".to_string(),
                "a loaf of bread".to_string(),
                -10,
                0,
            )],
            vec![],
        ),
    );

    rooms.insert(
        "South Dungeon".to_string(),
        Room::new(
            "South Dungeon".to_string(),
            "a dark dungeon".to_string(),
            vec![],
            vec![Item::new(
                "window bar".to_string(),
                "a bar".to_string(),
                0,
                50,
            )],
            vec![],
        ),
    );

    rooms.insert(
        "East Dungeon Cell".to_string(),
        Room::new(
            "East Dungeon Cell".to_string(),
            "a dark dungeon cell".to_string(),
            vec![],
            vec![Item::new(
                "chain".to_string(),
                "a chain".to_string(),
                0,
                100,
            )],
            vec![Key::new("dungeon key".to_string())],
        ),
    );

    rooms.insert(
        "Dungeon Corridor".to_string(),
        Room::new(
            "Dungeon Corridor".to_string(),
            "a dark corridor".to_string(),
            vec![
                Door::new(
                    "North Dungeon".to_string(),
                    "a dark dungeon".to_string(),
                    true,
                    Key::new("dungeon key".to_string()),
                    None,
                    "Dungeon".to_string(),
                    None,
                ),
                Door::new(
                    "East Dungeon".to_string(),
                    "a dark dungeon".to_string(),
                    false,
                    Key::new("".to_string()),
                    None,
                    "Dungeon".to_string(),
                    None,
                ),
                Door::new(
                    "West Dungeon".to_string(),
                    "a dark dungeon".to_string(),
                    true,
                    Key::new("dungeon key".to_string()),
                    None,
                    "Dungeon".to_string(),
                    None,
                ),
                Door::new(
                    "South Dungeon".to_string(),
                    "a dark dungeon".to_string(),
                    false,
                    Key::new("dungeon key".to_string()),
                    None,
                    "Dungeon".to_string(),
                    None,
                ),
            ],
            vec![],
            vec![],
        ),
    );

    rooms.insert(
        "Downstairs Staircase".to_string(),
        Room::new(
            "Downstairs Staircase".to_string(),
            "a staircase leading to the basement".to_string(),
            vec![Door::new(
                "Dungeon Corridor".to_string(),
                "a dark corridor".to_string(),
                true,
                Key::new("dungeons".to_string()),
                None,
                "Dungeon Corridor".to_string(),
                None,
            )],
            vec![],
            vec![],
        ),
    );

    rooms.insert(
        "Elf Treehouse".to_string(),
        Room::new(
            "Elf Treehouse".to_string(),
            "a treehouse full of elves".to_string(),
            vec![Door::new(
                "Elf Basement".to_string(),
                "a basement full of elves".to_string(),
                true,
                Key::new("elf basement".to_string()),
                Some(Player::new(
                    "Elf".to_string(),
                    empty_map.clone(),
                    vec![],
                    vec![],
                    800,
                    200,
                    vec![],
                    empty_room.clone(),
                    "".to_string(),
                )),
                "Elf Basement".to_string(),
                None,
            )],
            vec![
                Item::new("elf hat".to_string(), "a hat".to_string(), 0, 10),
                Item::new(
                    "elf shoes".to_string(),
                    "a pair of shoes".to_string(),
                    10,
                    0,
                ),
            ],
            vec![Key::new("dungeons".to_string())],
        ),
    );

    rooms.insert(
        "Elf Basement".to_string(),
        Room::new(
            "Elf Basement".to_string(),
            "a basement full of elves".to_string(),
            vec![],
            vec![Item::new(
                "elven scythe".to_string(),
                "a scythe".to_string(),
                0,
                70,
            )],
            vec![],
        ),
    );

    rooms.insert(
        "Forest Cabin".to_string(),
        Room::new(
            "Forest Cabin".to_string(),
            "a little hideaway".to_string(),
            vec![Door::new(
                "Elf Treehouse".to_string(),
                "a treehouse full of elves".to_string(),
                false,
                Key::new("".to_string()),
                None,
                "Elf Treehouse".to_string(),
                None,
            )],
            vec![
                Item::new("beans".to_string(), "a can of beans".to_string(), 10, 0),
                Item::new("axe".to_string(), "a sharp axe".to_string(), 0, 10),
                Item::new("beanbag".to_string(), "a beanbag".to_string(), 20, 10),
            ],
            vec![],
        ),
    );

    rooms.insert(
        "Forest Clearing".to_string(),
        Room::new(
            "Forest Clearing".to_string(),
            "a room with a lot of trees and a clearing".to_string(),
            vec![Door::new(
                "Forest Cabin".to_string(),
                "a little hideaway".to_string(),
                true,
                Key::new("cabin weekend pass".to_string()),
                None,
                "Forest Cabin".to_string(),
                None,
            )],
            vec![
                Item::new("stick".to_string(), "a stick".to_string(), 0, 5),
                Item::new("rock".to_string(), "a rock".to_string(), 5, 0),
                Item::new("mushroom".to_string(), "a mushroom".to_string(), 40, 0),
                Item::new("berry".to_string(), "a berry".to_string(), 10, 0),
            ],
            vec![],
        ),
    );

    rooms.insert(
        "Forest".to_string(),
        Room::new(
            "Forest".to_string(),
            "a room with a lot of trees".to_string(),
            vec![Door::new(
                "Forest Clearing".to_string(),
                "a room with a lot of trees and a clearing".to_string(),
                false,
                Key::new("".to_string()),
                None,
                "Forest Clearing".to_string(),
                None,
            )],
            vec![
                Item::new("stick".to_string(), "a stick".to_string(), 0, 5),
                Item::new("rock".to_string(), "a rock".to_string(), 5, 0),
                Item::new("mushroom".to_string(), "a mushroom".to_string(), 40, 0),
                Item::new("berry".to_string(), "a berry".to_string(), 10, 0),
                Item::new(
                    "felled tree".to_string(),
                    "a felled tree".to_string(),
                    0,
                    50,
                ),
            ],
            vec![],
        ),
    );

    rooms.insert(
        "Jousting Store".to_string(),
        Room::new(
            "Jousting Store".to_string(),
            "a room with a lot of jousting equipment".to_string(),
            vec![],
            vec![
                Item::new("helmet".to_string(), "a helmet".to_string(), 20, 0),
                Item::new(
                    "jousting stick".to_string(),
                    "a jousting stick".to_string(),
                    0,
                    100,
                ),
            ],
            vec![Key::new("cabin weekend pass".to_string())],
        ),
    );

    rooms.insert(
        "Jousting Arena".to_string(),
        Room::new(
            "Jousting Arena".to_string(),
            "a room with a lot of horses and knights".to_string(),
            vec![Door::new(
                "Jousting Store".to_string(),
                "a room with a lot of jousting equipment".to_string(),
                false,
                Key::new("".to_string()),
                Some(Player::new(
                    "Knight".to_string(),
                    empty_map.clone(),
                    vec![],
                    vec![],
                    400,
                    80,
                    vec![],
                    empty_room.clone(),
                    "".to_string(),
                )),
                "Armory".to_string(),
                Some(RoomRequirements {
                    health: 400,
                    attack: 60,
                }),
            )],
            vec![
                Item::new("lance".to_string(), "a lance".to_string(), 0, 20),
                Item::new("horse".to_string(), "a horse".to_string(), 150, 0),
            ],
            vec![],
        ),
    );

    rooms.insert(
        "Armory".to_string(),
        Room::new(
            "Armory".to_string(),
            "a room with a lot of weapons".to_string(),
            vec![Door::new(
                "Trophy Cupboard".to_string(),
                "a room with a lot of trophies".to_string(),
                true,
                Key::new("trophy cupboard".to_string()),
                Some(Player::new(
                    "Trophy Keeper".to_string(),
                    empty_map.clone(),
                    vec![],
                    vec![],
                    100,
                    20,
                    vec![],
                    empty_room.clone(),
                    "".to_string(),
                )),
                "Trophy Cupboard".to_string(),
                None,
            )],
            vec![
                Item::new("shield".to_string(), "a shield".to_string(), 20, 0),
                Item::new("axe".to_string(), "a sharp axe".to_string(), 0, 20),
            ],
            vec![
                Key::new("trophy cupboard".to_string()),
                Key::new("pantry".to_string()),
            ],
        ),
    );

    rooms.insert(
        "Trophy Cupboard".to_string(),
        Room::new(
            "Trophy Cupboard".to_string(),
            "a room with a lot of trophies".to_string(),
            vec![],
            vec![
                Item::new("trophy".to_string(), "a golden trophy".to_string(), 30, 0),
                Item::new("bow".to_string(), "a bow".to_string(), 0, 30),
            ],
            vec![Key::new("silverware drawer".to_string())],
        ),
    );

    rooms.insert(
        "Pantry".to_string(),
        Room::new(
            "Pantry".to_string(),
            "a room with a lot of food".to_string(),
            vec![Door::new(
                "Silverware Drawer".to_string(),
                "a room with a lot of silverware".to_string(),
                true,
                Key::new("silverware drawer".to_string()),
                Some(Player::new(
                    "Silverware Demon".to_string(),
                    empty_map.clone(),
                    vec![],
                    vec![],
                    100,
                    20,
                    vec![],
                    empty_room.clone(),
                    "".to_string(),
                )),
                "Silverware Drawer".to_string(),
                None,
            )],
            vec![
                Item::new("bread".to_string(), "a loaf of bread".to_string(), 10, 0),
                Item::new("dagger".to_string(), "a sharp dagger".to_string(), 0, 10),
            ],
            vec![],
        ),
    );

    rooms.insert(
        "Silverware Drawer".to_string(),
        Room::new(
            "Silverware Drawer".to_string(),
            "a room with a lot of silverware".to_string(),
            vec![],
            vec![
                Item::new("fork".to_string(), "a fork".to_string(), 5, 0),
                Item::new("knife".to_string(), "a knife".to_string(), 0, 5),
                Item::new(
                    "Grandma's Special Spoon".to_string(),
                    "a spoon".to_string(),
                    0,
                    50,
                ),
            ],
            vec![],
        ),
    );

    rooms.insert(
        "Dining Room".to_string(),
        Room::new(
            "Dining Room".to_string(),
            "a room with a table and chairs".to_string(),
            vec![],
            vec![
                Item::new("chair".to_string(), "a chair".to_string(), 0, 0),
                Item::new("tablecloth".to_string(), "a table".to_string(), 0, 20),
            ],
            vec![],
        ),
    );

    rooms.insert(
        "Concert Hall".to_string(),
        Room::new(
            "Concert Hall".to_string(),
            "a room with a stage and a lot of seats".to_string(),
            vec![
                Door::new(
                    "Stage".to_string(),
                    "a room with a stage".to_string(),
                    true,
                    Key::new("actor's pass".to_string()),
                    None,
                    "Stage".to_string(),
                    Some(RoomRequirements {
                        health: 300,
                        attack: 0,
                    }),
                ),
                Door::new(
                    "Backstage".to_string(),
                    "a room with a lot of props".to_string(),
                    false,
                    Key::new("".to_string()),
                    None,
                    "Backstage".to_string(),
                    None,
                ),
            ],
            vec![],
            vec![],
        ),
    );

    rooms.insert(
        "Stage".to_string(),
        Room::new(
            "Stage".to_string(),
            "a room with a stage".to_string(),
            vec![],
            vec![
                Item::new("microphone".to_string(), "a microphone".to_string(), 0, 10),
                Item::new("XLR cable".to_string(), "a mic cable".to_string(), 0, 20),
            ],
            vec![],
        ),
    );

    rooms.insert(
        "Backstage".to_string(),
        Room::new(
            "Backstage".to_string(),
            "a room with a lot of props".to_string(),
            vec![
                Door::new(
                    "Dressing Room".to_string(),
                    "a room with a lot of costumes".to_string(),
                    true,
                    Key::new("staff pass".to_string()),
                    Some(Player::new(
                        "Stressed actor".to_string(),
                        empty_map.clone(),
                        vec![
                            Item::new("costume".to_string(), "a costume".to_string(), 20, 0),
                            Item::new("makeup".to_string(), "a makeup kit".to_string(), 0, 5),
                        ],
                        vec![],
                        100,
                        20,
                        vec![],
                        empty_room.clone(),
                        "".to_string(),
                    )),
                    "Dressing Room".to_string(),
                    None,
                ),
                Door::new(
                    "Tech Room".to_string(),
                    "a room with a lot of tech".to_string(),
                    false,
                    Key::new("".to_string()),
                    None,
                    "Tech Room".to_string(),
                    None,
                ),
            ],
            vec![
                Item::new("guitar".to_string(), "a guitar".to_string(), 0, 30),
                Item::new(
                    "drumsticks".to_string(),
                    "a pair of drumsticks".to_string(),
                    0,
                    40,
                ),
            ],
            vec![Key::new("staff pass".to_string())],
        ),
    );

    rooms.insert(
        "Dressing Room".to_string(),
        Room::new(
            "Dressing Room".to_string(),
            "a room with a lot of costumes".to_string(),
            vec![],
            vec![
                Item::new("costume".to_string(), "a costume".to_string(), 20, 0),
                Item::new("makeup".to_string(), "a makeup kit".to_string(), 0, 5),
            ],
            vec![Key::new("actor's pass".to_string())],
        ),
    );

    rooms.insert(
        "Tech Room".to_string(),
        Room::new(
            "Tech Room".to_string(),
            "a room with a lot of tech".to_string(),
            vec![],
            vec![
                Item::new("laptop".to_string(), "a laptop".to_string(), 0, 30),
                Item::new(
                    "headphones".to_string(),
                    "a pair of headphones".to_string(),
                    20,
                    0,
                ),
                Item::new(
                    "sound board".to_string(),
                    "a sound board".to_string(),
                    0,
                    50,
                ),
            ],
            vec![],
        ),
    );

    let mut player = Player {
        name: "Player".to_string(),
        map: rooms.clone(),
        items_held: vec![],
        keys_held: vec![],
        health: 100,
        attack: 10,
        battles: vec![],
        current_room: rooms
            .clone()
            .entry("Entrance Hall".to_string())
            .or_insert(empty_room.clone())
            .clone(),
        game_name: "".to_string(),
    };

    out!("Would you like to load a savegame? (y/n)", "yellow");

    if term.read_line().unwrap().trim() == "y" {
        out!("What is the name of the savegame?", "yellow");

        let mut savegame_name = term.read_line().unwrap();

        savegame_name = savegame_name.trim().to_string();

        player = serde_json::from_reader(
            File::open(format!("savegames/{}.json", savegame_name)).unwrap_or_else(|e| {
                eprintln!("Could not open savegame file: {}", e);
                std::process::exit(1);
            }),
        )
        .unwrap();

        write(
            format!("You are in the {}", player.current_room.name).as_str(),
            "blue",
        );
    } else {
        out!("What is your name?", "yellow");

        player.name = term.read_line().unwrap().trim().to_string();

        out!("What is the name of this game?", "yellow");

        player.game_name = term.read_line().unwrap().trim().to_string();

        out!(format!(
            "Welcome to the game, {}! You are in the {}",
            player.name, player.current_room.name
        )
        .as_str());
    }

    loop {
        let input = term.read_line().unwrap();

        let input = input.trim();

        let mut commands: Vec<&str> = input.splitn(2, " ").collect();

        if std::env::args()
            .collect::<Vec<_>>()
            .contains(&"-d".to_string())
            || std::env::args()
                .collect::<Vec<_>>()
                .contains(&"--debug".to_string())
        {
            if commands[0] == "give" {
                let item = Item::new(
                    commands[1].to_string(),
                    "a debug item".to_string(),
                    100,
                    100,
                );

                player.items_held.push(item);

                commands[0] = "blank";
            } else if commands[0] == "givekey" {
                let key = Key::new(commands[1].to_string());

                player.keys_held.push(key);

                commands[0] = "blank";
            }
        }

        match commands[0] {
            "blank" => {
                continue;
            }

            "quit" => {
                write("Goodbye!", "green");
                player.save();
                break;
            }

            "help" => {
                out!(
                    format!(
                        "
{}
- look: print the description of the room
- go [room]: move to another room (checks if you have the key)
- take [item]: take an item
- takekey [key]: take a key
- search: search the room for items, keys, and doors
- save: save the game
- battles: print the battles you've fought
- inventory: print the items and keys you have
- use: use an item now, but for only half the effect",
                        style("Commands:").bold()
                    )
                    .as_str(),
                    "green"
                );

                if std::env::args()
                    .collect::<Vec<String>>()
                    .contains(&"--debug".to_string())
                    || std::env::args()
                        .collect::<Vec<String>>()
                        .contains(&"-d".to_string())
                {
                    out!(
                        format!(
                            "{}
- debug: print the current room's data
- use [item]: use an item",
                            style("Debug Commands:").bold()
                        )
                        .as_str(),
                        "green"
                    );
                }

                out!(
                    format!("If you can't figure out what to do, try using `search`!\nStuck for a key? Make sure to look around!",)
                        .as_str(),
                    "cyan"
                )
            }

            "look" => {
                out!(player.current_room.description.as_str());
            }

            "go" => {
                if commands.len() < 2 {
                    write("Go where?", "red");
                } else {
                    player.move_through_door(commands[1].to_string());
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
                            player.use_item(Item::new(
                                item.name.clone(),
                                item.description.clone(),
                                item.health / 2 as i32,
                                item.attack / 2 as i32,
                            ));
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
                        format!(
                            "You see \"{}\" (item) that buffs {} health and {} attack",
                            item.name, item.health, item.attack
                        )
                        .as_str(),
                        "green",
                    );
                }

                for key in &player.current_room.keys {
                    write(format!("You see \"{}\" (key)", key.name).as_str(), "green");
                }

                for door in &player.current_room.doors {
                    write(
                        format!(
                            "You see \"{}\" ({}, {} door)",
                            door.name,
                            if door.locked && !player.keys_held.contains(&door.key) {
                                "locked"
                            } else {
                                "unlocked"
                            },
                            if door.enemy.is_some()
                                && !player
                                    .battles
                                    .iter()
                                    .any(|b| b.enemy_name == door.enemy.clone().unwrap().name)
                            {
                                "guarded"
                            } else {
                                "unguarded"
                            }
                        )
                        .as_str(),
                        "green",
                    );
                }
            }

            "save" => {
                player.save();
            }

            "battles" => {
                for battle in &player.battles {
                    write(
                        format!(
                            "You fought \"{}\" and {}. You had {} health and they had {} health.",
                            battle.enemy_name,
                            if battle.winner { "won" } else { "lost" },
                            battle.player_health,
                            battle.enemy_health
                        )
                        .as_str(),
                        "green",
                    );
                }
            }

            "inventory" => {
                for item in &player.items_held {
                    write(
                        format!(
                            "You have the \"{}\" (item) that buffs {} health and {} attack",
                            item.name, item.health, item.attack
                        )
                        .as_str(),
                        "green",
                    );
                }

                for key in &player.keys_held {
                    write(format!("You have the {} (key)", key.name).as_str(), "green");
                }

                write(
                    format!(
                        "You have {} health and {} attack",
                        player.health, player.attack
                    )
                    .as_str(),
                    "green",
                );

                if player.keys_held.len() == 0 && player.items_held.len() == 0 {
                    out!("You have nothing in your inventory.", "red");
                }
            }

            _ => {
                write(
                    "I don't understand that command. Try using `help` if you need it!",
                    "red",
                );
            }
        }
    }
}
