///@function scrGetChain(i, j)
///@description Return a ds_list of all spaces that form a valid chain connected to [i, j], or undefined if there is none
///@param i
///@param j
var i = argument0, j = argument1;
//Get initial piece
var _type;
if (instance_exists(global.game_grid[i, j])) { _type = global.game_grid[i, j].image_index; }
else { return undefined; }

//Initialize lists
var _chain_v = ds_list_create();
var _chain_h = ds_list_create();

//Look left
if (j > 0) {
	for(var xx = j - 1; xx >= 0; --xx) {
		if (instance_exists(global.game_grid[i, xx])) {
			if (global.game_grid[i, xx].image_index == _type) {
				ds_list_add(_chain_h, scrCoordEncode(i, xx));
			}
			else { break; }
		}
		else { break; }
	}
}

//Look right
if (j < 7) {
	for(var xx = j + 1; xx <= 7; ++xx) {
		if (instance_exists(global.game_grid[i, xx])) {
			if (global.game_grid[i, xx].image_index == _type) {
				ds_list_add(_chain_h, scrCoordEncode(i, xx));
			}
			else { break; }
		}
		else { break; }
	}
}

//Look up
if (i > 0) {
	for(var yy = i - 1; yy >= 0; --yy) {
		if (instance_exists(global.game_grid[yy, j])) {
			if (global.game_grid[yy, j].image_index == _type) {
				ds_list_add(_chain_v, scrCoordEncode(yy, j));
			}
			else { break; }
		}
		else { break; }
	}
}

///Look down
if (i < 7) {
	for(var yy = i + 1; yy <= 7; ++yy) {
		if (instance_exists(global.game_grid[yy, j])) {
			if (global.game_grid[yy, j].image_index == _type) {
				ds_list_add(_chain_v, scrCoordEncode(yy, j));
			}
			else { break; }
		}
		else { break; }
	}
}

if (ds_list_size(_chain_h) >= 2) { 
	ds_list_destroy(_chain_v);
	ds_list_add(_chain_h, scrCoordEncode(i, j));
	return _chain_h; 
}
else if (ds_list_size(_chain_v) >= 2) {
	ds_list_destroy(_chain_h);
	ds_list_add(_chain_v, scrCoordEncode(i, j));
	return _chain_v;
}
else {
	ds_list_destroy(_chain_h);
	ds_list_destroy(_chain_v);
	return undefined;
}