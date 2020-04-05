///@description Manage state
if (global.gameState == GameState.NewQuestion) {
	//Create dialogue box
	if (!db_exists) {
		//Choose a question from the list
		questionId = irandom(maxQuestion);
		if (questionId == lastQuestion) { questionId = wrap(questionId + 1, 0, maxQuestion); }
		lastQuestion = questionId;
		db_create(tb_static, room_width/2, 135, 2, true, scrQuestionGet(questionId, QuestionProperties.Text));
	}
	
	//Create options
	else if (global.textboxDone) {
		//Create boxes
		if (!instance_exists(objOption)) {
			optionsReferences[0] = instance_create_v(-sprite_get_width(sprOption), 279, "InstanceActors", objOption, scrQuestionGet(questionId, QuestionProperties.Option0), 0);
			optionsReferences[1] = instance_create_v(-sprite_get_width(sprOption), 416, "InstanceActors", objOption, scrQuestionGet(questionId, QuestionProperties.Option1), 1);
			optionsReferences[2] = instance_create_v(room_width, 279, "InstanceActors", objOption, scrQuestionGet(questionId, QuestionProperties.Option2), 2);
			optionsReferences[3] = instance_create_v(room_width, 416, "InstanceActors", objOption, scrQuestionGet(questionId, QuestionProperties.Option3), 3);
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
		var _correct_option = scrQuestionGet(questionId, QuestionProperties.CorrectOption);
		for(var i = 0; i < 4; ++i) {
			if (i == _correct_option) {
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
		var _score_increase = (_correct_option == global.optionChosen)? ((questionTimer div GAME_SPEED) + 1)*10 : 0;
		gameScore += _score_increase;
		instance_create_v(48, 214, "InstanceActors", objScore, "+"+string(_score_increase));
		
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
			global.gameState = GameState.NewQuestion;
			global.optionChosen = -1;
			answerCountdown = -1;
			instance_destroy(objOption);
		}
	}
}