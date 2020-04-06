///@description Selection
if (global.game_state == GameState.Player) {
	if (device_mouse_check_button_pressed(0, mb_left) && scrMouseOnMe(0)) {
		if (global.piece_one == noone) { 
			global.piece_one = self; 
			drag_detect_flag = true;
		}
	}
	if (global.piece_one != noone && device_mouse_check_button(0, mb_left) && 
		scrMouseInRegion(0, bbox_left + drag_detect_buffer, bbox_top + drag_detect_buffer, 
							bbox_right - drag_detect_buffer, bbox_bottom - drag_detect_buffer)) {
		if (!drag_detect_flag && global.piece_two == noone) { global.piece_two = self; }
	}
	else { drag_detect_flag = false; }
}