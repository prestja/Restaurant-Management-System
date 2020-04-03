///@description Initialize global values
#macro ct_argument global.g_ct_argument
#macro ct_count global.g_ct_count
#macro mask_width sprite_get_width(mask_index)
#macro mask_height sprite_get_width(mask_index)
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