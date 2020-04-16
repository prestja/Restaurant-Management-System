///@function string_insert_newline(string, max_width)
///@description Replace whitespace with newlines so that no line is longer than max_width (taking operator tags into account)
///@param string
///@param max_width
var str = argument0, max_width = argument1, out = "";
while(string_length(str) > 0) {
	//Get the next chunk of text, up to and including the next newline
	var _chunk_end = string_pos("\n", str);
	var _chunk_length = (_chunk_end > 0)? _chunk_end : string_length(str);
	var _chunk_text = string_copy(str, 1, _chunk_length);
	var _pos = _chunk_length;
	
	//If the chunk is longer than maxWidth, break it apart
	if (string_length(_chunk_text) > max_width) {
		//Check for spaces
		var _space_count = string_count(" ", _chunk_text);
		
		//Spaces are present
		if (_space_count > 0) {
			var _line_length = 0, _space_last = 0;
			while(string_length(_chunk_text) > 0) {
				//Get the next chunk of text, up to and including the next whitespace
				var _space_end = string_pos(" ", _chunk_text);
				var _space_length = (_space_end > 0)? _space_end : string_length(_chunk_text);
				var _space_text = string_copy(_chunk_text, 1, _space_length);
				
				//Get its length without operator tags
				var _text_length = string_length_formatted(_space_text);
				//show_message("["+_space_text+"]\nspace_length:"+string(_space_length)+"\ntext_length:"+string(_text_length)+"\nline_length:"+string(_line_length)+"\nspace_last:"+string(_space_last))
				
				//Replace the previous whitespace with a newline
				if (((_line_length + _text_length) - 1) > max_width) {
					str = string_replace_char("\n", str, _space_last);
					_pos = _line_length;
					break;
				}
				_space_last += _space_length;
				
				//Delete the chunk
				_line_length += _text_length;
				_chunk_text = string_delete(_chunk_text, 1, _space_length);
			}
		}
		//Unbroken string of characters
		else {
			//Get the position at maxWidth, disregarding operator tags
			var _line_length = string_length(_chunk_text);
			var _text_length = string_length_formatted(_chunk_text);
			var _offset = (_line_length - _text_length);
			str = string_insert("\n", str, max_width + 1 + _offset);
		}
	}
	
	//Delete the chunk
	out += string_copy(str, 1, _pos);
	str = string_delete(str, 1, _pos);
}
return out;