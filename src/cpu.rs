use bit_field::BitField;

pub struct CPU {
    pub pc: u16,
    pub reg_a: u8,
    pub sp: u8,
    pub index_reg_x: u8,
    pub index_reg_y: u8,
    pub status: u8,
    memory: [u8; 0xFFFF]
}

const NEGATIVE_BIT: usize = 7;

const STATUS_BIT_N: usize = 7;
// const STATUS_BIT_V: usize = 6;
// const STATUS_BIT_B: usize = 4;
// const STATUS_BIT_D: usize = 3;
// const STATUS_BIT_I: usize = 2;
const STATUS_BIT_Z: usize = 1;
// const STATUS_BIT_C: usize = 0;

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            reg_a: 0,
            sp: 0,
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

        self.pc = self.mem_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.mem_read(self.pc);
            self.pc += 1;

            match opcode {
                0xA9 => {
                    let param = self.mem_read(self.pc);
                    self.pc += 1;
                    self.lda(param);
                }
                0xAA => self.tx(),
                0xE8 => self.inx(),
                0x00 => {
                    return;
                }
                _ => todo!(),
            }
        }
    }

    fn update_zero_and_negative_flags(&mut self, reg: u8) {
        self.status.set_bit(STATUS_BIT_Z, reg == 0);
        self.status.set_bit(STATUS_BIT_N, reg.get_bit(NEGATIVE_BIT));
    }

    fn lda(&mut self, value: u8) {
        self.reg_a = value;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn tx(&mut self) {
        self.index_reg_x = self.reg_a;
        self.update_zero_and_negative_flags(self.index_reg_x);
    }

    fn inx(&mut self) {
        self.index_reg_x = self.index_reg_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.index_reg_x);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xaa, 0x00]);
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
        cpu.load(vec![0xaa, 0x00]);
        cpu.reset();
        cpu.reg_a = 0;
        cpu.run();
        assert!(cpu.index_reg_x == 0);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

       #[test]
   fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

       assert_eq!(cpu.index_reg_x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.reset();
        cpu.index_reg_x = 0xff;
        cpu.run();
        assert_eq!(cpu.index_reg_x, 1)
    }
}
