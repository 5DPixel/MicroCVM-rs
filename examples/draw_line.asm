mov v0, 255 ; Set R to 255
mov v1, 0 ; Set G to 0
mov v2, 0 ; Set B to 0
call fill_screen
mov v0, 255 ; Set R to 255
mov v1, 0 ; Set G to 0
mov v2, 255 ; Set B to 255
mov v3, 3 ; Set Thickness
mov v4, 73 ; Set starting x to 73
mov v5, 300 ; Set starting y to 300
mov v6, 230 ; Set ending x to 230
mov v7, 67 ; Set ending y to 67
call draw_line
mov v0, 255 ; Set R to 255
mov v1, 255 ; Set G to 0
mov v2, 255 ; Set B to 255
mov v3, 5 ; Set Thickness
mov v4, 42 ; Set starting x to 73
mov v5, 153 ; Set starting y to 300
mov v6, 80 ; Set ending x to 230
mov v7, 23 ; Set ending y to 67
call draw_line
