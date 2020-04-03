///@description Selection
if (global.game_state == GameState.Player) {
	if (device_mouse_check_button_pressed(0, mb_left) && scrMouseOnMe(0)) {
		//Select first piece
		if (global.piece_selected == noone) {
			global.piece_selected = self;
		}
		else {
			//Check if this piece is next to the first selected one
			if (abs(global.piece_selected.grid_y - grid_y) <= 1 && abs(global.piece_selected.grid_x - grid_x) <= 1) {
				//Check if moving the piece would create a chain
				if (scrAttemptPieceSwap(global.piece_selected.grid_x, global.piece_selected.grid_y, scrGetPieceDir(global.piece_selected, self)))
				|| (scrAttemptPieceSwap(grid_x, grid_y, scrGetPieceDir(self, global.piece_selected))) {
					goto_grid_y = global.piece_selected.grid_y;
					goto_grid_x = global.piece_selected.grid_x;
					global.piece_selected.goto_grid_y = grid_y;
					global.piece_selected.goto_grid_x = grid_x;
					global.game_state = GameState.Swapping;
				}
			}
			global.piece_selected = noone;
		}
	}
	
	if (device_mouse_check_button_pressed(0, mb_right) && scrMouseOnMe(0)) {
		var _chain = scrGetChain(grid_x, grid_y, image_index);
		show_message(ds_list_print(_chain));
	}
}