///@description Swap pieces
if (global.game_state == GameState.Swapping) {
	//Wait for pieces to finish moving
	global.pieces_finished = true;
	for(var k = 0; k < instance_number(objJewel); ++k) {
		with(instance_find(objJewel, k)) {
			if (cpt_running(myPath)) { global.pieces_finished = false; }
		}
	}
	if (global.pieces_finished) {
		//Mark all chains for deletion
		game_score += scrMarkChains();
		
		//Delete all chains
		var _pieces_destroyed = false;
		var _gaps = array_create(8, -1);
		for(var i = 7; i >= 0; --i) {
			for(var j = 0; j < 8; ++j) {
				if (global.destroy_grid[i, j]) { 
					_pieces_destroyed = true;
					instance_destroy(global.game_grid[i, j]);
					_gaps[j]++;
					for(var k = i - 1; k >= 0; --k) {
						//show_message("Dropping i="+string(k)+", j="+string(j))
						if (instance_exists(global.game_grid[k, j])) {
							global.game_grid[k, j].goto_grid_y++;
						}
					}
				}
				global.destroy_grid[i, j] = false;
			}
		}
		
		//Fill in spaces
		for(var j = 0; j < 8; ++j) {
			//Create enough pieces to fill the spaces
			for(var k = _gaps[j]; k >= 0; --k) {
				var _y = (_gaps[j] - k + 1)*64;
				var _obj = instance_create_v(global.board_x + (j*64), global.board_y - _y, "InstanceActors", objJewel, irandom(6), k, j);
				_obj.grid_y = -1;
				global.game_grid[k, j] = _obj;
			}
		}
			
		//Return to player if pieces are done falling
		if (!_pieces_destroyed) { global.game_state = GameState.Player; }
	}
}