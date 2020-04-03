///@function instance_create_v(x, y, layer, obj, [...arg])
///@description Creates an object, passing optional arguments to it's Create event
///@param x
///@param y
///@param layer
///@param obj
///@param [...arg]
//Script design by @YellowAfterlife
//Build argument array
var n = argument_count - 4;
ct_argument = undefined;
ct_count = n;
for(var i=0; i<n; i++) { ct_argument[i] = argument[4 + i]; }
var r = instance_create_layer(argument[0], argument[1], argument[2], argument[3]);

//Reset arrays
ct_argument = undefined;
ct_count = undefined;
return r;