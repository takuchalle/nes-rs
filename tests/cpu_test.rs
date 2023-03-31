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

#[test]
fn test_asl() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0xf0, /* lda #0xf0 */
        0x0a, /* asl accumulator */
        0x00 /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0xe0);
    assert_eq!(cpu.status & 0b0000_0001, 0b0000_0001);
}

#[test]
fn test_asl_zero_page() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x78, /* lda #0x78 */
        0x85, 0x00, /* sta zero */
        0x06, 0x00, /* asl zero */
        0xa5, 0x00, /* lda zero */
        0x00 /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0xf0);
    assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
}

#[test]
fn test_branch() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x00, /* lda #0x00 */
        0xf0, 0x02, /* BEQ #0x02 */
        0x00, /* BRK */
        0x00, /* BRK */
        0xa9, 0x0e, /* lda #0xee */
        0x00 /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0x0e);
    assert_eq!(cpu.status, 0x00);
}

#[test]
fn test_bit() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0xf0, /* lda #0xf0 */
        0x85, 0x00, /* sta zero */
        0xa9, 0x0f, /* lda #0x0f */
        0x24, 0x00, /* bit zero */
        0x00 /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0x0f);
    assert_eq!(cpu.status & 0b1100_0010, 0b1100_0010);
}

#[test]
fn test_bit_non_zero_flg() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0xf0, /* lda #0xf0 */
        0x85, 0x00, /* sta zero */
        0xa9, 0xf0, /* lda #0xf0 */
        0x24, 0x00, /* bit zero */
        0x00 /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0xf0);
    assert_eq!(cpu.status & 0b1100_0010, 0b1100_0000);
}

#[test]
fn test_clc() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0xff, /* lda #0xff */
        0x69, 0x01, /* adc #0x01 */
        0x18, /* CLC */
        0x00 /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0x00);
    assert_eq!(cpu.status & 0b0000_0001, 0b0000_0000);
}
