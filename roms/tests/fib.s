setup:
    mov     r0, #20
    mov     r1, #0
    mov     r2, #1
    mov     r3, #0

    mov     r4, #1

loop:
    cmp     r4, #1
    moveq   r10, #1
    cmp     r4, #2
    moveq   r10, #2
    add     r3, r1, r2
    mov     r1, r2
    mov     r2, r3
    mov     r10, r3
    add     r4, r4, #1
    cmp     r4, r0
    bne     loop
infinite:
    b       infinite