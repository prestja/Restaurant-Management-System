///@function room_goto_fade(room, [color])
///@description Wrapper for fancy room transitions
///@param room
///@param [color]
var rm = argument[0];
var color = (argument_count > 1)? argument[1] : c_black;
instance_create_v(0, 0, "InstanceControl", objRoomTransition, rm, color);