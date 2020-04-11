///@function ds_list_add_exclusive(id, val1, [val2 ... max_val]);
///@descirption Only add the values to the list if they do not already exist in it
///@param id
///@param val1
///@param [val2...max_val]
var list_id = argument[0];
if (!ds_exists(list_id, ds_type_list)) { show_error("List does not exist.", true); }
if (argument_count < 2) { show_error("Too few arguments provided.", true); }
for(var i = 1; i < argument_count; ++i) {
	if (ds_list_find_index(list_id, argument[i]) == -1) { ds_list_add(list_id, argument[i]); }
}