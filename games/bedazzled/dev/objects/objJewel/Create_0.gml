///@description Initialize
image_speed = 0;
image_index = ct_argument[0];
grid_y = ct_argument[1];
grid_x = ct_argument[2];
goto_grid_y = grid_y;
goto_grid_x = grid_x;
cpt_init();
myPos = [x, y];
myDest = [x, y];
myPath = -1;