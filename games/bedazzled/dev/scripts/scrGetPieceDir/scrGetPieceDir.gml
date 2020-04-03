///@function scrGetPieceDir(piece1, piece2)
///@description Get the direction piece 2 is in relative to piece 1
///@param piece1
///@param piece2
var piece1 = argument0, piece2 = argument1;
	 if (piece2.grid_x < piece1.grid_x) { return Dir.Left; }
else if (piece2.grid_x > piece1.grid_x) { return Dir.Right; }
else if (piece2.grid_y < piece1.grid_y) { return Dir.Up; }
else if (piece2.grid_y > piece1.grid_y) { return Dir.Down; }