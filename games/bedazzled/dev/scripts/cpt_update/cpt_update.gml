///@function cpt_update(index, pos)
///@description Update the given path & save the new coordinates in pos
///@param index
///@param pos
var index = argument0, pos = argument1;
if (cpt_running(index))
{
	//Increment time
	if (cpt_time[index] == 0) 
	{ 
		var _s = cpt_start[index];
		pos[@ X] = _s[X]; 
		pos[@ Y] = _s[Y]; 
	}
	cpt_time[index]++;
	
	//Get destination point
	var _dest = ds_queue_head(cpt_path[index]);
	var _d = point_to_point(pos[@ X], pos[@ Y], _dest[X], _dest[Y]);
	var _T = cpt_time_end[index];
	var _t = cpt_time[index];
	var _Vi = cpt_speed_init[index];
	var _Vf = cpt_speed_final[index];
	var _V = 0;
	
	//Calculate speed
	switch(cpt_type[index])
	{
		case cpt_linear:
			_V = _Vi;
		break;
		case cpt_fastin:
			_V = (((_Vi - _Vf) / _T)*_t) + _Vf;
		break;
		case cpt_fastout:
			_V = (((_Vf - _Vi) / _T)*_t) + _Vi;
		break;
		case cpt_fastmid:
			if (_t <= (_T/2))
			{
				_V = ((2*(_Vf - _Vi) / _T)*_t) + _Vi;
			}
			else
			{
				_V = ((2*(_Vi - _Vf) / _T)*(_t - (_T/2))) + _Vf;
			}
		break;
		case cpt_slowmid:
			if (_t <= (_T/2))
			{
				_V = ((2*(_Vi - _Vf) / _T)*_t) + _Vf;
			}
			else
			{
				_V = ((2*(_Vf - _Vi) / _T)*(_t - (_T/2))) + _Vi;
			}
		break;
	}
	//show_message("T = "+string(_T)+"\nt = "+string(_t)+"\nv = "+string(_V)+"\nd = "+string(_d));
	
	//If the remaining distance is less than the calculated speed,
	if (_d < abs(_V))
	{
		//Snap to destination
		pos[@ X] = _dest[X];
		pos[@ Y] = _dest[Y];
		ds_queue_dequeue(cpt_path[index]);
		
		//If that was the last point, end the path
		if (ds_queue_empty(cpt_path[index])) 
		{ 
			cpt_stop(index); 
			return true;
		}
		//Otherwise, set the next destination point
		else
		{
			_dest = ds_queue_head(cpt_path[index]);
			_V -= _d;
		}
	}
	
	//Move towards destination
	var _A = point_direction(pos[@ X], pos[@ Y], _dest[X], _dest[Y]);
	pos[@ X] += lengthdir_x(_V, _A);
	pos[@ Y] += lengthdir_y(_V, _A);
	
	//Successful move
	return true;
}
else
{
	if (cpt_finished(index)) { cpt_finish[index] = false; }
	return false;
}