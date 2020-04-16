///@description Initialize global state
//Define macros
#macro ct_argument global.g_ct_argument
#macro ct_count global.g_ct_count
#macro font_w 12
#macro font_h 22

enum GameState {
	None,
	Playing, GameOver,
	Total
}

//Set options
global.fontDraw=font_add_sprite(sprFont, 33, false, -2);
draw_set_font(global.fontDraw);
global.game_score = 0;
device_mouse_dbclick_enable(false);		//Disable double-tapping registering as a right click

//Resolution setup
ideal_w=0;
ideal_h=240;
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
//Set zoom level
if (os_browser == browser_not_a_browser) { zoom = 2; }
else { zoom = min(screen_w div ideal_w, screen_h div ideal_h); }
//Set window sizes
surface_resize(application_surface, ideal_w*zoom, ideal_h*zoom);
display_set_gui_size(ideal_w*(zoom div 2), ideal_h*(zoom div 2));
window_set_size(ideal_w*zoom, ideal_h*zoom);

//Camera creation
center_camera();
camera = camera_create();
var vm = matrix_build_lookat(x, y, -1000, x, y, 0, 0, 1, 0);
var pm = matrix_build_projection_ortho(ideal_w, ideal_h, 1, 10000);
camera_set_view_mat(camera, vm);
camera_set_proj_mat(camera, pm);
view_camera[0] = camera;

game_state = GameState.None;
wall_timer = 120;