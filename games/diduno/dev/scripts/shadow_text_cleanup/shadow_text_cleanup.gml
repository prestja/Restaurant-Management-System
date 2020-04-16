///@function string_controller_cleanup()
///@description Destroy the shadow text buffer if it exists
if (ds_exists(global.textBuffer, ds_type_queue)) 
{ 
	//debug_log("[SETUP] Destroying shadow text buffer in room "+room_get_name(room));
	ds_queue_destroy(global.textBuffer); 
	//debug_log("[SETUP] Shadow text buffer destroyed successfully");
}