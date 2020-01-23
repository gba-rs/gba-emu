            bl       mul_test
infinite:	
			b		infinite					; Infinite loop

mul_test:
            mov     r0, #5
            mov     r1, #5
            add     r12, r12, #1                ; Setup test bit
            mul     r2, r0, r1                  ; 5 x 5
            cmp     r2, #25                     ; Check for 25
            eorne   r12, r12, #1                ; Check for Zero Flag
            mov     pc, lr                      ; Return