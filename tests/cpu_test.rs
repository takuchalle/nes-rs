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

#[test]
fn test_and() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0x29, 0x0f, 0x00]);
    assert_eq!(cpu.reg_a, 0x0f);
}

#[test]
fn test_and_zero_page() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0xff, /* lda #0xff */
        0x85, 0x00, /* sta zero */
        0xa9, 0x0f, /* lda #0x0f */
        0x25, 0x00, /* and zero */
        0x00 /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0x0f);
}

#[test]
fn test_and_absolute() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0xf0, /* lda #0xf0 */
        0x85, 0x00, /* sta zero */
        0xa9, 0x0f, /* lda #0x0f */
        0x2d, 0x00, 0x00, /* and absolute */
        0x00 /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0x00);
    assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
}
