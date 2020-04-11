///@function ds_list_print(id)
///@description Return a test representation of the list.
///@param id
var list = argument0;
if (!ds_exists(list, ds_type_list)) { show_error("List does not exist.", true); }
var _str = "[\n";
for(var k=0; k<ds_list_size(list); ++k) {
	var _element = ds_list_find_value(list, k);
	_str += string(_element);
	_str += ",\n";
}
_str += "]";
return _str;