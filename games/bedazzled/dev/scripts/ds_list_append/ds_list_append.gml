///@function ds_list_append(id1, id2, duplicate);
///@description Add the contents of list 2 on to the end of list 1, allowing duplicate entries if duplicate is true
///@param id1
///@param id2
///@param duplicate
var list1 = argument0, list2 = argument1, dupe = argument2;
if (!ds_exists(list1, ds_type_list)) { show_error("List 1 does not exist.", true); }
if (!ds_exists(list2, ds_type_list)) { show_error("List 2 does not exist.", true); }
for(var i = 0; i < ds_list_size(list2); ++i) {
	var _element = ds_list_find_value(list2, i);
	if (dupe) { ds_list_add(list1, _element); }
	else { ds_list_add_exclusive(list1, _element); }
}