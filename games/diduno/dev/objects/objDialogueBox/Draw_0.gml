/// @description Draw box & text
//Top Left
if (tb_border_spr[0] != undefined) { draw_sprite(tb_border_spr[0], 0, x - drawW - tb_border, y - drawH - tb_border); }
//Top Right
if (tb_border_spr[2] != undefined) { draw_sprite(tb_border_spr[2], 0, x + drawW, y - drawH - tb_border); }
//Bottom Left
if (tb_border_spr[6] != undefined) { draw_sprite(tb_border_spr[6], 0, x - drawW - tb_border, y + drawH); }
//Bottom Right
if (tb_border_spr[8] != undefined) { draw_sprite(tb_border_spr[8], 0, x + drawW, y + drawH); }

//Left edge
if (tb_border_spr[3] != undefined) { draw_sprite_ext(tb_border_spr[3], 0, x - drawW - tb_border, y - drawH, 1, (drawH * 2), 0, c_white, 1); }
//Right edge
if (tb_border_spr[5] != undefined) { draw_sprite_ext(tb_border_spr[5], 0, x + drawW, y - drawH, 1, (drawH * 2), 0, c_white, 1) }
//Top edge
if (tb_border_spr[1] != undefined) { draw_sprite_ext(tb_border_spr[1], 0, x - drawW, y - drawH - tb_border, (drawW * 2), 1, 0, c_white, 1); }
//Bottom edge
if (tb_border_spr[7] != undefined) { draw_sprite_ext(tb_border_spr[7], 0, x - drawW, y + drawH, (drawW * 2), 1, 0, c_white, 1); }

//Center
if (tb_border_spr[4] != undefined) { draw_sprite_ext(tb_border_spr[4], 0, x - drawW, y - drawH, (drawW * 2), (drawH * 2), 0, c_white, 1); }

//Text
if (!cpt_running(sizePath)) { draw_text_shadow(drawTextX, drawTextY, drawText); }
/*
//Arrows
var _frame = ((global.frameTimer mod (GAME_SPEED/2)) < (GAME_SPEED/4));
if (myState == 1) { draw_sprite(tb_next_spr, _frame, x + drawW, y + drawH); }
if (myState == 2) { 
	draw_sprite(tb_stop_spr, _frame, x + drawW, y + drawH);
}*/