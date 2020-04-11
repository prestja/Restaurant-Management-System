///@description Initialize
cpt_init();
image_speed = 0;
image_index = ct_argument[0];
grid_i = ct_argument[1];
grid_j = ct_argument[2];
goto_grid_i = grid_i;
goto_grid_j = grid_j;
myPos = [x, y];
myDest = [x, y];
myPath = -1;
settled = true;
drag_detect_buffer = 6;
drag_detect_flag = false;