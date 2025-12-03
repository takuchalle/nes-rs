use core::panic;

use crate::opcodes;
use bit_field::BitField;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct CPU {
    pub pc: u16,
    pub reg_a: u8,
    pub sp: u8,
    pub index_reg_x: u8,
    pub index_reg_y: u8,
    pub status: u8,
    memory: [u8; 0xFFFF],
}

const NEGATIVE_BIT: usize = 7;
const MSB: usize = 7;

const STATUS_BIT_N: usize = 7;
const STATUS_BIT_V: usize = 6;
// const STATUS_BIT_B: usize = 4;
const STATUS_BIT_D: usize = 3;
const STATUS_BIT_I: usize = 2;
const STATUS_BIT_Z: usize = 1;
const STATUS_BIT_C: usize = 0;

const STACK_RESET: u8 = 0xfd;
const STACK_BASE: u16 = 0x100;

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            reg_a: 0,
            sp: STACK_RESET,
            index_reg_x: 0,
            index_reg_y: 0,
            status: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 1) as u16;
        hi << 8 | lo
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let lo = (data & 0xFF) as u8;
        let hi = (data >> 8 & 0xFF) as u8;
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.index_reg_x = 0;
        self.status = 0;
        self.sp = STACK_RESET;

        self.pc = self.mem_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: &[u8]) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: &[u8]) {
        self.memory[0x0600..(0x0600 + program.len())].copy_from_slice(program);
        self.mem_write_u16(0xFFFC, 0x0600);
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| Ok(())).unwrap();
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F) -> std::io::Result<()>
    where
        F: FnMut(&mut CPU) -> std::io::Result<()>,
    {
        let opcodes = &opcodes::OPCODES_MAP;
        loop {
            callback(self)?;

            let code = self.mem_read(self.pc);
            self.pc += 1;
            let pc_state = self.pc;
            let opcode = match opcodes.get(&code) {
                Some(o) => o,
                None => return Err(std::io::Error::from(std::io::ErrorKind::Unsupported)),
            };

            match code {
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => {
                    self.ldx(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => {
                    self.ldy(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0x86 | 0x96 | 0x8e => {
                    self.stx(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0x84 | 0x94 | 0x8c => {
                    self.sty(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => {
                    self.and(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0x0a => {
                    self.asl_accumulator();
                    self.pc += (opcode.len - 1) as u16;
                }

                0x06 | 0x16 | 0x0e | 0x1e => {
                    self.asl(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0x4a => self.lsr_accumulator(),

                0x46 | 0x56 | 0x4e | 0x5e => {
                    self.lsr(&opcode.mode);
                }

                0x90 => {
                    self.branch(!self.status.get_bit(STATUS_BIT_C));
                }

                0xb0 => {
                    self.branch(self.status.get_bit(STATUS_BIT_C));
                }

                0xf0 => {
                    self.branch(self.status.get_bit(STATUS_BIT_Z));
                }

                0x30 => {
                    self.branch(self.status.get_bit(STATUS_BIT_N));
                }

                0xd0 => {
                    self.branch(!self.status.get_bit(STATUS_BIT_Z));
                }

                0x10 => {
                    self.branch(!self.status.get_bit(STATUS_BIT_N));
                }

                0x50 => {
                    self.branch(!self.status.get_bit(STATUS_BIT_V));
                }

                0x70 => {
                    self.branch(self.status.get_bit(STATUS_BIT_V));
                }

                0x24 | 0x2c => {
                    self.bit(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {
                    self.cmp(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0xe0 | 0xe4 | 0xec => {
                    self.cpx(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0xc0 | 0xc4 | 0xcc => {
                    self.cpy(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0xc6 | 0xd6 | 0xce | 0xde => {
                    self.dec(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0xe6 | 0xf6 | 0xee | 0xfe => {
                    self.inc(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0xca => {
                    self.dex();
                    self.pc += (opcode.len - 1) as u16;
                }

                0x88 => {
                    self.dey();
                    self.pc += (opcode.len - 1) as u16;
                }

                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => {
                    self.eor(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => {
                    self.ora(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                    self.sbc(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0x2a => self.rol_accumulator(),
                0x26 | 0x36 | 0x2e | 0x3e => {
                    self.rol(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                0x6a => self.ror_accumulator(),
                0x66 | 0x76 | 0x6e | 0x7e => {
                    self.ror(&opcode.mode);
                    self.pc += (opcode.len - 1) as u16;
                }

                /* Clear */
                0x18 => {
                    self.status.set_bit(STATUS_BIT_C, false);
                }
                0xd8 => {
                    self.status.set_bit(STATUS_BIT_D, false);
                }
                0x58 => {
                    self.status.set_bit(STATUS_BIT_I, false);
                }
                /* Set */
                /* Carry flag */
                0x38 => {
                    self.status.set_bit(STATUS_BIT_C, true);
                }
                /* Decimal flag */
                0xf8 => {
                    self.status.set_bit(STATUS_BIT_D, true);
                }
                /* Interrupt Disable */
                0x78 => {
                    self.status.set_bit(STATUS_BIT_I, true);
                }
                0xAA => self.tax(),
                0x8A => self.txa(),
                0xE8 => self.inx(),
                0xc8 => self.iny(),
                0x20 => self.jsr(),

                /* JMP Absolute */
                0x4c => {
                    let addr = self.mem_read_u16(self.pc);
                    self.pc = addr;
                }

                /* JMP Indirect */
                0x6c => {
                    let addr = self.mem_read_u16(self.pc);

                    let indirect_ref = if addr & 0x00FF == 0x00FF {
                        let lo = self.mem_read(addr);
                        let hi = self.mem_read(addr & 0xFF00);
                        (hi as u16) << 8 | (lo as u16)
                    } else {
                        self.mem_read_u16(addr)
                    };

                    self.pc = indirect_ref;
                }

                0x40 => self.rti(),
                0x60 => self.rts(),
                0x48 => self.stack_push(self.reg_a),
                0x08 => self.stack_push(self.status),
                0x68 => self.reg_a = self.stack_pop(),
                0x28 => self.status = self.stack_pop(),
                0xea => {} // NOP
                0x00 => {
                    return Ok(());
                }
                _ => todo!(),
            }

            if pc_state == self.pc {
                self.pc += (opcode.len - 1) as u16;
            }
        }
    }

    fn stack_pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.mem_read(STACK_BASE + self.sp as u16)
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop();
        let hi = self.stack_pop();
        (hi as u16) << 8 | lo as u16
    }

    fn stack_push(&mut self, data: u8) {
        self.mem_write(STACK_BASE + self.sp as u16, data);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, data: u16) {
        let hi = ((data & 0xFF00) >> 8) as u8;
        let lo = (data & 0x00FF) as u8;

        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.pc,
            AddressingMode::ZeroPage => self.mem_read(self.pc) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.pc),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.pc);
                pos.wrapping_add(self.index_reg_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.pc);
                pos.wrapping_add(self.index_reg_y) as u16
            }
            AddressingMode::Absolute_X => {
                let pos = self.mem_read_u16(self.pc);
                pos.wrapping_add(self.index_reg_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let pos = self.mem_read_u16(self.pc);
                pos.wrapping_add(self.index_reg_y as u16)
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.pc);

                let ptr = base.wrapping_add(self.index_reg_x);
                let lo = self.mem_read(ptr as u16) as u16;
                let hi = self.mem_read(ptr.wrapping_add(1) as u16) as u16;
                hi << 8 | lo
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.pc);
                let lo = self.mem_read(base as u16) as u16;
                let hi = self.mem_read(base.wrapping_add(1) as u16) as u16;

                let deref_base = hi << 8 | lo;
                deref_base.wrapping_add(self.index_reg_y as u16)
            }
            AddressingMode::NoneAddressing => panic!(""),
        }
    }

    fn update_zero_and_negative_flags(&mut self, reg: u8) {
        self.status.set_bit(STATUS_BIT_Z, reg == 0);
        self.status.set_bit(STATUS_BIT_N, reg.get_bit(NEGATIVE_BIT));
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.reg_a = self.mem_read(addr);
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.index_reg_x = self.mem_read(addr);
        self.update_zero_and_negative_flags(self.index_reg_x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.index_reg_y = self.mem_read(addr);
        self.update_zero_and_negative_flags(self.index_reg_y);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.reg_a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.index_reg_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.index_reg_y);
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let c = u16::from(self.status.get_bit(STATUS_BIT_C));

        let result = u16::from(value) + u16::from(self.reg_a) + c;

        self.status.set_bit(STATUS_BIT_C, result > 0xFF);

        let result = (result & 0xFF) as u8;
        self.status.set_bit(
            STATUS_BIT_V,
            ((result ^ value) & (result ^ self.reg_a) & 0x80) != 0,
        );

        self.reg_a = result;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    // A - B - (1 - C) = A + (-B) - 1 + C = A + (-B - 1) + C
    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let c = u16::from(self.status.get_bit(STATUS_BIT_C));
        let value = (value as i8).wrapping_neg().wrapping_sub(1) as u8;

        let result = u16::from(value) + u16::from(self.reg_a) + c;

        self.status.set_bit(STATUS_BIT_C, result > 0xFF);

        let result = (result & 0xFF) as u8;
        self.status.set_bit(
            STATUS_BIT_V,
            ((result ^ value) & (result ^ self.reg_a) & 0x80) != 0,
        );

        self.reg_a = result;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.reg_a &= self.mem_read(addr);
        self.update_zero_and_negative_flags(self.reg_a);
    }

    /* Arithmetic Shift Left */
    fn asl_accumulator(&mut self) {
        self.status.set_bit(STATUS_BIT_C, self.reg_a.get_bit(MSB));
        self.reg_a <<= 1;
        self.update_zero_and_negative_flags(self.reg_a);
    }
    fn asl(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);
        self.status.set_bit(STATUS_BIT_C, value.get_bit(MSB));
        value <<= 1;
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn lsr_accumulator(&mut self) {
        let mut value = self.reg_a;
        self.status.set_bit(STATUS_BIT_C, value.get_bit(0));
        value >>= 1;
        self.reg_a = value;
        self.update_zero_and_negative_flags(value);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);
        self.status.set_bit(STATUS_BIT_C, value.get_bit(0));
        value >>= 1;
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn tax(&mut self) {
        self.index_reg_x = self.reg_a;
        self.update_zero_and_negative_flags(self.index_reg_x);
    }

    fn txa(&mut self) {
        self.reg_a = self.index_reg_x;
        self.update_zero_and_negative_flags(self.reg_a);
    }
    fn inx(&mut self) {
        self.index_reg_x = self.index_reg_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.index_reg_x);
    }

    fn iny(&mut self) {
        self.index_reg_y = self.index_reg_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.index_reg_y);
    }

    fn branch(&mut self, c: bool) {
        if c {
            let jump = self.mem_read(self.pc) as i8;
            let value = self.pc.wrapping_add(1).wrapping_add(jump as u16);
            self.pc = value;
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = self.reg_a & value;
        self.status.set_bit(STATUS_BIT_Z, result == 0x0);
        self.status.set_bit(STATUS_BIT_V, value.get_bit(6));
        self.status.set_bit(STATUS_BIT_N, value.get_bit(7));
    }

    fn cmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = self.reg_a.wrapping_sub(value);
        self.status.set_bit(STATUS_BIT_Z, self.reg_a == value);
        self.status.set_bit(STATUS_BIT_C, self.reg_a >= value);
        self.status.set_bit(STATUS_BIT_N, result.get_bit(MSB));
    }

    fn cpx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = self.index_reg_x.wrapping_sub(value);
        self.status.set_bit(STATUS_BIT_Z, self.index_reg_x == value);
        self.status.set_bit(STATUS_BIT_C, self.index_reg_x >= value);
        self.status.set_bit(STATUS_BIT_N, result.get_bit(MSB));
    }

    fn cpy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = self.index_reg_y.wrapping_sub(value);
        self.status.set_bit(STATUS_BIT_Z, self.index_reg_y == value);
        self.status.set_bit(STATUS_BIT_C, self.index_reg_y >= value);
        self.status.set_bit(STATUS_BIT_N, result.get_bit(MSB));
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);
        value = value.wrapping_sub(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);
        value = value.wrapping_add(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn dex(&mut self) {
        self.index_reg_x = self.index_reg_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.index_reg_x);
    }

    fn dey(&mut self) {
        self.index_reg_y = self.index_reg_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.index_reg_y);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.reg_a ^= value;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.reg_a |= value;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn rol_accumulator(&mut self) {
        let old = self.reg_a;
        let mut value = old << 1;
        self.status.set_bit(STATUS_BIT_C, old.get_bit(MSB));
        value.set_bit(0, old.get_bit(MSB));
        self.reg_a = value;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let old = self.mem_read(addr);
        let mut value = old << 1;
        self.status.set_bit(STATUS_BIT_C, old.get_bit(MSB));
        value.set_bit(0, old.get_bit(MSB));
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn ror_accumulator(&mut self) {
        let old = self.reg_a;
        let mut value = old >> 1;
        self.status.set_bit(STATUS_BIT_C, old.get_bit(0));
        value.set_bit(MSB, old.get_bit(0));
        self.reg_a = value;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let old = self.mem_read(addr);
        let mut value = old >> 1;
        self.status.set_bit(STATUS_BIT_C, old.get_bit(0));
        value.set_bit(MSB, old.get_bit(0));
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn jsr(&mut self) {
        self.stack_push_u16(self.pc + 2 - 1);
        self.pc = self.mem_read_u16(self.pc);
    }

    fn rti(&mut self) {
        self.status = self.stack_pop();
        self.pc = self.stack_pop_u16();
    }

    fn rts(&mut self) {
        self.pc = self.stack_pop_u16() + 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_lda_from_zero_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);
        cpu.load_and_run(&vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.reg_a, 0x55);
    }

    #[test]
    fn test_lda_from_absolute_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x55);
        cpu.load_and_run(&vec![0xad, 0x00, 0x10, 0x00]);
        assert_eq!(cpu.reg_a, 0x55);
    }

    #[test]
    fn test_sta_to_zero_memory() {
        let mut cpu = CPU::new();
        cpu.load(&vec![0x85, 0x10, 0x00]);
        cpu.reset();
        cpu.reg_a = 0x55;
        cpu.run();
        assert_eq!(cpu.mem_read(0x10), 0x55);
    }

    #[test]
    fn test_sta_to_zero_x_memory() {
        let mut cpu = CPU::new();
        cpu.load(&vec![0x95, 0x10, 0x00]);
        cpu.reset();
        cpu.reg_a = 0x55;
        cpu.index_reg_x = 0x01;
        cpu.run();
        assert_eq!(cpu.mem_read(0x11), 0x55);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load(&vec![0xaa, 0x00]);
        cpu.reset();
        cpu.reg_a = 10;
        cpu.run();
        assert!(cpu.index_reg_x == 10);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x_zero_flg() {
        let mut cpu = CPU::new();
        cpu.load(&vec![0xaa, 0x00]);
        cpu.reset();
        cpu.reg_a = 0;
        cpu.run();
        assert!(cpu.index_reg_x == 0);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.index_reg_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load(&vec![0xe8, 0xe8, 0x00]);
        cpu.reset();
        cpu.index_reg_x = 0xff;
        cpu.run();
        assert_eq!(cpu.index_reg_x, 1)
    }
}
