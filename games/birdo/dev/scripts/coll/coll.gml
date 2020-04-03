///@function coll(obj)
///@description Script for simulating collsisions in step events
///@param obj
return collision_rectangle(x-sprite_get_xoffset(mask_index), y-sprite_get_yoffset(mask_index), x+mask_width-sprite_get_xoffset(mask_index), y+mask_height-sprite_get_yoffset(mask_index), argument0, false, true);
