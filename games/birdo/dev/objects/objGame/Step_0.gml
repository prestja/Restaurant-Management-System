///@description Start Game
var _dw = display_get_gui_width()/2, _dh = display_get_gui_height();
if (game_state == GameState.None && device_mouse_check_button_pressed(0, mb_left)) ||
   (game_state == GameState.GameOver && device_mouse_check_button_pressed(0, mb_left) && scrMouseInGuiRegion(_dw - (font_w*7), _dh - (font_h*2), _dw + (font_w*7), _dh)) {
	instance_destroy(objWall);
	instance_destroy(objPoints);
	instance_destroy(objBirdo);
	global.game_score = 0;
	instance_create_layer(64, 128, "InstanceActors", objBirdo);
	wall_timer = 120;
	alarm[0] = wall_timer;
	game_state = GameState.Playing;
}