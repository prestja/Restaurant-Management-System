///@description Register space selection
if (game_state == GameState.PlayerTurn && global.space_selected.image_index == 0) {
	//Mark space
	if (player_turn == 0) {
		with (global.space_selected) { image_index = 1; }
	}
	else {
		with (global.space_selected) { image_index = 2; }
	}
	
	//Check for victory
	if (spaces[0].image_index == 1 && spaces[1].image_index == 1 && spaces[2].image_index == 1)
	|| (spaces[3].image_index == 1 && spaces[4].image_index == 1 && spaces[5].image_index == 1)
	|| (spaces[6].image_index == 1 && spaces[7].image_index == 1 && spaces[8].image_index == 1)
	|| (spaces[0].image_index == 1 && spaces[3].image_index == 1 && spaces[6].image_index == 1)
	|| (spaces[1].image_index == 1 && spaces[4].image_index == 1 && spaces[7].image_index == 1)
	|| (spaces[2].image_index == 1 && spaces[5].image_index == 1 && spaces[8].image_index == 1)
	|| (spaces[0].image_index == 1 && spaces[4].image_index == 1 && spaces[8].image_index == 1)
	|| (spaces[6].image_index == 1 && spaces[4].image_index == 1 && spaces[2].image_index == 1) { game_state = GameState.VictoryOne; }
	else if (spaces[0].image_index == 2 && spaces[1].image_index == 2 && spaces[2].image_index == 2)
		 || (spaces[3].image_index == 2 && spaces[4].image_index == 2 && spaces[5].image_index == 2)
		 || (spaces[6].image_index == 2 && spaces[7].image_index == 2 && spaces[8].image_index == 2)
		 || (spaces[0].image_index == 2 && spaces[3].image_index == 2 && spaces[6].image_index == 2)
		 || (spaces[1].image_index == 2 && spaces[4].image_index == 2 && spaces[7].image_index == 2)
		 || (spaces[2].image_index == 2 && spaces[5].image_index == 2 && spaces[8].image_index == 2)
		 || (spaces[0].image_index == 2 && spaces[4].image_index == 2 && spaces[8].image_index == 2)
		 || (spaces[6].image_index == 2 && spaces[4].image_index == 2 && spaces[2].image_index == 2) { game_state = GameState.VictoryTwo; }
	else if (spaces[0].image_index != 0 && spaces[1].image_index != 0 && spaces[2].image_index != 0)
		 && (spaces[3].image_index != 0 && spaces[4].image_index != 0 && spaces[5].image_index != 0)
		 && (spaces[6].image_index != 0 && spaces[7].image_index != 0 && spaces[8].image_index != 0) { game_state = GameState.Draw; }
		
	//Change turn
	if (game_state == GameState.PlayerTurn) {
		player_turn = (player_turn == 0)? 1 : 0;
		if (player_turn == 0) { 
			with(objPlayer1Marker) { event_perform(ev_other, ev_user1); }
			with(objPlayer2Marker) { event_perform(ev_other, ev_user0); }
		}
		else { 
			with(objPlayer1Marker) { event_perform(ev_other, ev_user0); }
			with(objPlayer2Marker) { event_perform(ev_other, ev_user1); }
		}
	}
	else if (game_state == GameState.Draw) {
		with(objPlayer1Marker) { event_perform(ev_other, ev_user0); }
		with(objPlayer2Marker) { event_perform(ev_other, ev_user0); }
	}
}
else if (game_state == GameState.VictoryOne || game_state == GameState.VictoryTwo || game_state == GameState.Draw) {
	for(var i=0; i<9; i++) { spaces[i].image_index = 0; }
	game_state = GameState.PlayerTurn;
	player_turn = 0;
	with(objPlayer1Marker) { event_perform(ev_other, ev_user1); }
	with(objPlayer2Marker) { event_perform(ev_other, ev_user0); }
}

//Reset space tracker
global.space_selected = noone;