///@description Move offscreen
if (x < get_camera_x()) { myDest = [get_camera_l() - sprite_width, y]; }
else { myDest = [get_camera_r() + sprite_width, y]; }
myPos = [x, y];
myPath = cpt_begin(myPos, cpt_fastout, 8, 4, myDest);
settled = false;