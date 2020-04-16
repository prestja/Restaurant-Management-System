///@function scrMouseInRegion(device, x1, y1, x2, y2)
///@description Checks if the device mouse position is in the given region in game space
///@param device
///@param x1
///@param y1
///@param x2
///@param y2
var device = argument0, x1 = argument1, y1 = argument2, x2 = argument3, y2 = argument4;
return (device_mouse_x(device) >= x1 && device_mouse_x(device) < x2 &&
		device_mouse_y(device) >= y1 && device_mouse_y(device) < y2);