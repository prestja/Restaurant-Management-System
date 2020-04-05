///@function string_format_right(val, len, [char])
///@description Returns the value formatted as a string len characters long, right-justified, filling space with [char]
///@param val
///@param len
///@param [char]
var text = (is_string(argument[0]))? argument[0] : string(argument[0]);
var len = argument[1];
var char = (argument_count > 2)? argument[2] : " ";
while(string_length_formatted(text) < len) { text = string_insert(char, text, 0); }
return text;