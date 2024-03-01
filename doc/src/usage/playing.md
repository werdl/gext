# Playing
- this page contains some tips and tricks for playing the game
## player stats
- the player has 3 stats: health, attack, and defense
- health is the amount of damage the player can take before dying
- attack is the amount of damage the player can deal to an enemy
- defense is the amount of damage the player can block from an enemy
## doors
- doors are used to move between rooms
- some doors are guarded by an enemy, where you need to defeat the enemy to pass
- other doors are locked, where you need to find a key to pass
## items
- items are used to buff the player
- items can be found in rooms
- items can be taken with the `take` command
- they buff one or more of the player's stats
## enemies
- enemies are found behind doors
- enemies have stats, just like an ordinary player (in the game files, they are just an instance of the `Player` struct)
- enemies can be fought when attempting to pass a door (but you need the key first)
- by using the `search` command, you can check if a door is guarded by an enemy, or if it is locked
- once you attempt to pass a door, you can't go back (if you die, all your stats are reset, but any items you used in the battle are not returned to you)
## saving
- the game can be saved with the `save` command
- at the start of each game, you are asked if you want to load a save file
- if you choose to load a save file, you are asked for the name of the save file
- game files usually have the `.save.json` suffix