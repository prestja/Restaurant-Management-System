///@description Initialize
goto = get_camera_y() + (sprite_get_height(sprBackgroundGrid)/2) - sprite_get_height(sprSpace)
cpt_init();
image_speed = 0;
image_index = 1;
myPos = [x, y];
myDest = [x, y];
myPath = -1;
event_perform(ev_other, ev_user1);