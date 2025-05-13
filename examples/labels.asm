test:
    mov r1, 'A'

jmp test

mov v3, r1
mov v4, 100
mov v5, 100
mov v0, 255
mov v1, 255
mov v2, 255

call draw_character
hlt
