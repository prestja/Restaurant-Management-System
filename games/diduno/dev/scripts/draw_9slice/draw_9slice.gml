///@function draw_9slice(sprite, border_size, xx, yy, ww, hh)
///@description Draw dynamically sized box
///@param sprite
///@param border_size
///@param xx
///@param yy
///@param ww
///@param hh
var spr = argument[0], border = argument[1], xx = argument[2], yy = argument[3], ww = argument[4], hh = argument[5];
//Top Left
if (spr[0] != undefined) { draw_sprite(spr[0], 0, xx, yy); }
//Top Right
if (spr[2] != undefined) { draw_sprite(spr[2], 0, xx + border + ww, yy); }
//Bottom Left
if (spr[6] != undefined) { draw_sprite(spr[6], 0, xx, yy + border + hh); }
//Bottom Right
if (spr[8] != undefined) { draw_sprite(spr[8], 0, xx + border + ww, yy + border + hh); }

//Left edge
if (spr[3] != undefined) { draw_sprite_ext(spr[3], 0, xx, yy + border, 1, hh, 0, c_white, 1); }
//Right edge
if (spr[5] != undefined) { draw_sprite_ext(spr[5], 0, xx + border + ww, yy + border, 1, hh, 0, c_white, 1) }
//Top edge
if (spr[1] != undefined) { draw_sprite_ext(spr[1], 0, xx + border, yy, ww, 1, 0, c_white, 1); }
//Bottom edge
if (spr[7] != undefined) { draw_sprite_ext(spr[7], 0, xx + border, yy + border + hh, ww, 1, 0, c_white, 1); }

//Center
if (spr[4] != undefined) { draw_sprite_ext(spr[4], 0, xx + border, yy + border, ww, hh, 0, c_white, 1); }