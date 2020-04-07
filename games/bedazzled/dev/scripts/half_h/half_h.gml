///@function half_h([sprite])
///@description Returns half of the sprites height
///@param [sprite]
var spr = (argument_count>0)? argument[0] : sprite_index;
return (sprite_get_height(spr)/2);
