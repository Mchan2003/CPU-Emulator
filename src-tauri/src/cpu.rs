use core::panic;

use crate::instructions::*;

pub struct CPU{
    pc: u32,
    regs: [u32; 32],
    ram: Vec<u8>,
}

impl CPU{
    fn new(program: Vec<u8>) -> Self {
        let mut cpu = Self {
            pc: 0,
            regs: [0; 32],       
            ram: program,
        };
        cpu.regs[0] = 0;
        cpu
    }

    fn read_register(&self, reg: usize) -> u32{
        self.regs[reg]
    }

    fn fetch(&self) -> u32{
         let index = self.pc as usize;
         return (self.ram[index] as u32) 
            | ((self.ram[index + 1] as u32) << 8)
            | ((self.ram[index + 2] as u32) << 16)
            | ((self.ram[index + 3] as u32) << 24)
    }

    fn decode(&mut self, instruction: u32){
        let opcode = instruction & 0x7f;
        match opcode{
            OP_RALU => {
                let rd = ((instruction >> 7) & 0x1f) as usize;
                let funct3= (instruction >> 12) & 0x7;
                let rs1 = ((instruction >> 15) & 0x1f) as usize;
                let rs2 = ((instruction >> 20) & 0x1f) as usize;
                let funct7 = (instruction >> 25) & 0x7f;
                self.execute_alu(rd, funct3, self.regs[rs1], self.regs[rs2], funct7);
            }
            OP_IALU => {
                let rd = ((instruction >> 7) & 0x1f) as usize;
                let funct3= (instruction >> 12) & 0x7;
                let rs1 = ((instruction >> 15) & 0x1f) as usize;
                let imm = (instruction >> 20) & 0xFFF;
                let funct7 = (instruction >> 25) & 0x7f;
                self.execute_alu(rd, funct3, self.regs[rs1], imm, funct7);
            }
            OP_ILD => {

            }
            _ => {
                panic!("Unknown opcode:{:#09b}", opcode);
            }
        }
    }

    fn execute_alu(&mut self, rd: usize,  funct3: u32, rs1_data: u32, rs2_data: u32, funct7: u32){
        match funct3{
            FT3_SUM => {
                if funct7 == 0b0000000 {
                    self.regs[rd] = rs1_data.wrapping_add(rs2_data);
                } else if funct7 == 0b0100000 {
                    self.regs[rd] = rs1_data.wrapping_sub(rs2_data);
                } else {
                    panic!("Unknown funct7:{:#09b}", funct7);
                }
            }
            FT3_SLL => {
                self.regs[rd] = rs1_data << (rs2_data & 0x1f)
            }
            FT3_SLT => {
                self.regs[rd] = if (rs1_data as i32) < (rs1_data as i32) {1} else {0};
            }
            FT3_SLTU => {
                self.regs[rd] = if rs1_data < rs1_data {1} else {0};
            }
            FT3_XOR => {
                self.regs[rd] = rs1_data ^ rs2_data;
            }
            FT3_SHR => {
                if funct7 == 0b0000000 {
                    self.regs[rd] = rs1_data >> (rs2_data & 0x1f);
                } else if funct7 == 0b0100000 {
                    self.regs[rd] = ((rs1_data as i32) >> (rs2_data & 0x1f)) as u32;
                }
            }
            FT3_OR => {
                self.regs[rd] = rs1_data | rs2_data;
            }
            FT3_AND => {
                self.regs[rd] = rs1_data & rs2_data;
            }
            _ => {
                panic!("Unknown funct3:{:#09b}", funct3);
            }
        }
    }
}

#[cfg(test)]
mod cpu_tests{
    use super::*;

    #[test]
    fn fetch_instruction(){
        let instruction: Vec<u8> = [0x93, 0x00, 0x50, 0x00].to_vec(); //addi x1, x0, 5
        let risc_v = CPU::new(instruction);
        let fetched = risc_v.fetch();

        assert_eq!(fetched, 0x00500093);
    }
    
    #[test]
    fn decode_instruction(){
        let instruction: Vec<u8> = [0x93, 0x00, 0x50, 0x00].to_vec(); //addi x1, x0, 5
        let risc_v = CPU::new(instruction);
        let fetched = risc_v.fetch();

        let rd = ((fetched >> 7) & 0x1f) as usize;
        let funct3= (fetched >> 12) & 0x7;
        let rs1 = ((fetched >> 15) & 0x1f) as usize;
        let imm = (fetched >> 20) & 0xFFF;

        assert_eq!(rd, 0x1);
        assert_eq!(funct3, FT3_SUM);
        assert_eq!(rs1, 0x0);
        assert_eq!(imm, 5);
    }

    #[test]
    fn load_imm_in_register() {
        let instruction: Vec<u8> = [0x93, 0x00, 0x50, 0x00].to_vec(); //addi x1, x0, 5
        let mut risc_v = CPU::new(instruction);
        let fetched = risc_v.fetch();
        risc_v.decode(fetched);
        
        assert_eq!(risc_v.read_register(1), 5);
    }
}

