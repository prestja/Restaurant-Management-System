///@function half_w([sprite])
///@description Returns half of the sprites width
///@param [sprite]
var spr = (argument_count>0)? argument[0] : sprite_index;
return (sprite_get_width(spr)/2);
