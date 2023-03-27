use bit_field::BitField;

pub struct CPU {
    pub pc: u16,
    pub reg_a: u8,
    pub sp: u8,
    pub index_reg_x: u8,
    pub index_reg_y: u8,
    pub status: u8,
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
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.pc = 0;
        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;

            match opcode {
                0xA9 => {
                    let param = program[self.pc as usize];
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
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.reg_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);
        assert!(cpu.index_reg_x == 10);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x_zero_flg() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0;
        cpu.interpret(vec![0xaa, 0x00]);
        assert!(cpu.index_reg_x == 0);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

       #[test]
   fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

       assert_eq!(cpu.index_reg_x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.index_reg_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.index_reg_x, 1)
    }
}
