/// @description Update custom paths & manage state
if (cpt_update(sizePath, myPos))
{
	drawW = myPos[X];
	drawH = myPos[Y];
} else { sizePath = -1; }
if (cpt_update(scrollPath, myPos))
{
	drawTextY = myPos[Y];
} else { scrollPath = -1; }

//Start text scrolling
if (cpt_finished(sizePath))
{
	growing = false;
	if (mySpeed > 0) { alarm[0] = mySpeed; }
	else { drawText = myText; }
}

//Scroll line
if (myState == 3)
{
	if (cpt_finished(scrollPath))
	{
		//scrollPath = -1;
		myState = 0;
		drawTextY = y - boxH;
		alarm[0] = mySpeed;
	}
}

//Input 
if (global.advanceTextbox || myType == tb_brief) {
	global.advanceTextbox = false;
	if (myState == 1)
	{
		//Keep scrolling text
		lineCount = 1;
		myState = 3;
		myPos = [0, drawTextY + font_h];
		scrollPath = cpt_begin(myPos, cpt_linear, 2, 2, [0, drawTextY]);
		alarm[1] = 1;
	}
	else if (myState == 2) { 
		//Let new textboxes be created
		myState = 99;
		global.dialogueExistsFlag = false;
	}	
}