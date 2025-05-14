; === Initialization ===
mov v4, 10         ; x position
mov v5, 30         ; y position
mov v0, 255        ; color R
mov v1, 255        ; color G
mov v2, 255        ; color B

mov r2, 10         ; kerning (character spacing)
mov r4, 32         ; space keycode (ASCII)
mov r5, 8          ; backspace keycode (ASCII)
mov r6, 0          ; underscore_drawn = 0

; === Main Loop ===
loop:
    cmp k0, 0
    je no_key

    mov r6, 0           ; reset underscore_drawn

    cmp k0, r5
    je handle_backspace

    cmp k0, r4
    je handle_space     ; skip rendering, but still move forward

    mov v3, k0
    call draw_character

    add v4, r2
    jmp wait_release

; === Handle No Key Press ===
no_key:
    cmp r6, 1
    je loop

    mov v3, 95
    call draw_character
    mov r6, 1
    jmp loop

; === Handle Backspace ===
handle_backspace:
    sub v4, r2
    call clear_screen
    jmp wait_release

; === Handle Space ===
handle_space:
    add v4, r2
    mov r6, 1
    jmp wait_release

; === Wait for Key Release ===
wait_release:
    cmp k0, 0
    jne wait_release
    jmp loop
