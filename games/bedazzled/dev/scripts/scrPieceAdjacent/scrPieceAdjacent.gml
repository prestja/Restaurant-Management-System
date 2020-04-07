///@function scrPieceAdjacent(piece1, piece2)
///@description Return true if the two pieces are orthogonally next to each other
///@param piece1
///@param piece2
var piece1 = argument0, piece2 = argument1;
if (!instance_exists(piece1) || !instance_exists(piece2)) { return false; }
else {
	var i1 = piece1.grid_i, j1 = piece1.grid_j, i2 = piece2.grid_i, j2 = piece2.grid_j;
	return ((abs(i2 - i1) == 1 && j1 == j2) ||
			(abs(j2 - j1) == 1 && i1 == i2));
}