///@description Draw score
if (game_state == GameState.None) {
	draw_text(0, 0, "Tap anywhere to begin");
}
else{
	draw_text(0, 0, global.game_score);
	if (game_state == GameState.GameOver) {
		draw_set_halign(fa_center);
		draw_set_valign(fa_bottom);
		
		var _dw = display_get_gui_width()/2, _dh = display_get_gui_height();
		draw_rectangle(_dw - (font_w*7), _dh - (font_h*2), _dw + (font_w*7), _dh, true);
		draw_text(_dw, _dh, "Tap here to\nplay again");
		
		draw_set_halign(fa_left);
		draw_set_valign(fa_top);
	}
}