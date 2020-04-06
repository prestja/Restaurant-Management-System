///@description Initialize
cpt_init();
image_speed = 0;
settled = false;
myText = "";
myChoice = ct_argument[1];
myPos = [x, y];
myDest = [x, y];
myPath = -1;
myState = 0;

//Set text settings
cols = (sprite_width - 2*tb_border) div font_w;
myText = string_insert_newline(ct_argument[0], cols);

//Start path
if (x < get_camera_x()) { myDest = [get_camera_l() + half_w(), y]; }
else { myDest = [get_camera_r() - half_w(), y]; }
myPath = cpt_begin(myPos, cpt_fastout, 12, 3, myDest);