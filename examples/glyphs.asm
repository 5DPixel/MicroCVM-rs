mov v3, 65
mov v4, 100
mov v5, 100
mov v0, 255
mov v1, 255
mov v2, 255

call draw_character
add v3, 1
add v4, 30
cmp v4, 280
jne 18
