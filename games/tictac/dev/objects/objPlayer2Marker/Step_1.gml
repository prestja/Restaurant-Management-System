/// @description Update custom paths
if (cpt_update(myPath, myPos))
{
	x = myPos[X];
	y = myPos[Y];
} else { myPath = -1; }