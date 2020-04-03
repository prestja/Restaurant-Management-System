///@description Swap pieces
if (global.game_state == GameState.Player) {
	if (global.piece_one != noone && global.piece_two != noone) {
		//Check if pieces are adjacent & swapping would make a chain
		if (scrPieceAdjacent(global.piece_one, global.piece_two) &&
			scrAttemptSwap(global.piece_one, global.piece_two)) {
			//Tell pieces to swap
			var _i1 = global.piece_one.grid_i, _j1 = global.piece_one.grid_j;
			var _i2 = global.piece_two.grid_i, _j2 = global.piece_two.grid_j;
			scrPieceMove(global.piece_one, _i2, _j2);
			scrPieceMove(global.piece_two, _i1, _j1);
			global.game_state = GameState.Swapping;
		}
		global.piece_one = noone;
		global.piece_two = noone;
	}
}
else if (global.game_state == GameState.Swapping) {
	//Wait for pieces to finish moving
	var _pieces_finished = true;
	for(var k = 0; k < instance_number(objJewel); ++k) {
		var _obj = instance_find(objJewel, k);
		if (!_obj.settled) { 
			_pieces_finished = false; 
			break;
		}
	}
	if (_pieces_finished) {
		//Increase the combo level
		global.combo_level++;
		
		//Mark all chains for deletion
		game_score += scrMarkChains();
		
		//Delete all chains
		var _pieces_destroyed = false;
		var _gaps = array_create(8, 0);
		
		for(var i = 7; i >= 0; --i) {
			for(var j = 0; j < 8; ++j) {
				//Destroy the piece
				if (global.destroy_grid[i, j]) {
					_pieces_destroyed = true;
					instance_destroy(global.game_grid[i, j]);
					global.game_grid[i, j] = noone;
					//Count the number of holes in this column
					_gaps[j]++;
					//Tell each piece above this one to move down
					for(var k = i - 1; k >= 0; --k) { scrPieceFall(global.game_grid[k, j]); }
				}
				//Clear destruction grid
				global.destroy_grid[i, j] = false;
			}
		}
		
		//Fill in spaces
		for(var j = 0; j < 8; ++j) {
			for(var k = 0; k < _gaps[j]; ++k) {
				var _obj = instance_create_v(global.board_x + (j*64), global.board_y - ((k + 1)*64), "InstanceActors", objJewel, irandom(6), -1, j);
				scrPieceMove(_obj, _gaps[j] - k - 1, j);
			}
		}
			
		//Return control to player
		if (!_pieces_destroyed) { 
			global.combo_level = -1;
			//Check if there are any moves left
			if (scrGetRemainingMoves() > 0) { global.game_state = GameState.Player; }
			else { global.game_state = GameState.GameOver; }
		}
	}
}
else if (global.game_state == GameState.GameOver) {
	if (device_mouse_check_button_pressed(0, mb_left) && device_mouse_x(0) < 320  && device_mouse_y(0) < 72) {
		global.game_state = GameState.Player;
		game_score = 0;
		scrEmptyGameGrid();
		scrEmptyDestroyGrid();
		scrGenerateBoard();
	}
}