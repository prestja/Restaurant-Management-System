///@function scrEmptyGameGrid()
///@description Clear all gems from the game board & fill it with null pointers
for(var i = 0; i < 8; ++i) {
	for(var j = 0; j < 8; ++j) {
		global.game_grid[i, j] = noone;
	}
}
instance_destroy(objJewel);