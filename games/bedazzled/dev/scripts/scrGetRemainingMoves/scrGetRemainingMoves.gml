///@function scrGetRemainingMoves()
///@description Return the number of possible remaining moves
var _numMoves = 0;
for(var i = 0; i < 8; ++i) {
	for(var j = 0; j < 8; ++j) {
		//Try to swap this piece in each direction
		for(var d = 0; d < Dir.Total; ++d) {
			if (scrAttemptPieceSwap(j, i, d)) { _numMoves++; }
		}
	}
}
return _numMoves;