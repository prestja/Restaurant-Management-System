///@description Start Game
if (game_state == GameState.None && device_mouse_check_button_pressed(0, mb_left)) ||
   (game_state == GameState.GameOver && device_mouse_check_button_pressed(0, mb_left) && device_mouse_x_to_gui(0) < 160  && device_mouse_y_to_gui(0) < 72){
	instance_destroy(objWall);
	instance_destroy(objPoints);
	instance_destroy(objBirdo);
	global.game_score = 0;
	instance_create_layer(64, 128, "InstanceActors", objBirdo);
	wall_timer = 120;
	alarm[0] = wall_timer;
	game_state = GameState.Playing;
}