///@function wrap(value, min, max);
///@description Contains value between min and max, wrapping above and below
///@param value
///@param min
///@param max
while (argument0<argument1) //Below minimum
{
    var _diff=argument1-argument0; //Get how far below the bottom you are
    argument0=argument2-(_diff-1); //Return to the top, minus that many units
}
while (argument0>argument2) //Above maximum
{
    var _diff=argument0-argument2; //Get how far above the top you are
    argument0=argument1+(_diff-1); //Return to the bottom, plus that many units
}
return argument0;
