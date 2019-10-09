
/// Branch and Exchange operation. Copies the operand into the pc. 
/// This flushes the pipeline and restarts it from the address specified.
/// First bit of the operand determines  
///
/// # Returns
/// a bool saying whether we are in arm or thumb mode.
/// False = ARM
/// True = THUMB 
pub fn branch_exchange(operand: u32, pc: &mut u32) -> bool {
    *pc = operand;
    return (operand & 0x01) != 0;
}