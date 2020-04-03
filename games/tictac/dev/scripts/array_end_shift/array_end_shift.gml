///@function array_end_shift(array, pos)
///@description Move the element at pos in array to the end, shifting later positions up to fill the space
///@param array
///@param pos
var array = argument0, pos = argument1;
var _temp = array[@ pos];
for(var i=pos+1; i<array_length_1d(array); i++) { array[@ i - 1] = array[@ i]; }
array[@ array_length_1d(array) - 1] = _temp;
return;