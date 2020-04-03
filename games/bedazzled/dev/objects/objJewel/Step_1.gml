/// @description Update custom paths
if (!cpt_running(myPath) && (goto_grid_x != grid_x || goto_grid_y != grid_y)) {
	myPos = [x, y];
	myDest = [global.board_x + (goto_grid_x*64), global.board_y + (goto_grid_y*64)];
	myPath = cpt_begin(myPos, cpt_fastin, 2, 6, myDest);
}
if (cpt_update(myPath, myPos))
{
	x = myPos[X];
	y = myPos[Y];
} else { myPath = -1; }
if (cpt_finished(myPath)) { 
	grid_x = goto_grid_x;
	grid_y = goto_grid_y;
	global.game_grid[grid_y, grid_x] = self;
}