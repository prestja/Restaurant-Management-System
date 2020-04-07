///@description Draw HUD
draw_set_color(c_white);
draw_text(6, 6, game_score);
if (global.game_state == GameState.GameOver) {	
	draw_text(6, 28, "Game Over!\nPress here\nto restart");
	draw_rectangle(0, 24, 128, 94, true);
}