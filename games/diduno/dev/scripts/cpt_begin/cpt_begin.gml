///@function cpt_begin(start, type, initial_speed, final_speed, points...)
///@description Start a path with a list of points
///@param start
///@param type
///@param initial_speed
///@param final_speed
///@param points...
var start = argument[0], type = argument[1], vi = argument[2], vf = argument[3];
if (start == -1) { start = [x, y]; }
for(var i=0; i<NUM_PATHS; i++)
{
	if (cpt_type[i] == cpt_none)
	{
		//Save values
		cpt_start[i] = start;
		cpt_speed_init[i] = vi;
		cpt_speed_final[i] = vf;
		cpt_type[i] = type;
		
		//Calculate total distance
		var _D = 0;
		var _p1 = [start[X], start[Y]];
		for(var p=4; p<argument_count; p++) 
		{ 
			var _p2 = argument[p];
			ds_queue_enqueue(cpt_path[i], _p2); 
			_D += point_to_point(_p1[X], _p1[Y], _p2[X], _p2[Y]);
			_p1 = _p2;
		}
		
		//Calculate end time
		switch(type)
		{
			case cpt_linear:
				cpt_time_end[i] = _D / cpt_speed_init[i];
			break;
			case cpt_fastin:
			case cpt_fastout:
			case cpt_fastmid:
			case cpt_slowmid:
				cpt_time_end[i] = (2*_D) / (cpt_speed_final[i] + cpt_speed_init[i]);
			break;
		}
		
		//Return index
		return i;
	}
}

//No more open path slots
return -1;