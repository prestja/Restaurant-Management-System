///@function scrGenerateBoard()
///@description Populate the game board with a valid starting table
while(true) {
	//Generate board
	for(var i = 0; i < 8; ++i) {
		for(var j = 0; j < 8; ++j) {
			global.destroy_grid[i, j] = false;
			//Create piece
			var _val = irandom(6);
			global.game_grid[i, j] = instance_create_v(global.board_x + (j*64), global.board_y + (i*64), "InstanceActors", objJewel, _val, i, j);
			//Choose a starting value that wont make a chain
			var _chain = scrGetChain(i, j);
			while(_chain != undefined) {
				//Choose a new value & reevaluate the chain
				_val = wrap(_val + 1, 0, 6);
				global.game_grid[i, j].image_index = _val;
				ds_list_destroy(_chain);
				_chain = scrGetChain(i, j);
			}
		}
	}
	
	//Check if any moves are possible
	if (scrGetRemainingMoves() > 0) { break; }
	else { scrEmptyGameGrid(); }
}