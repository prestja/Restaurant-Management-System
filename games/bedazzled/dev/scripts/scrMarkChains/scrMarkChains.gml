///@function scrMarkChains()
///@description Mark all chains in the board for deletion
var _num_chains = 0, _score_increase = 0, _new_chain = false;
for(var i = 0; i < 8; ++i) {
	for(var j = 0; j < 8; ++j) {
		var _type = global.game_grid[i, j].image_index;
		var _chain = scrGetChain(j, i, _type);
		if (ds_list_size(_chain) >= 3) {
			//Mark all spaces in the chain
			_new_chain = true;
			for(var k = 0; k < ds_list_size(_chain); ++k) {
				var _space = scrCoordDecode(ds_list_find_value(_chain, k));
				if (global.destroy_grid[_space[1], _space[0]]) { 
					//This chain has already been marked
					_new_chain = false;
					break;
				}
				else { global.destroy_grid[_space[1], _space[0]] = true; }
			}
			//Count score increase
			if (_new_chain) {
				_num_chains++;
				_score_increase += _num_chains*(10 + ((ds_list_size(_chain) - 3)*5));
			}
		}
		ds_list_destroy(_chain);
	}
}
return _score_increase;