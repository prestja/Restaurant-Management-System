///@description Initialize
cpt_init();
image_speed = 0;
settled = false;
myText = ct_argument[0];
myChoice = ct_argument[1];
myPos = [x, y];
myDest = [x, y];
myPath = -1;
myState = 0;

//Start path
if (x < 0) { myDest = [0, y]; }
else { myDest = [room_width - sprite_width, y]; }
myPath = cpt_begin(myPos, cpt_fastout, 14, 2, myDest);