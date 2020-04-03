///@description Motion & Input
if (!dead) {
	//Input
	if (device_mouse_check_button(0, mb_left) && jump_counter < jump_timer) {
		jump_counter++;
		vspeed -= jump_boost;
	}
	if (!device_mouse_check_button(0, mb_left) && jump_counter > 0) { jump_counter--; }

	//Gravity
	vspeed = approach(vspeed, jump_terminal, jump_gravity);

	//Change angle
	if (vspeed < 0) { 
		if (image_angle < 30 || image_angle >= 330) { image_angle = wrap(image_angle + 2, 0, 359); }
	}
	else {
		if (image_angle <= 32 || image_angle > 330) { image_angle = wrap(image_angle - 1, 0, 359);; }
	}

	//Keep inside level bounds
	if (y < 0) { y = 0; }
	if (y > room_height) { event_perform(ev_other, ev_user0); }
}
else {
	//Speeeeeeeeeeen
	image_angle += 10;
	
	//Weaker gravity
	vspeed = approach(vspeed, jump_terminal, jump_gravity*0.5);
	
	if (y > room_height + half_h()) { instance_destroy(); }
}