///@function avg(val1, val2, ...max_val)
///@description Returns the average of the given values
///@param val1
///@param val2
///@param ...max_val
var _sum=0;
for(var i=0; i<argument_count; i++) { _sum+=argument[i]; }
return (_sum/argument_count);
