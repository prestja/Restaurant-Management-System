///@function scrPieceMove(piece, dest_i, dest_j)
///@description Tell the piece to move to the given destination coordinates
///@param piece
///@param dest_i
///@param dest_j
var piece = argument0, dest_i = argument1, dest_j = argument2;
if (instance_exists(piece)) {
	piece.goto_grid_i = dest_i;
	piece.goto_grid_j = dest_j;
}