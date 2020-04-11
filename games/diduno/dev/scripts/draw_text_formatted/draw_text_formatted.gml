///@function draw_text_formatted(x, y, text)
///@description Draws text with support for modification operators
///@param x
///@param y
///@param text
var xx = argument0, yy = argument1, text = argument2;
var _dx = xx, _dy = yy, _pos = 0;
var _draw_col = c_white, _shadow_col = c_black, _move_type = 0;
while(string_length(text) > 0)
{
	//Get the next chunk of text up to the next operator tag (or the end)
	var _chunk_end = string_pos(op_open, text) - 1;
	var _chunk_len = (_chunk_end >= 0)? _chunk_end : string_length(text);
	var _chunk_text = string_copy(text, 0, _chunk_len);
	_pos = _chunk_len;
	
	//Draw the text chunk
	while (string_length(_chunk_text) > 0)
	{
		//Get the next line
		var _line_end = string_pos("\n", _chunk_text) - 1;
		var _line_len = (_line_end >= 0)? _line_end : string_length(_chunk_text);
		var _line_text = string_copy(_chunk_text, 0, _line_len);
		
		//Draw line
		if (draw_get_color() != _draw_col) { draw_set_color(_draw_col); }
		switch(_move_type)
		{
			case 1:
				//Waving text
				var _wave_off = -1, _wave_mod = (GAME_SPEED / 10), _wave_period = ((2*pi)/GAME_SPEED), _wave_amp = 2;
				for(var i=0; i<string_length(_line_text); i++)
				{
					//Calculate height
					_wave_off = (string_char_at(_line_text, i + 1) == " ")? 0 : _wave_off + 1;
					var _h = _wave_amp * sin(_wave_period * (global.frameTimer + (_wave_off * _wave_mod)));
					
					//Draw character
					draw_text_shadow(_dx, _dy + _h, string_char_at(_line_text, i + 1), _shadow_col);
					_dx += string_width(string_char_at(_line_text, i + 1));
				}

				//Set draw coords
				if (_line_end >= 0) { _dx = xx; }
				_dy = (_line_end >= 0)? (_dy + string_height("A")) : _dy;
			break;
			case 2:
				//Shaking text
				for(var i=0; i<string_length(_line_text); i++)
				{
					//Calculate offset
					var _h = ((global.frameTimer + irandom(15)) mod 29 > 15)? 0 : 1;
					var _w = ((global.frameTimer + irandom(15)) mod 31 > 15)? 0 : 1;
					//var _h = ((global.frameTimer + i) mod 3)? 0 : irandom_range(0, 1)
					//var _w = ((global.frameTimer + i) mod 3)? 0 : irandom_range(0, 1);
					
					//Draw character
					draw_text_shadow(_dx + _w, _dy + _h, string_char_at(_line_text, i + 1), _shadow_col);
					_dx += string_width(string_char_at(_line_text, i + 1));
				}
				
				//Set draw coords
				if (_line_end >= 0) { _dx = xx; }
				_dy = (_line_end >= 0)? (_dy + string_height("A")) : _dy;
			break;
			default:
				//Normal text
				draw_text_shadow(_dx, _dy, _line_text, _shadow_col);
		
				//Set draw coords
				_dx = (_line_end >= 0)? xx : (_dx + string_width(_line_text));
				_dy = (_line_end >= 0)? (_dy + string_height("A")) : _dy;
			break;
		}
		
		//Trim down remaining text
		_chunk_text = string_delete(_chunk_text, 1, _line_len + 1);
	}
	
	//Get the operator tag
	if (_chunk_end >= 0)
	{
		//Get the size of the operator tag
		var _op_end = string_pos(op_close, text) - 1;
		var _op_text = string_copy(text, _pos + 2, _op_end - _pos - 1);
		if (_op_end < 0)
		{
			//Error
			break;
		}
		
		//Iterate through operators in tag
		while(string_length(_op_text) > 0)
		{
			//Get the first tag
			var _tag_end = string_pos(op_sep, _op_text) - 1;
			var _tag_len = (_tag_end >= 0)? _tag_end : string_length(_op_text);
			var _tag_text = string_copy(_op_text, 0, _tag_len);
			
			//Error check
			if (string_char_at(_tag_text, 2) != "=")
			{
				//Error
				break;
			}
			
			//Evaluate tag
			switch(string_char_at(_tag_text, 1))
			{
				case "C":
					//Get tag value
					_tag_text = string_delete(_tag_text, 1, 2);
					switch(_tag_text)
					{
						case "red": _draw_col = c_red; break;
						case "green": _draw_col = c_green; break;
						case "blue": _draw_col = c_blue; break;
						case "white": _draw_col = c_white; break;
						case "black": _draw_col = c_black; break;
						case "gray": _draw_col = c_gray; break;
						default:
							//Error
						break;
					}
				break;
				case "S":
					//Get tag value
					_tag_text = string_delete(_tag_text, 1, 2);
					switch(_tag_text)
					{
						case "red": _shadow_col = c_red; break;
						case "green": _shadow_col = c_green; break;
						case "blue": _shadow_col = c_blue; break;
						case "white": _shadow_col = c_white; break;
						case "black": _shadow_col = c_black; break;
						default:
							//Error
						break;
					}
				break;
				case "M":
					//Get tag value
					_tag_text = string_delete(_tag_text, 1, 2);
					switch(_tag_text)
					{
						case "normal": _move_type = 0; break;
						case "wave": _move_type = 1; break;
						case "shake": _move_type = 2; break;
						default:
							//Error
						break;
					}
				break;
				default:
					//Error
				break;
			}
			
			//Trim down remaining text
			_op_text = string_delete(_op_text, 1, _tag_len+1);
		}
		
		//Set position past tag
		_pos = _op_end + 1;
	}
	
	//Trim down string
	text = string_delete(text, 1, _pos);
	
	//debug_popup("Chunk: ["+_chunk_text+"]\nRemaining: ["+text+"]\nPos: "+string(_pos));
}