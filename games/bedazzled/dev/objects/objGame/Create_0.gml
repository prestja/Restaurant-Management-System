///@description Initialize game state
global.board_x = 403;
global.board_y = 15;
global.game_grid[7, 7] = noone;
global.destroy_grid[7, 7] = false;
global.piece_selected = noone
global.game_state = GameState.Player;
game_score = 0;

while(true) {
	//Generate board
	for(var i = 0; i < 8; ++i) {
		for(var j = 0; j < 8; ++j) {
			global.destroy_grid[i, j] = false;
			global.game_grid[i, j] = instance_create_v(global.board_x + (j*64), global.board_y + (i*64), "InstanceActors", objJewel, irandom(6), i, j);
		}
	}
	
	//Eliminate starting chains
	for(var i = 0; i < 8; ++i) {
		for(var j = 0; j < 8; ++j) {
			var _type = global.game_grid[i, j].image_index;
			var _chain = scrGetChain(j, i, _type);
			while (ds_list_size(_chain) >= 3) {
				_type = wrap(_type + 1, 0, 6);
				global.game_grid[i, j].image_index = _type;
				ds_list_destroy(_chain);
				_chain = scrGetChain(j, i, _type);
			}
		}
	}
	
	//Check if any moves are possible
	if (scrGetRemainingMoves() > 0) { break; }
	else { instance_destroy(objJewel); }
}