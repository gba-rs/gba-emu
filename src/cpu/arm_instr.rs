use super::{cpu::InstructionFormat};

pub const arm_instructions: [InstructionFormat; 4096] = [
    // 0000 0000 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Multiply, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer,

// 0000 0001 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Multiply, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0000 0010 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Multiply, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,

// 0000 0011 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Multiply, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,

// 0000 0100 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer,

// 0000 0101 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0000 0110 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,

// 0000 0111 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,

// 0000 1000 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::MultiplyLong, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer,

// 0000 1001 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::MultiplyLong, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0000 1010 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::MultiplyLong, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,

// 0000 1011 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::MultiplyLong, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,

// 0000 1100 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::MultiplyLong, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer,

// 0000 1101 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::MultiplyLong, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0000 1110 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::MultiplyLong, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,

// 0000 1111 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::MultiplyLong, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::Undefiend,

// 0001 0000 0000
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Multiply, InstructionFormat::SingleDataSwap, InstructionFormat::Multiply, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::Multiply, InstructionFormat::SingleDataTransfer, InstructionFormat::Multiply, InstructionFormat::SingleDataTransfer,

// 0001 0001 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0001 0010 0000
InstructionFormat::DataProcessing, InstructionFormat::BranchAndExchange, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Multiply, InstructionFormat::Undefiend, InstructionFormat::Multiply, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::Multiply, InstructionFormat::SingleDataTransfer, InstructionFormat::Multiply, InstructionFormat::SingleDataTransfer,

// 0001 0011 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0001 0100 0000
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::MultiplyLong, InstructionFormat::SingleDataSwap, InstructionFormat::MultiplyLong, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::MultiplyLong, InstructionFormat::SingleDataTransfer, InstructionFormat::MultiplyLong, InstructionFormat::SingleDataTransfer,

// 0001 0101 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0001 0110 0000
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Multiply, InstructionFormat::Undefiend, InstructionFormat::Multiply, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::Multiply, InstructionFormat::SingleDataTransfer, InstructionFormat::Multiply, InstructionFormat::SingleDataTransfer,

// 0001 0111 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0001 1000 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer,

// 0001 1001 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0001 1010 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer,

// 0001 1011 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0001 1100 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer,

// 0001 1101 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0001 1110 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::SingleDataTransfer,

// 0001 1111 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::Undefiend, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,
InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer, InstructionFormat::DataProcessing, InstructionFormat::HalfwordDataTransfer,

// 0010 0000 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 0001 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 0010 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 0011 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 0100 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 0101 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 0110 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 0111 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 1000 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 1001 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 1010 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 1011 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 1100 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 1101 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 1110 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0010 1111 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 0000 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 0011 0001 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 0010 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 0011 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 0100 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 0011 0101 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 0110 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 0111 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 1000 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 1001 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 1010 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 1011 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 1100 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 1101 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 1110 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0011 1111 0000
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,
InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing, InstructionFormat::DataProcessing,

// 0100 0000 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 0001 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 0010 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 0011 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 0100 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 0101 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 0110 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 0111 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 1000 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 1001 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 1010 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 1011 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 1100 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 1101 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 1110 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0100 1111 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 0000 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 0001 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 0010 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 0011 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 0100 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 0101 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 0110 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 0111 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 1000 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 1001 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 1010 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 1011 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 1100 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 1101 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 1110 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0101 1111 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,
InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer, InstructionFormat::SingleDataTransfer,

// 0110 0000 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 0001 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 0010 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 0011 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 0100 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 0101 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 0110 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 0111 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 1000 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 1001 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 1010 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 1011 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 1100 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 1101 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 1110 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0110 1111 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 0000 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 0001 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 0010 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 0011 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 0100 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 0101 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 0110 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 0111 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 1000 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 1001 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 1010 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 1011 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 1100 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 1101 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 1110 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 0111 1111 0000
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,
InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend, InstructionFormat::SingleDataTransfer, InstructionFormat::Undefiend,

// 1000 0000 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 0001 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 0010 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 0011 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 0100 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 0101 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 0110 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 0111 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 1000 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 1001 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 1010 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 1011 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 1100 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 1101 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 1110 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1000 1111 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 0000 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 0001 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 0010 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 0011 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 0100 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 0101 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 0110 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 0111 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 1000 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 1001 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 1010 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 1011 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 1100 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 1101 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 1110 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1001 1111 0000
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,
InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer, InstructionFormat::BlockDataTransfer,

// 1010 0000 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 0001 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 0010 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 0011 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 0100 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 0101 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 0110 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 0111 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 1000 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 1001 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 1010 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 1011 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 1100 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 1101 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 1110 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1010 1111 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 0000 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 0001 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 0010 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 0011 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 0100 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 0101 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 0110 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 0111 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 1000 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 1001 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 1010 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 1011 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 1100 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 1101 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 1110 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1011 1111 0000
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,
InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch, InstructionFormat::Branch,

// 1100 0000 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 0001 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 0010 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 0011 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 0100 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 0101 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 0110 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 0111 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 1000 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 1001 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 1010 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 1011 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 1100 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 1101 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 1110 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1100 1111 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 0000 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 0001 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 0010 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 0011 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 0100 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 0101 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 0110 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 0111 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 1000 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 1001 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 1010 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 1011 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 1100 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 1101 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 1110 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1101 1111 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 0000 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 0001 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 0010 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 0011 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 0100 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 0101 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 0110 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 0111 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 1000 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 1001 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 1010 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 1011 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 1100 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 1101 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 1110 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1110 1111 0000
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,
InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend, InstructionFormat::Undefiend,

// 1111 0000 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 0001 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 0010 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 0011 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 0100 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 0101 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 0110 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 0111 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 1000 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 1001 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 1010 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 1011 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 1100 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 1101 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 1110 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

// 1111 1111 0000
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,
InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt, InstructionFormat::SoftwareInterrupt,

];
