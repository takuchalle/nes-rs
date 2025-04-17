pub mod cpu;
pub mod opcodes;

pub struct Nes {
    cpu: cpu::CPU,
}

impl Nes {
    pub fn new(rom: &[u8]) -> Nes {
        let mut cpu = cpu::CPU::new();
        cpu.load(rom);
        Self { cpu }
    }
}
