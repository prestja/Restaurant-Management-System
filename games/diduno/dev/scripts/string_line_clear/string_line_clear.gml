///@function string_line_clear(str)
///@description Erase the first line of a string, but keep the operator tags
///@param str
var str = argument0;
var _pos = 1;
var _char = string_char_at(str, _pos);
var _true_len = string_length_formatted(str);
while(_char != "\n" && _true_len > 0)
{
	//Skip operator tags
	while(_char == op_open)
	{
		while(_char != op_close)
		{
			_pos++;
			_char = string_char_at(str, _pos);
		}
			
		//Get next char
		_pos++;
		_char = string_char_at(str, _pos);
	}
	
	if (_char != "\n")
	{
		//Delete character
		_true_len--;
		str = string_delete(str, _pos, 1);
		
		//Get next char
		_char = string_char_at(str, _pos);
	}
}

//Delete newline
if (_true_len > 0) { str = string_delete(str, _pos, 1); }
return str;