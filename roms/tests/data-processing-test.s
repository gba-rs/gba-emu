			bl		subs_test
			bl		rsbs_test
			bl		adds_test
			bl		movs_test
infinite:	
			b		infinite					; Infinite loop
			
adds_test:	
			mov		r0, #10						; Load 10 into r0
			mov		r1, #10						; Load 10 into r0
			adds	r0, r1, r0					; Add with flags
			add		r12, r12, #4				; Set the test output bit to true
			eoreq	r12, r12, #4				; Check Zero Not Set
			eorcs	r12, r12, #4				; Check Carry Not Set
			eormi	r12, r12, #4				; Check Negative Not Set
			eorvs	r12, r12, #4				; Check Signed Overflow Not Set
			cmp 	r0, #20						; Check add result
			eorne	r12, r12, #4				; Check Zero Set
			mov		pc, lr						; Return
			
subs_test:	
			mov		r0, #10
			mov		r1, #10
			subs	r0, r1, r0
			add		r12, r12, #1
			eorcc	r12, r12, #1
			eorne	r12, r12, #1
			eormi	r12, r12, #1
			eorvs	r12, r12, #1
			cmp		r0, #0
			eorne	r12, r12, #1
			mov		pc, lr						; Return
			
rsbs_test:	
			mov		r0, #5						; Load 5 into r0
			mov		r1, #10						; Load 10 into r1
			rsbs	r0, r1, r0					; Do r0 = r0 - r1
			add		r12, r12, #2				; Set the test output bit to true
			eorpl	r12, r12, #2				; Check Negative Set
			eoreq	r12, r12, #2				; Check Zero Not Set
			eorcs	r12, r12, #2				; Check Carry Not Set
			eorvs	r12, r12, #2				; Check Signed Overflow Not Set
			ldr		r1, RSBS_TEST_VAL			; Load the test val 
			cmp		r0, r1						; cmp result vs the test val
			eorne	r12, r12, #2				; Check the result
			mov		pc, lr						; Return
			
movs_test:	
			mov		r0, #1						; Load 1 into r0
			mov		r0, r0, lsl #31				; Load r0 logical shifted left 31 into r0
			movs	r1, r0						; move r0 into r1 with flags
			add		r12, r12, #8				; Set the test output bit to true
			eorpl	r12, r12, #8				; check Negative Set
			eoreq	r12, r12, #8				; Check Zero Not Set
			mov		r0, #0xFF000000				; Load 0xFF000000 into r0
			adds	r0, r0, #0xFF000000			; add with flags
			movs	r0, r12, lsl #0				; mov with flags (special case)
			eorcc	r12, r12, #8				; Check Carry Set
			eorMI	r12, r12, #8				; Check Negative Not Set
			mov		pc, lr						; Return

; Data Section
RSBS_TEST_VAL: .long 0xFFFFFFFB