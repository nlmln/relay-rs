# WebSocket communication structure  
a ws message may use the structure `ws://$RELAY_HOST/$ACTION/$INPUT_VARIABLE` if an input is required for the action to be taken or simply `ws://$RELAY_HOST/$ACTION` if no input is required.  
  
**supported messages are currently:**  
- `ws://$RELAY_HOST/add_time/$LANE`  
- `ws://$RELAY_HOST/subtract_time/$LANE`  
- `ws://$RELAY_HOST/reset_lane/$LANE`  
- `ws://$RELAY_HOST/reset_all`  
- `ws://$RELAY_HOST/status`  
> $RELAY_HOST refers to the IP of the Pi running relay_rs, $ACTION refers to the action that the relay_rs server should take, $LANE refers to the number (by default valid numbers are: 1, 2, 3, 4) corresponding to the lane the action should affect.  
  
**example responses will look like this:**  
  
```
{
  "success": "true",
}
```  
> for actions `add_time, subtract_time, reset_lane and reset_all`  

or  
```
{
    "timers": ["00:15:00", "00:00:00", "01:23:57", "00:00:57"],
    "active": [true, false, true, true]
}
```  
> for action `status`  
  
## NOTE:  
*when creating a UI, don't forget that websockets unlike most web protocols are real time, two way interaction, long lived connections, as such, once opened even when idling, the connection will stay alive. this design decision was taken because in the previous implementation status was queried every second and that meant opening a connection to the pi every second for every client which is incredibly inefficent.*  
