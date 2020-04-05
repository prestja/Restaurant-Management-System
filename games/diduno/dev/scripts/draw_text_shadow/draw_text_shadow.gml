///@function draw_text_shadow(x, y, text, [shadowCol])
///@description Place the text & it's properties in the buffer to be drawn in a batch
///@param x
///@param y
///@param text
///@param [shadowCol]
var xx = argument[0], yy = argument[1], text = argument[2];
var shadow = (argument_count > 3)? argument[3] : c_black;
var _array = [xx, yy, text, shadow, draw_get_color()];
ds_queue_enqueue(global.textBuffer, _array);
//draw_set_color(c_black);
//draw_text(xx+1, yy+1, text);
//draw_set_color(c_white);
//draw_text(xx, yy, text);