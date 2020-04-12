///@function scrMouseOnMe(device)
///@description Checks if the device mouse position is on the given instance
///@param device
var device = argument0;
return (device_mouse_x(device) >= bbox_left && device_mouse_x(device) < bbox_right &&
		device_mouse_y(device) >= bbox_top && device_mouse_y(device) < bbox_bottom);