use crate::cpu::cpu_6502::Cpu6502;

#[derive(PartialEq, Eq, Clone)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    Absolute,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
}

impl AddressingMode {
    pub fn execute(&self, cpu: &mut Cpu6502) -> u8{
        match self {
            &AddressingMode::Implied => {
                0
            },
            &AddressingMode::Accumulator => {
                0
            },
            &AddressingMode::Immediate => {
                0
            },
            &AddressingMode::Absolute => {
                0
            },
            &AddressingMode::ZeroPage => {
                0
            },
            &AddressingMode::ZeroPageX => {
                0
            },
            &AddressingMode::ZeroPageY => {
                0
            },
            &AddressingMode::AbsoluteX => {
                0
            },
            &AddressingMode::AbsoluteY => {
                0
            },
            &AddressingMode::Relative => {
                0
            },
            &AddressingMode::Indirect => {
                0
            },
            &AddressingMode::IndirectX => {
                0
            },
            &AddressingMode::IndirectY => {
                0
            },
        }
    }
}