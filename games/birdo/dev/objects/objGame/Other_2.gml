///@description Initialize global state
//Define macros
#macro mask_width sprite_get_width(mask_index)
#macro mask_height sprite_get_width(mask_index)
#macro ct_argument global.g_ct_argument
#macro ct_count global.g_ct_count

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