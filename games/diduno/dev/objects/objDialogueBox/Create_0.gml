/// @description Initialize
cpt_init();
cols = 40;
rows = 4;
myType = ct_argument[0];
mySpeed = ct_argument[1];
myState = 0;
drawPos = 0;
drawLength = 1;
drawText = "";
drawLines = 1;
lineCount = 1;

//Determine width & height
growing = false;
boxW = (cols * font_w) / 2;
boxH = (rows * font_h) / 2;
if (ct_argument[2]) {
	drawW = 0;
	drawH = 0;
	growing = true;
}
else {
	drawW = boxW;
	drawH = boxH;
}
myPos = [drawW, drawH];
sizePath = cpt_begin(myPos, cpt_linear, 8, 8, [boxW, boxH]);
scrollPath = -1;

//Bound the box position
x = clamp(x, boxW + (tb_border * 2), room_width - boxW - (tb_border * 2));
y = clamp(y, boxH + (tb_border * 2), room_width - boxH - (tb_border * 2));
drawTextX = x - boxW;
drawTextY = y - boxH;

//Format text
myText = string_insert_newline(ct_argument[3], cols);
length = string_length(myText);

global.dialogueExistsFlag = true;
global.textboxDone = false;