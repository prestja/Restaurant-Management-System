///@function scrAttemptPieceSwap(x, y, dir)
///@description Return true if moving the given piece in the given direction would result in a match.
///@param x
///@param y
///@param dir
var xx = argument0, yy = argument1, dir = argument2;

//Check for border
if (xx == 0 && dir == Dir.Left)
|| (xx == 7 && dir == Dir.Right)
|| (yy == 0 && dir == Dir.Up)
|| (yy == 7 && dir == Dir.Down)
|| (xx < 0 || xx > 7 || yy < 0 || yy > 7) { return false; }

//Check for matches
var _type = global.game_grid[yy, xx].image_index;
var _size;
switch(dir) {
	case Dir.Left: 
		var _temp = global.game_grid[yy, xx - 1];
		global.game_grid[yy, xx - 1] = global.game_grid[yy, xx];
		global.game_grid[yy, xx] = _temp;
		
		var _chain = scrGetChain(xx - 1, yy, _type);
		_size = ds_list_size(_chain);
		ds_list_destroy(_chain);
		
		var _temp = global.game_grid[yy, xx - 1];
		global.game_grid[yy, xx - 1] = global.game_grid[yy, xx];
		global.game_grid[yy, xx] = _temp;
	break;
	case Dir.Right: 
		var _temp = global.game_grid[yy, xx + 1];
		global.game_grid[yy, xx + 1] = global.game_grid[yy, xx];
		global.game_grid[yy, xx] = _temp;
		
		var _chain = scrGetChain(xx + 1, yy, _type);
		_size = ds_list_size(_chain);
		ds_list_destroy(_chain);
		
		var _temp = global.game_grid[yy, xx + 1];
		global.game_grid[yy, xx + 1] = global.game_grid[yy, xx];
		global.game_grid[yy, xx] = _temp;
	break;
	case Dir.Up: 
		var _temp = global.game_grid[yy - 1, xx];
		global.game_grid[yy - 1, xx] = global.game_grid[yy, xx];
		global.game_grid[yy, xx] = _temp;
		
		var _chain = scrGetChain(xx, yy - 1, _type);
		_size = ds_list_size(_chain);
		ds_list_destroy(_chain);
		
		var _temp = global.game_grid[yy - 1, xx];
		global.game_grid[yy - 1, xx] = global.game_grid[yy, xx];
		global.game_grid[yy, xx] = _temp;
	break;
	case Dir.Down: 
		var _temp = global.game_grid[yy + 1, xx];
		global.game_grid[yy + 1, xx] = global.game_grid[yy, xx];
		global.game_grid[yy, xx] = _temp;
		
		var _chain = scrGetChain(xx, yy + 1, _type);
		_size = ds_list_size(_chain);
		ds_list_destroy(_chain);
		
		var _temp = global.game_grid[yy + 1, xx];
		global.game_grid[yy + 1, xx] = global.game_grid[yy, xx];
		global.game_grid[yy, xx] = _temp;
	break;
}

return (_size >= 3);