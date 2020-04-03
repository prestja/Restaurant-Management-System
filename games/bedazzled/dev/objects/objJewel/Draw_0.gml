///@description Draw highlights
draw_self();
//draw_set_color(c_white);
//draw_text(x, y, "x"+string(grid_x)+"\ny"+string(grid_y)+"/"+string(goto_grid_y)+"\nt"+string(image_index));
if (global.piece_selected == self) {
	//draw_set_color(c_white);
	draw_rectangle(x, y, x + sprite_width, y + sprite_height, true);
}
/*
if (grid_y >= 0) {
	if (global.destroy_grid[grid_y, grid_x]) {
		draw_set_color(c_red);
		draw_rectangle(x, y, x + sprite_width, y + sprite_height, true);
	}
}*/