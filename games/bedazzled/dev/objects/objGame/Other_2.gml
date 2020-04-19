///@description Initialize global values
#macro mask_width sprite_get_width(mask_index)
#macro mask_height sprite_get_width(mask_index)
#macro ct_argument global.g_ct_argument
#macro ct_count global.g_ct_count
#macro X 0
#macro Y 1
#macro NUM_PATHS 4
#macro cpt_none -1
#macro cpt_linear 0
#macro cpt_fastmid 1
#macro cpt_fastin 2
#macro cpt_fastout 3
#macro cpt_slowmid 4
#macro piece_size 32

global.fontDraw=font_add_sprite(sprFont, 33, false, -2);
draw_set_font(global.fontDraw);

enum GameState {
	None, Player, Swapping, GameOver,
	Total
}

enum Dir {
	Up = 0, Down, Left, Right,
	Total
}

//Resolution setup
ideal_w=0;
ideal_h=480;
if (os_browser == browser_not_a_browser) {
	screen_w=display_get_width();
	screen_h=display_get_height();
}
else {
	screen_w=max(browser_width, browser_height);
	screen_h=min(browser_width, browser_height);
}
//Get screen ratio
ratio=(screen_w/screen_h);
//Set proper width
ideal_w=round(ideal_h*ratio);
//Pixel Perfect Scaling
if(screen_w mod ideal_w !=0) {
    var d=round(screen_w/ideal_w);
    ideal_w=screen_w/d;
}
if(screen_h mod ideal_h !=0) {
    var d=round(screen_h/ideal_h);
    ideal_h=screen_h/d;
}
//Check for odd values
if(ideal_w & 1) {ideal_w++;}
if(ideal_h & 1) {ideal_h++;}
//Get zoom level
zoom = min(screen_w div ideal_w, screen_h div ideal_h);
if (os_browser == browser_not_a_browser) { zoom = 1; }
zoom = 1;
//Set window sizes
surface_resize(application_surface, ideal_w*zoom, ideal_h*zoom);
display_set_gui_size(ideal_w*0.5, ideal_h*0.5);
window_set_size(ideal_w*zoom, ideal_h*zoom);

//Camera creation
center_camera();
camera = camera_create();
var vm = matrix_build_lookat(x, y, -1000, x, y, 0, 0, 1, 0);
var pm = matrix_build_projection_ortho(ideal_w, ideal_h, 1, 10000);
camera_set_view_mat(camera, vm);
camera_set_proj_mat(camera, pm);
view_camera[0] = camera;

randomize();
global.game_grid[7, 7] = noone;
global.destroy_grid[7, 7] = false;
global.piece_one = noone
global.piece_two = noone;
global.game_state = GameState.Player;
global.combo_level = -1;
game_score = 0;

global.board_y = get_camera_t() + ((ideal_h - sprite_get_width(sprBackgroundGrid)) / 2);
global.board_x = get_camera_r() - sprite_get_width(sprBackgroundGrid) - global.board_y;

instance_create_layer(global.board_x, global.board_y, "InstanceActors", objBoard);
scrEmptyGameGrid();
scrEmptyDestroyGrid();
scrGenerateBoard();