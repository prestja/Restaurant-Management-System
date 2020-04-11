///@function cpt_finished(index)
///@description Return if the given path has finished running
///@param index
var index = argument0;
if (index >= 0 && index < NUM_PATHS) { return cpt_finish[index]; }
else { return false; }