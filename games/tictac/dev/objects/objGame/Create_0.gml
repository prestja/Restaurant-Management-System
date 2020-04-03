///@description Initialize game state
player_turn = 0;
game_state = GameState.PlayerTurn;
board_x = 232;
board_y = 23;

//Create spaces
spaces = array_create(9);
for(var i=0; i<9; i++) {
	var _xx = (i mod 3)*sprite_get_width(sprSpace);
	var _yy = (i div 3)*sprite_get_height(sprSpace);
	spaces[i] = instance_create_v(board_x + _xx, board_y + _yy, layer, objSpace, i);
}