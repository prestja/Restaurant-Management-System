/// @description Scroll text
var _next_char = true;
drawPos++;

//Skip formatting tags
var _char = string_char_at(myText, drawPos);
while (_char == op_open)
{
	//Go to the end of the tag
	while(_char != op_close)
	{
		drawPos++;
		_char = string_char_at(myText, drawPos);
	}
	
	//Go to the next char
	drawPos++;
	_char = string_char_at(myText, drawPos);
}

//Append to drawText
var _diff = drawPos - drawLength;
drawText += string_copy(myText, drawLength, _diff);
drawLength = drawPos;

//Count lines
if (_char == "\n" && drawPos <= length)
{
	drawLines++;
	lineCount++;
	
	//Scroll text up
	if (drawLines > rows && lineCount <= rows)
	{
		drawLines = rows;
		myState = 3;
		myPos = [0, drawTextY + font_h];
		scrollPath = cpt_begin(myPos, cpt_linear, 2, 2, [0, drawTextY]);
		_next_char = false;
		alarm[1] = 2;
	}
	
	//Pause box
	if (lineCount > rows) 
	{ 
		myState = 1; 
		_next_char = false;
	}
}

//End of text
if (drawPos > length)
{
	_next_char = false;
	myState = 2;
	global.textboxDone = true;
}

//Next character
if (_next_char) { alarm[0] = mySpeed; }