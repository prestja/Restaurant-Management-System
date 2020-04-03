///@function scrPieceJump(piece, dest_i, dest_j)
///@description Instantly jump a piece to the given destination coordinates
///@param piece
///@param dest_i
///@param dest_j
var piece = argument0, dest_i = argument1, dest_j = argument2;
if (instance_exists(piece)) {
	piece.grid_i = dest_i;
	piece.grid_j = dest_j;
	piece.goto_grid_i = dest_i;
	piece.goto_grid_j = dest_j;
	piece.x = (global.board_x + (dest_j*64));
	piece.y = (global.board_y + (dest_i*64));
	global.game_grid[dest_i, dest_j] = piece;
}