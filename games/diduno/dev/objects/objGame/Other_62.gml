///@description Get question from database
requestPending = false;
if (ds_map_find_value(async_load, "id") == httpGet) {
	var r_str = "";
	if (ds_map_find_value(async_load, "status") == 0) { r_str = ds_map_find_value(async_load, "result"); }
	else { exit; }
	var _resultMap = json_decode(r_str);
	
	if (httpToken == undefined) {
		//Get session token
		if (ds_map_find_value(_resultMap, "response_code") == 0) {
			httpToken = ds_map_find_value(_resultMap, "token");
		}
	}
	else {
		//Get question
		var _response = ds_map_find_value(_resultMap, "response_code");
		if (_response == 0) {
			//Success
			var _questionList = ds_map_find_value(_resultMap, "results");
			var _question = ds_list_find_value(_questionList, 0);
			var _questionIncorrects = ds_map_find_value(_question, "incorrect_answers");
			questionCorrect = irandom(3);
			questionText = base64_decode(ds_map_find_value(_question, "question"));
			var _index = 0;
			for(var i=0; i<4; ++i) {
				if (i == questionCorrect) {
					questionOptions[i] = base64_decode(ds_map_find_value(_question, "correct_answer"));
				}
				else {
					questionOptions[i] = base64_decode(ds_list_find_value(_questionIncorrects, _index));
					_index++;
				}
			}
			global.gameState = GameState.NewQuestion;
				
			//show_message(_questionText+"\n"+
			//_questionAnswers[0]+"\n"+
			//_questionAnswers[1]+"\n"+
			//_questionAnswers[2]+"\n"+
			//_questionAnswers[3]);
		}
		else if (_response == 3 || _response == 4) {
			//Token invalid
			httpToken = undefined;
		}
	}
}
