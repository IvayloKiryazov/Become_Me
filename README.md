# Become_Me
Become me is a turn-based strategy game with the goal of taking over the map through adapting, picking the right moment, and plotting with and against your fellow players.

```

Leader: You start with 25 square tiles. Each turn you can do combination of 5 (they can be repeating) actions. You have 60 seconds to execute your series of actions. Each action re-allocates a tile of your ownership to do that said action. The objective is to defeat the enemies on the board. You don't need to take over all tiles. A player loses the game when they lose all their population. A Leader is nothing without his people.

```

Actions per turn:

Move: Either re-destribute people on your tiles, or attack a tile. Attacking tiles has three outcomes: You destroy all of the populace and lose some units. You destroy some of the populace and gain some. You destroy some of the populace and lose all your units.

Search: This is used to find an artifact that will increase your over-all stats permanently. On the tile it is shown below the population field as a 0 or 1. Can search on a tile once per game. Every 25 tiles you get an item for sure. Untill then it's a dice roll. You can have up to 5 items.

Create: This is used so you can either craft an item that can boost your stats for a turn. On the tile it is shown below the searched field as a 0 or 1. You can search on a tile once per game unless you overtake an enemy tile. You can hold up to 20 items and you can use any amount in a turn.

Populate: Next turn your population will be increased. There is population cap based on the tiles you have. It is show in the left up corner of the tile.

```

Leader stats: // not fully implemented.

Influence - Increases amount of people that will listen to you when taking an action.

Science - Has a higher chance to drop an item in the dice-roll

Fertility - Increases population generated by populate.

Diplomacy - Increases chance of people joining your cause when attacking.

Mastery - Can increase inventory space.

```

Items:

Permanent: Increase permanent stats.

Expandable: Can increase stats for a turn, or be used for attack for a turn.

```

Tiles and map:

256 squares. Three types of tiles - yours, enemy, and neutral. There are 4 starting points at the edges of the map.

```

Server: Different rooms. You need 4 people to start a game A single player can create a room.