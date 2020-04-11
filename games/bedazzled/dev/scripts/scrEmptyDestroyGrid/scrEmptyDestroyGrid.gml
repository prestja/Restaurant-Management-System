///@function scrEmptyDestroyGrid()
///@description Set all values in the destroy grid to false
for(var i = 0; i < 8; ++i) {
	for(var j = 0; j < 8; ++j) {
		global.destroy_grid[i, j] = false;
	}
}