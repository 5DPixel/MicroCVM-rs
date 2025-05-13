ds "hheelllloo", 0

mov r1, 0
load v3, r1

mov v4, 100
mov v5, 100
mov v0, 255
mov v1, 255
mov v2, 255

mov r2, 20 ; Kerning

call draw_character

loop:
    cmp v3, 0
    je end

    inc r1
    load v3, r1
    add v4, r2
    cmp v3, 0
    jne loop

end:
    hlt
