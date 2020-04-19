///@function draw_pie(x, y, value, max, color, radius, [alpha], [start_angle], [num_segments], [color])
///@param x
///@param y
///@param value
///@param max
///@param color
///@param radius
///@param [alpha]
///@param [start_angle]
///@param [num_segments]
///@param [shadow_color]
var xx = argument[0], yy = argument[1], curr_val = argument[2], max_val = argument[3], col = argument[4], rad = argument[5];
var alpha = (argument_count > 6)? argument[6] : 1;
var start = (argument_count > 7)? argument[7] : 90;
var segments = (argument_count > 8)? argument[8] : 60;
var shadow = (argument_count > 9)? argument[9] : undefined;
if (curr_val > 0) {
	var i, len, tx, ty, val;
	var sizeofsection = 360/segments;
	var _old_alpha = draw_get_alpha(), _old_color = draw_get_color();
	val = (curr_val/max_val)*segments;
	
	// HTML5 version doesnt like triangle with only 2 sides
	if (val > 1) {
		//Draw shadow
		if (shadow != undefined) {
			draw_set_colour(shadow);
			draw_set_alpha(alpha);
		
			draw_primitive_begin(pr_trianglefan);
			draw_vertex(xx + 1, yy + 1);
		
			for(i=0; i<=val; i++) {
				len = (i*sizeofsection)+start;
				tx = lengthdir_x(rad, len);
				ty = lengthdir_y(rad, len);
				draw_vertex(xx + tx + 1, yy + ty + 1);
			}
			draw_primitive_end();
		}
		
		//Draw wheel
		draw_set_colour(col);
		draw_set_alpha(alpha);
		
		draw_primitive_begin(pr_trianglefan);
		draw_vertex(xx, yy);
		
		for(i=0; i<=val; i++) {
			len = (i*sizeofsection)+start;
			tx = lengthdir_x(rad, len);
			ty = lengthdir_y(rad, len);
			draw_vertex(xx+tx, yy+ty);
		}
		draw_primitive_end();
	}
	draw_set_alpha(_old_alpha);
	draw_set_alpha(_old_color);
}