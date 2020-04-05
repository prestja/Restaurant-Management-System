///@function draw_textbox(sprite, border_size, xx, yy, text, [cols], [rows])
///@description Draw dynamically sized box w/ text
///@param sprite
///@param border_size
///@param xx
///@param yy
///@param text
///@param [cols]
///@param [rows]
var spr = argument[0], border = argument[1], xx = argument[2], yy = argument[3], text = argument[4], cols = 0, rows = 0;
if (argument_count > 5) { cols = argument[5]; }
else
{
	var _c = 0;
	for(var i=0; i<string_length_formatted(text); i++)
	{
		if (string_char_at(text, i) == "\n") { _c = 0; }
		else { cols = max(cols, ++_c); }
	}
}
if (argument_count > 6) { rows = argument[6]; }
else { rows = string_count("\n", text) + 1; }

//Top Left
if (spr[0] != undefined) { draw_sprite(spr[0], 0, xx, yy); }
//Top Right
if (spr[2] != undefined) { draw_sprite(spr[2], 0, xx + border + (font_w*cols), yy); }
//Bottom Left
if (spr[6] != undefined) { draw_sprite(spr[6], 0, xx, yy + border + (font_h*rows)); }
//Bottom Right
if (spr[8] != undefined) { draw_sprite(spr[8], 0, xx + border + (font_w*cols), yy + border + (font_h*rows)); }

//Left edge
if (spr[3] != undefined) { draw_sprite_ext(spr[3], 0, xx, yy + border, 1, (font_h*rows), 0, c_white, 1); }
//Right edge
if (spr[5] != undefined) { draw_sprite_ext(spr[5], 0, xx + border + (font_w*cols), yy + border, 1, (font_h*rows), 0, c_white, 1) }
//Top edge
if (spr[1] != undefined) { draw_sprite_ext(spr[1], 0, xx + border, yy, (font_w*cols), 1, 0, c_white, 1); }
//Bottom edge
if (spr[7] != undefined) { draw_sprite_ext(spr[7], 0, xx + border, yy + border + (font_h*rows), (font_w*cols), 1, 0, c_white, 1); }

//Center
if (spr[4] != undefined) { draw_sprite_ext(spr[4], 0, xx + border, yy + border, (font_w*cols), (font_h*rows), 0, c_white, 1); }
//Text
draw_text_formatted(xx + border, yy + border, text);