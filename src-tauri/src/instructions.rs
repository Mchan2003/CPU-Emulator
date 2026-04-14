pub const OP_RALU: u32 = 0b0110011;
pub const OP_IALU: u32 = 0b0010011;
pub const OP_ILD: u32 = 0b0000011;
pub const OP_IJALR: u32 = 0b1100111;
pub const OP_SSTO: u32 = 0b0100011; 
pub const OP_BBRC: u32 = 0b1100011;
pub const OP_ULUI: u32 = 0b0110111;
pub const OP_UAUIPC: u32 = 0b0010111;
pub const OP_JJMP: u32 = 0b1101111;

pub const FT3_SUM: u32 = 0x0;
pub const FT3_SLL: u32 = 0x1;
pub const FT3_SLT: u32 = 0x2;
pub const FT3_SLTU: u32 = 0x3;
pub const FT3_XOR: u32 = 0x4;
pub const FT3_SHR: u32 = 0x5;
pub const FT3_OR: u32 = 0x6;
pub const FT3_AND: u32 = 0x7;