///@description Manage state
if (global.gameState == GameState.RequestPending) {
	if (!requestPending) {
		//Check for a valid token
		if (httpToken != undefined) {
			//Request a question
			httpGet = http_get("https://opentdb.com/api.php?amount=1&category=9&difficulty=easy&encode=base64&type=multiple&token="+httpToken);
		}
		else {
			//Request a new token
			httpGet = http_get("https://opentdb.com/api_token.php?command=request");
		}
		requestPending = true;
	}
}
if (global.gameState == GameState.NewQuestion) {
	//Create dialogue box
	if (!db_exists) {
		db_create(tb_static, x, get_camera_t() + (ideal_h/4), 2, true, questionText);
	}
	
	//Create options
	else if (global.textboxDone) {
		//Create boxes
		if (!instance_exists(objOption)) {
			var _y1 = get_camera_t() + floor(5*ideal_h/8), _y2 = get_camera_t() + floor(7*ideal_h/8), _w = half_w(sprOption);
			optionsReferences[0] = instance_create_v(get_camera_l() - _w, _y1, "InstanceActors", objOption, questionOptions[0], 0);
			optionsReferences[1] = instance_create_v(get_camera_l() - _w, _y2, "InstanceActors", objOption, questionOptions[1], 1);
			optionsReferences[2] = instance_create_v(get_camera_r() + _w, _y1, "InstanceActors", objOption, questionOptions[2], 2);
			optionsReferences[3] = instance_create_v(get_camera_r() + _w, _y2, "InstanceActors", objOption, questionOptions[3], 3);
		}
	
		//Wait for player respone
		else { 
			//Check if all option boxes are done moving
			var _finished_moving = true;
			for(var i = 0; i < instance_number(objOption); ++i) {
				var _obj = instance_find(objOption, i);
				if (!_obj.settled) {
					_finished_moving = false;
					break;
				}
			}
			if (_finished_moving) {
				global.gameState = GameState.WaitingResponse; 
				questionTimer = (GAME_SPEED*10) - 1;
			}
		}
	}
}
else if (global.gameState == GameState.WaitingResponse) {
	//Count down
	questionTimer--;
	
	//Check for answer
	if (questionTimer == 0 || global.optionChosen != -1) { global.gameState = GameState.AnsweredQuestion; }
}
else if (global.gameState == GameState.AnsweredQuestion) {
	if (answerCountdown < 0) {
		//Change options colors
		for(var i = 0; i < 4; ++i) {
			if (i == questionCorrect) {
				with(optionsReferences[i]) { event_perform(ev_other, ev_user0); }
				if (global.optionChosen == i) { optionsReferences[i].image_index = 1; }
				else { optionsReferences[i].image_index = 2; }
			}
			else {
				if (global.optionChosen == i) { optionsReferences[i].image_index = 3; }
				else { optionsReferences[i].image_index = 4; }
			}
		}
		
		//Increase score
		var _score_increase = (questionCorrect == global.optionChosen)? ((questionTimer div GAME_SPEED) + 1)*10 : 0;
		gameScore += _score_increase;
		var _x = get_camera_l() + 6, _y = get_camera_t() + floor(3*ideal_h/8);
		instance_create_v(_x, _y, "InstanceActors", objScore, "+"+string(_score_increase));
		
		//Start countdown
		answerCountdown = GAME_SPEED * 2;
	}
	else if (answerCountdown > 0) { 
		//Count down
		answerCountdown--; 
		
		//Clear screen
		if (answerCountdown == 0) {
			//Move options away
			for(var i = 0; i < 4; ++i) {
				with(optionsReferences[i]) { event_perform(ev_other, ev_user1); }
			}
			//Remove textbox
			global.advanceTextbox = true;
		}
	}
	else {
		//Check if all option boxes are done moving
		var _finished_moving = true;
		for(var i = 0; i < instance_number(objOption); ++i) {
			var _obj = instance_find(objOption, i);
			if (!_obj.settled) {
				_finished_moving = false;
				break;
			}
		}
		if (_finished_moving) {
			global.gameState = GameState.RequestPending;
			global.optionChosen = -1;
			answerCountdown = -1;
			instance_destroy(objOption);
		}
	}
}