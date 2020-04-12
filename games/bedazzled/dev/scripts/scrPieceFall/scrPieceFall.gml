///@function scrPieceMove(piece)
///@description Tell the piece to move down one space
///@param piece
var piece = argument0;
if (instance_exists(piece)) { piece.goto_grid_i++; }