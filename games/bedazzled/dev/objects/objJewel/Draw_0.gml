///@description Draw highlights
draw_self();
//draw_set_color(c_white);
//draw_text(x, y, "i"+string(grid_i)+"/"+string(goto_grid_i)+"\nj"+string(grid_j)+"\nt"+string(image_index));
if (global.piece_one == self) {
	//draw_set_color(c_white);
	draw_rectangle(x, y, x + sprite_width, y + sprite_height, true);
}
/*
if (scrInBounds(grid_i, grid_j)) {
	if (global.destroy_grid[grid_i, grid_j]) {
		draw_set_color(c_red);
		draw_rectangle(x, y, x + sprite_width, y + sprite_height, true);
	}
}*/