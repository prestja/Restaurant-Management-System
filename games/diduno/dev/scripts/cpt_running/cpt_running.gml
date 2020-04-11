///@function cpt_running(index)
///@description Return if the given path is currently running
///@param index
var index = argument0;
if (index >= 0 && index < NUM_PATHS) { return !(cpt_type[index] == cpt_none); }
else { return false; }