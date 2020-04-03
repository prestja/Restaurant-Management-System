///@description Selection
if (global.game_state == GameState.Player) {
	if (device_mouse_check_button_pressed(0, mb_left) && scrMouseOnMe(0)) {
		//Select first piece
		if (global.piece_one == noone) {
			global.piece_one = self;
		}
		else {
			global.piece_two = self;
		}
	}
	
	//Debug
	if (device_mouse_check_button_pressed(0, mb_right) && scrMouseOnMe(0)) {
		var _chain = scrGetChain(grid_i, grid_j);
		show_message(ds_list_print(_chain));
	}
}