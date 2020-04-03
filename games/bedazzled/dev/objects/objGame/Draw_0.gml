///@description Draw HUD
draw_set_color(c_white);
draw_text(0, 0, game_score);
if (global.game_state == GameState.GameOver) {	
	draw_text(0, 28, "Game Over!\nPress here to restart");
	draw_rectangle(0, 24, 320, 72, true);
}