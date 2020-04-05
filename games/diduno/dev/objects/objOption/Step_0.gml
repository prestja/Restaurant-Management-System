///@description Register touch
if (global.gameState == GameState.WaitingResponse) {
	if (device_mouse_check_button_pressed(0, mb_left) && scrMouseOnMe(0)) {
		global.optionChosen = myChoice;
	}
}