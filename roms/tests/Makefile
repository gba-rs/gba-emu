roms  = fib.rom shift-test.rom data-processing-test.rom mul-test.rom

all: $(roms)

clean:
	rm -f $(roms)
	
$(roms): %.rom: %.s
	vasmarm_std $< -Fbin -o $@