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