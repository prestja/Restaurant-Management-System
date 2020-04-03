///@function scrAttemptSwap(piece1, piece2)
///@description Return true if swapping the pieces would result in a chain
///@param piece1
///@param piece2
var piece1 = argument0, piece2 = argument1;
if (!instance_exists(piece1) || !instance_exists(piece2)) { return false; }
var _i1 = piece1.grid_i, _j1 = piece1.grid_j;
var _i2 = piece2.grid_i, _j2 = piece2.grid_j;

//Swap pieces
scrPieceJump(piece1, _i2, _j2);
scrPieceJump(piece2, _i1, _j1);

//Check for chains at either space
var _chain1 = scrGetChain(_i1, _j1);
var _chain2 = scrGetChain(_i2, _j2);

//Swap pieces back
scrPieceJump(piece1, _i1, _j1);
scrPieceJump(piece2, _i2, _j2);

//Return if any chains were valid
var _valid = (_chain1 != undefined || _chain2 != undefined);
if (_chain1 != undefined) { ds_list_destroy(_chain1); }
if (_chain2 != undefined) { ds_list_destroy(_chain2); }
return _valid;