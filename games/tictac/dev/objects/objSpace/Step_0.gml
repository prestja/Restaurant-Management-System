///@description Send message to game state
if (device_mouse_check_button_pressed(0, mb_left) && scrMouseOnMe(0)) {
	global.space_selected = self;
	with (objGame) { event_perform(ev_other, ev_user0); }
}