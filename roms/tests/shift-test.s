		mov		r0, #1
		mov		r1, r0, lsl #1
		mov		r2, #8
		mov		r2, r2, lsr #1
		mov		r3, #255
		mov		r3, r3, ror #31
		mov		r4, #1
		mov		r4, r4, lsl #31
		mov		r4, r4, asr #31
infinite:
    	b       infinite
