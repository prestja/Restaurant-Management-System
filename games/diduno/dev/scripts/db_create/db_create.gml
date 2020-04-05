///@function draw_textbox(type, x, y, speed, animate, text)
///@description Draw a textbox with formatted text scrolling at a speed, and if it should animate when it appears
///@param type
///@param x
///@param y
///@param speed
///@param animate
///@param text
if (!db_exists)
{
	var type = argument[0];
	var xx = (argument[1] >= 0)? argument[1] : room_width / 2;
	var yy = (argument[2] >= 0)? argument[2] : room_height;
	var spd = argument[3], anim = argument[4], text = argument[5];

	instance_create_v(xx, yy, "InstanceActors", objDialogueBox, type, spd, anim, text);
}