mov 0, v4
mov 0, v5
call load_bmp

mov v0, 255
mov v1, 0
mov v2, 0

mov v4, 100
mov v5, 100

mov v6, 80
call fill_rect
