pub struct CPU {
    pub pc: u16,
    pub reg_a: u8,
    pub sp: u8,
    pub index_reg_x: u8,
    pub index_reg_y: u8,
    pub status: u8,
}

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
                    self.reg_a = param;
                    if self.reg_a == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }

                    if self.reg_a & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000;
                    } else {
                        self.status = self.status & 0b0111_1111;
                    }
                }
                0x00 => {
                    return;
                }
                _ => todo!(),
            }
        }
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
}
