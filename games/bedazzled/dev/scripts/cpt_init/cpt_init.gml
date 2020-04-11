///@function cpt_init()
///@description Initialize Custom Path values
for (var i=NUM_PATHS - 1; i>=0; i--)
{
	cpt_start[i] = undefined;
	cpt_path[i] = ds_queue_create();
	cpt_speed_init[i] = 0;
	cpt_speed_final[i] = 0;
	cpt_time[i] = 0;
	cpt_time_end[i] = 0;
	cpt_type[i] = cpt_none;
	cpt_finish[i] = false;
}