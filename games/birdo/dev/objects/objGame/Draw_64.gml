///@description Draw score
if (game_state == GameState.None) {
	draw_text(0, 0, "Tap anywhere to begin");
}
else{
	draw_text(0, 0, global.game_score);
	if (game_state == GameState.GameOver) {
		draw_rectangle(0, 24, 160, 72, true);
		draw_text(0, 28, "Tap here to\nplay again");
	}
}