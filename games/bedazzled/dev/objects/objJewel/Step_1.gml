/// @description Update custom paths
if (!cpt_running(myPath) && (goto_grid_i != grid_i || goto_grid_j != grid_j)) {
	myPos = [x, y];
	myDest = [global.board_x + (goto_grid_j*piece_size), global.board_y + (goto_grid_i*piece_size)];
	myPath = cpt_begin(myPos, cpt_fastin, 2, 4, myDest);
	settled = false;
	if (scrInBounds(grid_i, grid_j)) { global.game_grid[grid_i, grid_j] = noone; }
}

if (cpt_update(myPath, myPos)) {
	x = myPos[X];
	y = myPos[Y];
} else { myPath = -1; }

if (cpt_finished(myPath)) { 
	grid_i = goto_grid_i;
	grid_j = goto_grid_j;
	if (scrInBounds(grid_i, grid_j)) { global.game_grid[grid_i, grid_j] = self; }
	alarm[0] = 2;
}