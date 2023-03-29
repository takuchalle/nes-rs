extern crate nes_rs;

#[test]
fn test_adc() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![0xa9, 0xfe, 0x69, 0x01, 0x00]);
    assert_eq!(cpu.reg_a, 0xFF);
    assert_eq!(cpu.status & 0b0000_0001, 0x0);
}

#[test]
fn test_adc_carried() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0x01, 0x00]);
    assert_eq!(cpu.reg_a, 0x00);
    assert_eq!(cpu.status & 0b0000_0001, 0b0000_00001);
}
