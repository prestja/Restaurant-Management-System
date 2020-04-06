///@description Initialize game state
#macro X 0
#macro Y 1
#macro mask_width sprite_get_width(mask_index)
#macro mask_height sprite_get_width(mask_index)
#macro ct_argument global.g_ct_argument
#macro ct_count global.g_ct_count
#macro GAME_SPEED 60
#macro NUM_PATHS 4
#macro cpt_none -1
#macro cpt_linear 0
#macro cpt_fastmid 1
#macro cpt_fastin 2
#macro cpt_fastout 3
#macro cpt_slowmid 4
#macro font_w 12
#macro font_h 22
#macro font_sprite sprFont
#macro op_open "{"
#macro op_close "}"
#macro op_sep ","

global.textbox_9slice_array = 
[sprBoxBorderTopLeft, sprBoxBorderTop, sprBoxBorderTopRight,
 sprBoxBorderMiddleLeft, sprBoxBorderMiddle, sprBoxBorderMiddleRight,
 sprBoxBorderBottomLeft, sprBoxBorderBottom, sprBoxBorderBottomRight];

#macro tb_border_spr global.textbox_9slice_array
#macro tb_border 16
#macro tb_static 500
#macro tb_brief 501
#macro db_exists instance_exists(objDialogueBox)

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
//Set zoom level
if (os_browser == browser_not_a_browser) { zoom = 1; }
else { zoom = min(screen_w div ideal_w, screen_h div ideal_h); }
//Set window sizes
surface_resize(application_surface, ideal_w*zoom, ideal_h*zoom);
display_set_gui_size(ideal_w, ideal_h);
window_set_size(ideal_w*zoom, ideal_h*zoom);

//Camera creation
center_camera();
camera = camera_create();
var vm = matrix_build_lookat(x, y, -1000, x, y, 0, 0, 1, 0);
var pm = matrix_build_projection_ortho(ideal_w, ideal_h, 1, 10000);
camera_set_view_mat(camera, vm);
camera_set_proj_mat(camera, pm);
view_camera[0] = camera;

shadow_text_init();
global.advanceTextbox = false;
global.textboxDone = false;
global.dialogueExistsFlag = false;
global.optionChosen = -1;
global.fontDraw=font_add_sprite(font_sprite, 33, false, -2);
draw_set_font(global.fontDraw);
global.frameTimer = 0;

enum GameState {
	None = 0,
	NewQuestion, WaitingResponse, AnsweredQuestion,
	Total
}

enum QuestionProperties {
	None = 0,
	Text, Option0, Option1, Option2, Option3, CorrectOption,
	Total
}

global.gameState = GameState.NewQuestion;
questionTimer = 0;
optionsReferences = [noone, noone, noone, noone];
questionId = -1;
lastQuestion = -1;
maxQuestion = 3;
answerCountdown = -1;
gameScore = 0;