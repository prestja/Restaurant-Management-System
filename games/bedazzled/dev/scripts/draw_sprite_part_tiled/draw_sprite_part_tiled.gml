///@function draw_sprite_part_tiled(sprite, image, left, top, sprite_width, sprite_height, x, y, width, height)
///@description Draw sprite part tiled in the region defined by x, y, width, and height
///@param sprite
///@param image
///@param left
///@param top
///@param sprite_width
///@param sprite_height
///@param x
///@param y
///@param width
///@param height
var spr = argument0, img = argument1, spr_x = argument2, spr_y = argument3, spr_w = argument4, spr_h = argument5, xx = argument6, yy = argument7, ww = argument8, hh = argument9;
var _tile_w = ceil(ww / spr_w), _draw_w = 0;
var _tile_h = ceil(hh / spr_h), _draw_h = 0;
for(var i=0; i<_tile_h; i++)
{
	_draw_h = (i == _tile_h - 1)? (spr_h - (((i+1) * spr_h) - hh)) : spr_h;
	for(var j=0; j<_tile_w; j++)
	{
		_draw_w = (j == _tile_w - 1)? (spr_w - (((j+1) * spr_w) - ww)) : spr_w;
		draw_sprite_part(spr, img, spr_x, spr_y, _draw_w, _draw_h, xx + (j*spr_w), yy + (i*spr_h));
	}
}