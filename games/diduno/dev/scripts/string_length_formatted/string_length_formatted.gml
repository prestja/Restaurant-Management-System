///@function string_length_formatted(text)
///@description Returns the length of a string without the formatting tags
///@param text
var text = argument0, _len = 0, _pos = 0;
while(string_length(text) > 0)
{
	//Get the next segment
	var _chunk_end = string_pos(op_open, text);
	var _chunk_len = (_chunk_end > 0)? _chunk_end - 1 : string_length(text);
	_pos = _chunk_len;
	
	//Record the length of the segment
	_len += _chunk_len;
	
	//Skip past any operator tags
	if (_chunk_end > 0)
	{
		var _op_end = string_pos(op_close, text) - _pos;
		var _op_len = (_op_end > 0)? _op_end : string_length(text);
		_pos += _op_len;
	}
	
	//Trim down remaining text
	text = string_delete(text, 1, _pos);
}
return _len;