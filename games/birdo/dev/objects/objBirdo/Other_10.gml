///@description Die
dead = true;
vspeed = -3 * jump_boost;
with(objWall) { hspeed = 0; }
with(objPoints) { hspeed = 0; }
with(objGame) { event_perform(ev_other, ev_user0); }