///@function scrGetChain(x, y, type)
///@description Find the full chain of spaces of the given type connected to the given point
///@param x
///@param y
///@param type
var xx = argument0, yy = argument1, type = argument2;
//Create empty list
var _chain = ds_list_create();
//Only search if you are in a valid position
if (xx >= 0 && xx <= 7 && yy >= 0 && yy <= 7) {
	scrCheckMatch(xx, yy, type, Dir.Right, _chain);
	scrCheckMatch(xx, yy, type, Dir.Left, _chain);
	scrCheckMatch(xx, yy, type, Dir.Up, _chain);
	scrCheckMatch(xx, yy, type, Dir.Down, _chain);
}
return _chain;