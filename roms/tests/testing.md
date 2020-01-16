# Running test roms
## Shift Test
### Expected Output
- r0: 0x1
- r1: 0x2
- r2: 0x4
- r3: 0x1FE
- r4: 0xFFFFFFFF
- Rest are not used
### What is being tested
- r0 is the output for a mov with no shift
- r1 is the output for a `lsl 1`
- r2 is the output for a `lsr 1`
- r3 is the output for a `ror 31`
- r4 is the output of a `lsl 31` and then an `asr 31`