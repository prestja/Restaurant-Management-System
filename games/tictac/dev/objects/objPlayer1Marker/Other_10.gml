///@description Move down
myPos = [x, y];
myDest = [x, get_camera_h() - floor(1.5*sprite_height)];
myPath = cpt_begin(myPos, cpt_fastin, 2, 6, myDest);