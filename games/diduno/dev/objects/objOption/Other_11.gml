///@description Move offscreen
if (x < (room_width / 2)) { myDest = [-sprite_width - 64, y]; }
else { myDest = [room_width + 64, y]; }
myPos = [x, y];
myPath = cpt_begin(myPos, cpt_fastout, 8, 4, myDest);
settled = false;