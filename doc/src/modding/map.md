# Map
- to modify the map, you can customise the map save file
- this is usually suffixed with `.map.json`
- the map save file is a json file, with the following structure:
## examples
- here are some example rooms, from the default save
- note that all rooms that they reference (ie "East Dungeon Cell" etc.) are not included in this snippet, and can be found at (default.map.json)[https://github.com/werdl/gext/blob/main/default.map.json]
```json
{
    "East Dungeon": {
        "name": "East Dungeon",
        "description": "a dark dungeon",
        "doors": [
            {
                "name": "East Dungeon Cell",
                "description": "a dark dungeon cell",
                "locked": false,
                "key": {
                    "name": ""
                },
                "enemy": null,
                "associated_room_name": "Dungeon",
                "requirements": null
            }
        ],
        "items": [
            {
                "name": "stick",
                "description": "a stick",
                "health": 0,
                "attack": 1,
                "defense": 0
            }
        ],
        "keys": []
    }
}
```
- this one describes a simple room, that contains a stick, and a door to a dungeon cell
```json
"Armory": {
        "name": "Armory",
        "description": "a room with a lot of weapons",
        "doors": [
            {
                "name": "Trophy Cupboard",
                "description": "a room with a lot of trophies",
                "locked": true,
                "key": {
                    "name": "trophy cupboard"
                },
                "enemy": {
                    "class": {
                        "name": "Default",
                        "description": "Looks like somebody didn't choose a class...",
                        "health": 100,
                        "attack": 10,
                        "defense": 10,
                        "starting_items": [],
                        "starting_keys": [],
                        "won_battle_attack_bonus": 5,
                        "won_battle_defense_bonus": 5,
                        "won_battle_health_bonus": 5
                    },
                    "name": "Trophy Keeper",
                    "map": {},
                    "items_held": [],
                    "keys_held": [],
                    "health": 100,
                    "attack": 20,
                    "defense": 10,
                    "battles": [],
                    "current_room": {
                        "name": "Empty Room",
                        "description": "a room with nothing in it",
                        "doors": [],
                        "items": [],
                        "keys": []
                    },
                    "game_name": ""
                },
                "associated_room_name": "Trophy Cupboard",
                "requirements": null
            }
        ],
        "items": [
            {
                "name": "shield",
                "description": "a shield",
                "health": 20,
                "attack": 0,
                "defense": 0
            },
            {
                "name": "axe",
                "description": "a sharp axe",
                "health": 0,
                "attack": 20,
                "defense": 0
            }
        ],
        "keys": [
            {
                "name": "trophy cupboard"
            },
            {
                "name": "pantry"
            }
        ]
    }
```
- this one describes a room with a shield, an axe, and a door to a trophy cupboard
- the trophy cupboard is locked, and contains an enemy
- the enemy is a `Trophy Keeper`, with 100 health, 20 attack, and 10 defense
