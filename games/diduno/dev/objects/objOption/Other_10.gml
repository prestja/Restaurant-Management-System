///@description Move forward
if (x < get_camera_x()) { myDest = [x + 64, y]; }
else { myDest = [x - 64, y]; }
myPos = [x, y];
myPath = cpt_begin(myPos, cpt_fastin, 8, 4, myDest);
settled = false;