/// @description Draw HUD & text
//Draw score
draw_text(get_camera_l() + 6, get_camera_t() + 6, gameScore);

//Draw timer
if (global.gameState == GameState.WaitingResponse || (global.gameState == GameState.AnsweredQuestion && db_exists)) {
	var _x = get_camera_x() + 256 + 2*font_w, _y = get_camera_t() + (ideal_h/4) + font_h;
	draw_sprite(sprTimer, 0, _x, _y);
	scrDrawPie(_x + font_w, _y - (font_h * 2), questionTimer, GAME_SPEED*10, c_white, font_h, 1, 90, 90, c_black);
	var _time = (questionTimer == 0)? 0 : (questionTimer div GAME_SPEED) + 1;
	draw_text_shadow(_x, _y, string(_time));
}

//Draw batched shadow text
if (ds_exists(global.textBuffer, ds_type_queue) && !ds_queue_empty(global.textBuffer))
{
	//Set shadow shader
	//shader_set(shdTextShadow);
	
	//Draw batched text
	while(!ds_queue_empty(global.textBuffer))
	{
		//Get text properties
		var _text_prop = ds_queue_dequeue(global.textBuffer);
		/*
		if (draw_get_color() != _text_prop[4]) { draw_set_color(_text_prop[4]); }
		var _r = color_get_red(_text_prop[3]);
		var _g = color_get_green(_text_prop[3]);
		var _b = color_get_blue(_text_prop[3]);
		
		//Draw text
		shader_set_uniform_f(global.upixel_w, global.texel_w);
		shader_set_uniform_f(global.upixel_h, global.texel_h);
		shader_set_uniform_f(global.upixel_c, _r, _g, _b);
		draw_text(_text_prop[0], _text_prop[1], _text_prop[2]);
		*/
		draw_set_color(_text_prop[3]);
		draw_text(_text_prop[0] + 1, _text_prop[1] + 1, _text_prop[2]);
		draw_set_color(_text_prop[4]);
		draw_text(_text_prop[0], _text_prop[1], _text_prop[2]);
	}
	
	//Reset shader
	//shader_reset();
}