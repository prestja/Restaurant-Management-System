///@function string_replace_char(substr, str, index)
///@description Replace the character at index in str with substr
///@param substr
///@param str
///@param index
var substr = argument0, str = argument1, index = argument2;
str = string_delete(str, index, 1);
str = string_insert(substr, str, index);
return str;