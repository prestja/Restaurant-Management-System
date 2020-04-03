///@function approach(val1, val2, rate)
///@description Step val1 towards val2 at the given rate, from either direction
///@param val1
///@param val2
///@param rate
var val1 = argument0, val2 = argument1, rate = argument2;
return (abs(val1 - val2) < rate)? val2 : (val1 - (rate * sign(val1 - val2)));