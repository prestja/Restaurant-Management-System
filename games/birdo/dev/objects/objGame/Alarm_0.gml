///@description Make walls
instance_create_layer(0, 0, "InstanceActors", objWallSpawn);
wall_timer = 40;
if (global.game_score < 50) { wall_timer = 80; }
if (global.game_score < 10) { wall_timer = 120; }
alarm[0] = wall_timer;