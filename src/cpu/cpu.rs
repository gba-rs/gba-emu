struct cpu();

impl cpu {

    let registers: [u32: 16] = [0:16];

    pub fn decode(instruction: u32) {
        let opcode: u16 = (((instruction >> 16) & 0xFF0) | ((instruction >> 4) & 0x0F)) as u16;
        match opcode {
            0x080 => { // ADD lli
                let format: DataProcessing = DataProcessing::from(instruction);
                },
                _ => {},
            }
    }
    pub fn fetch() -> u32 {
        return 0;
    }

    pub fn execute() {

    }

}


