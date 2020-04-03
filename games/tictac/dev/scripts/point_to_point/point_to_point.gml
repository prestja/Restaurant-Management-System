///@function point_to_point(x1, y1, x2, y2)
///@description Returns the distance from one (x1, y1) to (x2, y2)
///@param x1
///@param x2
///@param y1
///@param y2
var x1 = argument0, y1 = argument1, x2 = argument2, y2 = argument3;
return sqrt(((x2 - x1) * (x2 - x1)) + ((y2 - y1) * (y2 - y1)));