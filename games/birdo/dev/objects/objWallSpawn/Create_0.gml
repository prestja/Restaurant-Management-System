///@description Create walls & points
//Decide how far apart the walls will be
min_dist = 1;
if (global.game_score < 50) { min_dist = 2; }
if (global.game_score < 10) { min_dist = 3; }
max_dist = 3;
if (global.game_score < 50) { max_dist = 5; }
if (global.game_score < 10) { max_dist = 8; }
wall_dist = (irandom_range(min_dist, max_dist))*32;

//Decide opening height
wall_height = irandom_range(get_camera_t() + 84, get_camera_b() - 84);

//Create objects & give them motion
var _wall1 = instance_create_layer(get_camera_r() + 64, wall_height + (wall_dist / 2), layer, objWall);
var _wall2 = instance_create_layer(get_camera_r() + 64, wall_height - (wall_dist / 2), layer, objWall);
var _points = instance_create_layer(get_camera_r() + 64, 0, layer, objPoints);
_wall1.hspeed = -2;
_wall2.hspeed = -2;
_wall2.sprite_index = sprWallTop;
_points.hspeed = -2;

instance_destroy();