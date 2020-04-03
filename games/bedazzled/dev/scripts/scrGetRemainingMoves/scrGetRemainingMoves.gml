///@function scrGetRemainingMoves()
///@description Count the number of possible remaining moves
var _num_moves = 0;
for(var i = 0; i < 8; ++i) {
	for(var j = 0; j < 8; ++j) {	
		//Try to swap the piece in each direction
		if (j > 0) {
			var _piece_one = global.game_grid[i, j];
			var _piece_two = global.game_grid[i, j - 1];
			if (scrAttemptSwap(_piece_one, _piece_two)) { _num_moves++; }
		}
		if (j < 7) {
			_piece_one = global.game_grid[i, j];
			_piece_two = global.game_grid[i, j + 1];
			if (scrAttemptSwap(_piece_one, _piece_two)) { _num_moves++; }
		}
		if (i > 0) {
			_piece_one = global.game_grid[i, j];
			_piece_two = global.game_grid[i - 1, j];
			if (scrAttemptSwap(_piece_one, _piece_two)) { _num_moves++; }
		}
		if (i < 7) {
			_piece_one = global.game_grid[i, j];
			_piece_two = global.game_grid[i + 1, j];
			if (scrAttemptSwap(_piece_one, _piece_two)) { _num_moves++; }
		}
	}
}
return _num_moves;