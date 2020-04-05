///@description Initialize global values
#macro X 0
#macro Y 1
#macro NUM_PATHS 4
#macro cpt_none -1
#macro cpt_linear 0
#macro cpt_fastmid 1
#macro cpt_fastin 2
#macro cpt_fastout 3
#macro cpt_slowmid 4

enum GameState {
	None = 0,
	PlayerTurn, ChangeTurn,
	VictoryOne, VictoryTwo, Draw,
	Total
}

global.fontDraw=font_add_sprite(sprFont, 33, false, -2);
draw_set_font(global.fontDraw);
global.space_selected = noone;

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
//Set window sizes
surface_resize(application_surface, ideal_w, ideal_h);
display_set_gui_size(ideal_w, ideal_h);
window_set_size(ideal_w, ideal_h);

//Camera creation
center_camera();
camera = camera_create();
var vm = matrix_build_lookat(x, y, -1000, x, y, 0, 0, 1, 0);
var pm = matrix_build_projection_ortho(ideal_w, ideal_h, 1, 10000);
camera_set_view_mat(camera, vm);
camera_set_proj_mat(camera, pm);
view_camera[0] = camera;

player_turn = 0;
game_state = GameState.PlayerTurn;
board_x = (x - (ideal_w / 2)) + ((ideal_w - sprite_get_width(sprBackgroundGrid)) / 2);
board_y = (y - (ideal_h / 2)) + ((ideal_h - sprite_get_height(sprBackgroundGrid)) / 2);

//Create spaces
space_width = 120;
instance_create_layer(x, y, "InstanceActors", objBoard);
instance_create_layer((board_x - space_width) / 2, ideal_h - floor(1.5*sprite_get_height(sprSpace)), layer, objPlayer1Marker);
instance_create_layer(board_x + sprite_get_width(sprBackgroundGrid) + ((board_x - space_width) / 2), ideal_h - floor(1.5*sprite_get_height(sprSpace)), layer, objPlayer2Marker);
spaces = array_create(9);
for(var i=0; i<9; i++) {
	var _xx = (i mod 3)*space_width;
	var _yy = (i div 3)*space_width;
	spaces[i] = instance_create_layer(board_x + _xx, board_y + _yy, layer, objSpace);
}