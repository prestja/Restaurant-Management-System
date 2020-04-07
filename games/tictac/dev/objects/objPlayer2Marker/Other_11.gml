///@description Move up
myPos = [x, y];
myDest = [x, get_camera_h() - floor(1.5*sprite_height) - 64];
myPath = cpt_begin(myPos, cpt_fastin, 2, 6, myDest);