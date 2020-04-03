///@description Initialize game state
global.board_x = 403;
global.board_y = 15;
global.game_grid[7, 7] = noone;
global.destroy_grid[7, 7] = false;
global.piece_one = noone
global.piece_two = noone;
global.game_state = GameState.Player;
global.combo_level = -1;
game_score = 0;

scrEmptyGameGrid();
scrEmptyDestroyGrid();
scrGenerateBoard();