pub mod cpu;
pub mod opcodes;

pub struct Nes {
    pub cpu: cpu::CPU,
}

impl Nes {
    pub fn new(rom: &[u8]) -> Nes {
        let mut cpu = cpu::CPU::new();
        cpu.load(rom);
        cpu.reset();
        Self { cpu }
    }

    pub fn exec<F>(&mut self, callback: F) -> std::io::Result<()>
    where
        F: FnMut(&mut cpu::CPU) -> std::io::Result<()>,
    {
        self.cpu.run_with_callback(callback)
    }
}
