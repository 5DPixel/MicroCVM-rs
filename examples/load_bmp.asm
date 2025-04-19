mov 255, v0
mov 255, v1
mov 255, v2
call fill_screen

mov 0, v4 ; X position
mov 0, v5 ; Y position
call load_bmp
