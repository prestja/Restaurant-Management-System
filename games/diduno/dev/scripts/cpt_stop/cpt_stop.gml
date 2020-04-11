///@function cpt_stop(index)
///@description Stop the given path
///@param index
var index = argument0;
if (index >= 0 && index < NUM_PATHS)
{
	//Reset values
	cpt_start[index] = undefined;
	ds_queue_clear(cpt_path[index]);
	cpt_speed_init[index] = 0;
	cpt_speed_final[index] = 0;
	cpt_time[index] = 0;
	cpt_time_end[index] = 0;
	cpt_type[index] = cpt_none;
	
	//Set finished flag
	cpt_finish[index] = true;
}