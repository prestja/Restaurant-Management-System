///@description Draw HUD

if (game_state == GameState.PlayerTurn) {
	if (player_turn == 0) {
		//draw_set_color(c_red);
		draw_text(8, 8, "X's Turn");
	}
	else if (player_turn == 1) {
		//draw_set_color(c_blue);
		draw_text(8, 8, "O's Turn");
	}
}
else if (game_state == GameState.VictoryOne) {
	//draw_set_color(c_red);
	draw_text(8, 8, "X Wins!\nTap any space\nto restart");
}
else if (game_state == GameState.VictoryTwo) {
	//draw_set_color(c_blue);
	draw_text(8, 8, "O Wins!\nTap any space\nto restart");
}
else if (game_state == GameState.Draw) {
	//draw_set_color(c_white);
	draw_text(8, 8, "Draw...\nTap any space\nto restart");
}