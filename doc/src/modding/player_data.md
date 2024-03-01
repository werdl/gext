# Player Data
> How to customise items held by user, user stats etc
- all player data is stored in the game save file, a serialized `Player` instance
- therefore, by editing it after saving and before loading, you can customise the player's stats, items, and keys
## player stats
- the player has 3 stats: health, attack, and defense
- these are all integers
- to edit, change the `health`, `attack`, and `defense` fields in the `Player` struct (at top level of the json file)
## items
- items are stored in the `items_held` field of the `Player` struct
- to add an item, add a new item to the `items_held` array
- to remove an item, remove the item from the `items_held` array
## keys
- keys are stored in the `keys_held` field of the `Player` struct
- these can be added and removed in the same way as items
## battles
- battles are stored in the `battles` field of the `Player` struct
- this is an array of `Battle` structs
- to add a battle, add a new battle to the `battles` array
- to remove a battle, remove the battle from the `battles` array
## current room
- the current room is stored in the `current_room` field of the `Player` struct
- this is a `Room` struct
## game name
- the game name is stored in the `game_name` field of the `Player` struct
- exactly why you would want to change this is beyond me, but you can, I guess