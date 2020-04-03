///@function scrMarkChains()
///@description Mark all pieces that need to be deleted & count how much the score should increase
var _num_chains = 0, _score_increase = 0;
for(var i = 0; i < 8; ++i) {
	for(var j = 0; j < 8; ++j) {
		//Get the chain at this space
		var _new_chain = true;
		var _chain = scrGetChain(i, j);
		if (_chain != undefined) {
			//Mark every space in the chain for destruction
			for(var k = 0; k < ds_list_size(_chain); ++k) {
				var _elem = scrCoordDecode(ds_list_find_value(_chain, k));
				//Remember if this chain has already been marked
				if (global.destroy_grid[_elem[0], _elem[1]]) { _new_chain = false; }
				global.destroy_grid[_elem[0], _elem[1]] = true;
			}
			
			//If the chain hasn't already been marked, count it
			if (_new_chain) { _num_chains++; }
			
			//Calculate score increase
			_score_increase += (10 + ((ds_list_size(_chain) - 3)*5)) * (1.5 + global.combo_level);
		}
	}
}
_score_increase = floor(_score_increase * (1.2 * _num_chains));
return _score_increase;