extern crate nes_rs;
use bitflags::bitflags;

bitflags! {
    struct Status: u32 {
        const C = 0b00000001;
        const Z = 0b00000010;
        const I = 0b00000100;
        const D = 0b00001000;
        const B = 0b00010000;
        const V = 0b01000000;
        const N = 0b10000000;
    }
}

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
        0x00, /* BRK */
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
        0x00, /* BRK */
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
        0x00, /* BRK */
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
        0x00, /* BRK */
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
        0x00, /* BRK */
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
        0x00, /* BRK */
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
        0x00, /* BRK */
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
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0x00);
    assert_eq!(cpu.status & 0b0000_0001, 0b0000_0000);
}

#[test]
fn test_cmp() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x12, /* lda #0x12 */
        0xc9, 0x12, /* cmp #0x12 */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0x12);
    assert_eq!(cpu.status & 0b1000_0011, 0b0000_0011);
}

#[test]
fn test_cmp_negative() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x02, /* lda #0x02 */
        0x85, 0x00, /* sta zero */
        0xa9, 0xf2, /* lda #0xf2 */
        0xc5, 0x00, /* cmp zero */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0xf2);
    assert_eq!(cpu.status & 0b1000_0011, 0b1000_0001);
}

#[test]
fn test_ldx() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa2, 0x02, /* ldx #0x02 */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.index_reg_x, 0x02);
    assert_eq!(cpu.status, 0x0);
}

#[test]
fn test_ldy() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa0, 0x02, /* ldy #0x02 */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.index_reg_y, 0x02);
    assert_eq!(cpu.status, 0x0);
}

#[test]
fn test_cpx() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa2, 0x02, /* ldx #0x02 */
        0xe0, 0x02, /* cpy #0x02 */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.index_reg_x, 0x02);
    assert_eq!(cpu.status & 0b1000_0011, 0b0000_0011);
}

#[test]
fn test_cpy() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa0, 0x02, /* ldy #0x02 */
        0xc0, 0x02, /* cpy #0x02 */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.index_reg_y, 0x02);
    assert_eq!(cpu.status & 0b1000_0011, 0b0000_0011);
}

#[test]
fn test_dec() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x04, /* lda #0x04 */
        0x85, 0x00, /* sta zero */
        0xc6, 0x00, /* dec zero */
        0xa5, 0x00, /* lda zero */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0x03);
    assert_eq!(cpu.status & 0b0100_0001, 0b0000_0000);
}

#[test]
fn test_dex() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa2, 0x01, /* ldx #0x01 */
        0xca, /* dex */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.index_reg_x, 0x00);
    assert_eq!(cpu.status & 0b1000_0010, 0b0000_0010);
}

#[test]
fn test_dey() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa0, 0x01, /* ldy #0x01 */
        0x88, /* dey */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.index_reg_y, 0x00);
    assert_eq!(cpu.status & 0b1000_0010, 0b0000_0010);
}

#[test]
fn test_eor() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0xff, /* lda #0xff */
        0x85, 0x00, /* sta zero */
        0xa9, 0x0f, /* lda #0x0f */
        0x45, 0x00, /* eor zero */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0xf0);
    assert_eq!(cpu.status & 0b1000_0010, 0b1000_0000);
}

#[test]
fn test_inc() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x04, /* lda #0x04 */
        0x85, 0x00, /* sta zero */
        0xe6, 0x00, /* dec zero */
        0xa5, 0x00, /* lda zero */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0x05);
    assert_eq!(cpu.status & 0b0100_0001, 0b0000_0000);
}

#[test]
fn test_ora() {
    let mut cpu = nes_rs::cpu::CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x04, /* lda #0x04 */
        0x09, 0x40, /* ora #0x40 */
        0x00, /* BRK */
    ]);
    assert_eq!(cpu.reg_a, 0x44);
    assert_eq!(cpu.status & 0b0100_0001, 0b0000_0000);
}

