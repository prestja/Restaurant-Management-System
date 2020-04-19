///@description Draw HUD
draw_set_color(c_white);
draw_text(8, 8, game_score);
if (global.game_state == GameState.GameOver) {	
	draw_text(8, 28, "Game Over!\nPress here\nto restart");
	draw_rectangle(0, 24, 128, 94, true);
}