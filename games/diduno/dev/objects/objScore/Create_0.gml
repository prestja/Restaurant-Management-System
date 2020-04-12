///@description Initialize
cpt_init();
myVal = ct_argument[0];
myPos = [x, y];
myDest = [x, y - 96];
myPath = cpt_begin(myPos, cpt_fastout, 3, 1, myDest);