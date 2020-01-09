use crate::cpu::{program_status_register::ConditionFlags};

fn _add(op1: u16, op2: u16, carry_in: bool) -> (u16, ConditionFlags) {
    let output: u32 = (op1 as u32) + (op2 as u32) + (carry_in as u32);
    let carryout: bool = (output >> 16) !=0;
    let real_output: u16 = (output & 0xFFFF) as u16;
    let op1_sign: bool = (op1 >> 15) != 0;
    let op2_sign: bool = (op2 >> 15) != 0;
    let output_sign: bool = ((output >> 15) & 0x01) != 0;

    return (real_output, ConditionFlags{
        negative: (output & (0x1 << 15)) != 0,
        zero: real_output == 0,
        carry: carryout,
        signed_overflow: (op1_sign == op2_sign) && (op1_sign !=output_sign)
    });
}

pub fn add(op1: u16, op2: u16) -> (u16, ConditionFlags) {
    return _add(op1,op2, false);
}

pub fn sub(op1: u16, op2: u16) -> (u16, ConditionFlags) {
    return _add(op1, !op2, true);
}

// pub fn mul(op1: u16, op2: u16) -> (u16, ConditionFlags) {
//     let result = op1.overflowing_mul(op2);
//     let product = result.0;

//     return (product, ConditionFlags{
//         negative: (product >> 15) !=0,
//         zero: product == 0,
//         carry: false,
//         signed_overflow: result.1
//     });
// }

// pub fn mla(op1: u16, op2: u16, op3: u16) -> (u16, ConditionFlags) {

// }