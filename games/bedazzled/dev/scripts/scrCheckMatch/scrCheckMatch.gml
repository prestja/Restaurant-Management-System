///@function scrCheckMatch(x, y, type, dir, list)
///@description Check for matches for the given type at the given position
///@param x
///@param y
///@param type
///@param dir
///@param list
var xx = argument0, yy = argument1, type = argument2, dir = argument3, list = argument4;
//Add self to the list
ds_list_add_exclusive(list, scrCoordEncode(xx, yy));
//Search in a direction
switch dir {
	case Dir.Left: 
		var _x = xx;
		if (_x > 0) {
			for(_x = xx - 1; _x >= 0; --_x) {
				if (global.game_grid[yy, _x].image_index == type) { ds_list_add_exclusive(list, scrCoordEncode(_x, yy)); }
				else { break; }
			}
			++_x;
		}
		if (yy > 0) {
			if (global.game_grid[yy - 1, _x].image_index == type) { scrCheckMatch(_x, yy - 1, type, Dir.Up, list); }
		}
		if (yy < 7) {
			if (global.game_grid[yy + 1, _x].image_index == type) { scrCheckMatch(_x, yy + 1, type, Dir.Down, list); }
		}
	break;
	case Dir.Right: 
		var _x = xx;
		if (_x < 7) {
			for(_x = xx + 1; _x <= 7; ++_x) {
				if (global.game_grid[yy, _x].image_index == type) { ds_list_add_exclusive(list, scrCoordEncode(_x, yy)); }
				else { break; }
			}
			--_x;
		}
		if (yy > 0) {
			if (global.game_grid[yy - 1, _x].image_index == type) { scrCheckMatch(_x, yy - 1, type, Dir.Up, list); }
		}
		if (yy < 7) {
			if (global.game_grid[yy + 1, _x].image_index == type) { scrCheckMatch(_x, yy + 1, type, Dir.Down, list); }
		}
	break;
	case Dir.Up: 
		var _y = yy;
		if (_y > 0){
			for(_y = yy - 1; _y >= 0; --_y) {
				if (global.game_grid[_y, xx].image_index == type) { ds_list_add_exclusive(list, scrCoordEncode(xx, _y)); }
				else { break; }
			}
			++_y;
		}
		if (xx > 0) {
			if (global.game_grid[_y, xx - 1].image_index == type) { scrCheckMatch(xx - 1, _y, type, Dir.Left, list); }
		}
		if (xx < 7) {
			if (global.game_grid[_y, xx + 1].image_index == type) { scrCheckMatch(xx + 1, _y, type, Dir.Right, list); }
		}
	break;
	case Dir.Down: 
		var _y = yy;
		if (_y < 7) {
			for(_y = yy + 1; _y <= 7; ++_y) {
				if (global.game_grid[_y, xx].image_index == type) { ds_list_add_exclusive(list, scrCoordEncode(xx, _y)); }
				else { break; }
			}
			--_y;
		}
		if (xx > 0) {
			if (global.game_grid[_y, xx - 1].image_index == type) { scrCheckMatch(xx - 1, _y, type, Dir.Left, list); }
		}
		if (xx < 7) {
			if (global.game_grid[_y, xx + 1].image_index == type) { scrCheckMatch(xx + 1, _y, type, Dir.Right, list); }
		}
	break;
}