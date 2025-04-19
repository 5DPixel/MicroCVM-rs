mov 255, v0 ; Set R to 255
mov 0, v1 ; Set G to 0
mov 0, v2 ; Set B to 0
call fill_screen
mov 255, v0 ; Set R to 255
mov 0, v1 ; Set G to 0
mov 255, v2 ; Set B to 255
mov 3, v3 ; Set Thickness
mov 73, v4 ; Set starting x to 73
mov 300, v5 ; Set starting y to 300
mov 230, v6 ; Set ending x to 230
mov 67, v7 ; Set ending y to 67
call draw_line
mov 255, v0 ; Set R to 255
mov 255, v1 ; Set G to 0
mov 255, v2 ; Set B to 255
mov 5, v3 ; Set Thickness
mov 42, v4 ; Set starting x to 73
mov 153, v5 ; Set starting y to 300
mov 80, v6 ; Set ending x to 230
mov 23, v7 ; Set ending y to 67
call draw_line
