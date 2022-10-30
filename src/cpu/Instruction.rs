use crate::cpu::opcode::Opcode;
use crate::cpu::addressing_mode::AddressingMode;
use memoize::memoize;

#[derive(Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
    pub cycles: u8
}

impl Instruction {
    fn build_instruction(opcode: Opcode, addressing_mode: AddressingMode, cycles: u8) -> Self {
        Instruction {
            opcode,
            addressing_mode,
            cycles
        }
    }
}

#[memoize]
pub fn to_instruction(instruction_code: u8) -> Instruction {
    match instruction_code {
        /* *************** binary op ***************  */
        0x69 => Instruction::build_instruction(Opcode::ADC, AddressingMode::Immediate, 2),
        0x65 => Instruction::build_instruction(Opcode::ADC, AddressingMode::ZeroPage, 3),
        0x75 => Instruction::build_instruction(Opcode::ADC, AddressingMode::ZeroPageX, 4),
        0x6d => Instruction::build_instruction(Opcode::ADC, AddressingMode::Absolute, 4),
        0x7d => Instruction::build_instruction(Opcode::ADC, AddressingMode::AbsoluteX, 4),
        0x79 => Instruction::build_instruction(Opcode::ADC, AddressingMode::AbsoluteY, 4),
        0x61 => Instruction::build_instruction(Opcode::ADC, AddressingMode::IndirectX, 6),
        0x71 => Instruction::build_instruction(Opcode::ADC, AddressingMode::IndirectY, 5),

        0xe9 => Instruction::build_instruction(Opcode::SBC, AddressingMode::Immediate, 2),
        0xe5 => Instruction::build_instruction(Opcode::SBC, AddressingMode::ZeroPage, 3),
        0xf5 => Instruction::build_instruction(Opcode::SBC, AddressingMode::ZeroPageX, 4),
        0xed => Instruction::build_instruction(Opcode::SBC, AddressingMode::Absolute, 4),
        0xfd => Instruction::build_instruction(Opcode::SBC, AddressingMode::AbsoluteX, 4),
        0xf9 => Instruction::build_instruction(Opcode::SBC, AddressingMode::AbsoluteY, 4),
        0xe1 => Instruction::build_instruction(Opcode::SBC, AddressingMode::IndirectX, 6),
        0xf1 => Instruction::build_instruction(Opcode::SBC, AddressingMode::IndirectY, 5),

        0x29 => Instruction::build_instruction(Opcode::AND, AddressingMode::Immediate, 2),
        0x25 => Instruction::build_instruction(Opcode::AND, AddressingMode::ZeroPage, 3),
        0x35 => Instruction::build_instruction(Opcode::AND, AddressingMode::ZeroPageX, 4),
        0x2d => Instruction::build_instruction(Opcode::AND, AddressingMode::Absolute, 4),
        0x3d => Instruction::build_instruction(Opcode::AND, AddressingMode::AbsoluteX, 4),
        0x39 => Instruction::build_instruction(Opcode::AND, AddressingMode::AbsoluteY, 4),
        0x21 => Instruction::build_instruction(Opcode::AND, AddressingMode::IndirectX, 6),
        0x31 => Instruction::build_instruction(Opcode::AND, AddressingMode::IndirectY, 5),

        0x49 => Instruction::build_instruction(Opcode::EOR, AddressingMode::Immediate, 2),
        0x45 => Instruction::build_instruction(Opcode::EOR, AddressingMode::ZeroPage, 3),
        0x55 => Instruction::build_instruction(Opcode::EOR, AddressingMode::ZeroPageX, 4),
        0x4d => Instruction::build_instruction(Opcode::EOR, AddressingMode::Absolute, 4),
        0x5d => Instruction::build_instruction(Opcode::EOR, AddressingMode::AbsoluteX, 4),
        0x59 => Instruction::build_instruction(Opcode::EOR, AddressingMode::AbsoluteY, 4),
        0x41 => Instruction::build_instruction(Opcode::EOR, AddressingMode::IndirectX, 6),
        0x51 => Instruction::build_instruction(Opcode::EOR, AddressingMode::IndirectY, 5),

        0x09 => Instruction::build_instruction(Opcode::ORA, AddressingMode::Immediate, 2),
        0x05 => Instruction::build_instruction(Opcode::ORA, AddressingMode::ZeroPage, 3),
        0x15 => Instruction::build_instruction(Opcode::ORA, AddressingMode::ZeroPageX, 4),
        0x0d => Instruction::build_instruction(Opcode::ORA, AddressingMode::Absolute, 4),
        0x1d => Instruction::build_instruction(Opcode::ORA, AddressingMode::AbsoluteX, 4),
        0x19 => Instruction::build_instruction(Opcode::ORA, AddressingMode::AbsoluteY, 4),
        0x01 => Instruction::build_instruction(Opcode::ORA, AddressingMode::IndirectX, 6),
        0x11 => Instruction::build_instruction(Opcode::ORA, AddressingMode::IndirectY ,5),

        /* *************** shift/rotate op ***************  */
        0x0a => Instruction::build_instruction(Opcode::ASL, AddressingMode::Accumulator, 2),
        0x06 => Instruction::build_instruction(Opcode::ASL, AddressingMode::ZeroPage, 5),
        0x16 => Instruction::build_instruction(Opcode::ASL, AddressingMode::ZeroPageX, 6),
        0x0e => Instruction::build_instruction(Opcode::ASL, AddressingMode::Absolute, 6),
        0x1e => Instruction::build_instruction(Opcode::ASL, AddressingMode::AbsoluteX, 7),

        0x4a => Instruction::build_instruction(Opcode::LSR, AddressingMode::Accumulator, 2),
        0x46 => Instruction::build_instruction(Opcode::LSR, AddressingMode::ZeroPage, 5),
        0x56 => Instruction::build_instruction(Opcode::LSR, AddressingMode::ZeroPageX, 6),
        0x4e => Instruction::build_instruction(Opcode::LSR, AddressingMode::Absolute, 6),
        0x5e => Instruction::build_instruction(Opcode::LSR, AddressingMode::AbsoluteX, 7),

        0x2a => Instruction::build_instruction(Opcode::ROL, AddressingMode::Accumulator, 2),
        0x26 => Instruction::build_instruction(Opcode::ROL, AddressingMode::ZeroPage, 5),
        0x36 => Instruction::build_instruction(Opcode::ROL, AddressingMode::ZeroPageX, 6),
        0x2e => Instruction::build_instruction(Opcode::ROL, AddressingMode::Absolute, 6),
        0x3e => Instruction::build_instruction(Opcode::ROL, AddressingMode::AbsoluteX, 7),

        0x6a => Instruction::build_instruction(Opcode::ROR, AddressingMode::Accumulator, 2),
        0x66 => Instruction::build_instruction(Opcode::ROR, AddressingMode::ZeroPage, 5),
        0x76 => Instruction::build_instruction(Opcode::ROR, AddressingMode::ZeroPageX, 6),
        0x6e => Instruction::build_instruction(Opcode::ROR, AddressingMode::Absolute, 6),
        0x7e => Instruction::build_instruction(Opcode::ROR, AddressingMode::AbsoluteX, 7),

        /* *************** inc/dec op ***************  */
        0xe6 => Instruction::build_instruction(Opcode::INC, AddressingMode::ZeroPage, 5),
        0xf6 => Instruction::build_instruction(Opcode::INC, AddressingMode::ZeroPageX, 6),
        0xee => Instruction::build_instruction(Opcode::INC, AddressingMode::Absolute, 6),
        0xfe => Instruction::build_instruction(Opcode::INC, AddressingMode::AbsoluteX, 7),

        0xe8 => Instruction::build_instruction(Opcode::INX, AddressingMode::Implied, 2),
        0xc8 => Instruction::build_instruction(Opcode::INY, AddressingMode::Implied, 2),

        0xc6 => Instruction::build_instruction(Opcode::DEC, AddressingMode::ZeroPage, 5),
        0xd6 => Instruction::build_instruction(Opcode::DEC, AddressingMode::ZeroPageX, 6),
        0xce => Instruction::build_instruction(Opcode::DEC, AddressingMode::Absolute, 6),
        0xde => Instruction::build_instruction(Opcode::DEC, AddressingMode::AbsoluteX, 7),

        0xca => Instruction::build_instruction(Opcode::DEX, AddressingMode::Implied, 2),
        0x88 => Instruction::build_instruction(Opcode::DEY, AddressingMode::Implied, 2),

        /* *************** load/store op ***************  */
        0xa9 => Instruction::build_instruction(Opcode::LDA, AddressingMode::Immediate, 2),
        0xa5 => Instruction::build_instruction(Opcode::LDA, AddressingMode::ZeroPage, 3),
        0xb5 => Instruction::build_instruction(Opcode::LDA, AddressingMode::ZeroPageX, 4),
        0xad => Instruction::build_instruction(Opcode::LDA, AddressingMode::Absolute, 4),
        0xbd => Instruction::build_instruction(Opcode::LDA, AddressingMode::AbsoluteX, 4),
        0xb9 => Instruction::build_instruction(Opcode::LDA, AddressingMode::AbsoluteY, 4),
        0xa1 => Instruction::build_instruction(Opcode::LDA, AddressingMode::IndirectX, 6),
        0xb1 => Instruction::build_instruction(Opcode::LDA, AddressingMode::IndirectY, 5),

        0xa2 => Instruction::build_instruction(Opcode::LDX, AddressingMode::Immediate, 2),
        0xa6 => Instruction::build_instruction(Opcode::LDX, AddressingMode::ZeroPage, 3),
        0xb6 => Instruction::build_instruction(Opcode::LDX, AddressingMode::ZeroPageY, 4),
        0xae => Instruction::build_instruction(Opcode::LDX, AddressingMode::Absolute, 4),
        0xbe => Instruction::build_instruction(Opcode::LDX, AddressingMode::AbsoluteY, 4),

        0xa0 => Instruction::build_instruction(Opcode::LDY, AddressingMode::Immediate, 2),
        0xa4 => Instruction::build_instruction(Opcode::LDY, AddressingMode::ZeroPage, 3),
        0xb4 => Instruction::build_instruction(Opcode::LDY, AddressingMode::ZeroPageX, 4),
        0xac => Instruction::build_instruction(Opcode::LDY, AddressingMode::Absolute, 4),
        0xbc => Instruction::build_instruction(Opcode::LDY, AddressingMode::AbsoluteX, 4),

        0x85 => Instruction::build_instruction(Opcode::STA, AddressingMode::ZeroPage, 3),
        0x95 => Instruction::build_instruction(Opcode::STA, AddressingMode::ZeroPageX, 4),
        0x8d => Instruction::build_instruction(Opcode::STA, AddressingMode::Absolute, 4),
        0x9d => Instruction::build_instruction(Opcode::STA, AddressingMode::AbsoluteX, 5),
        0x99 => Instruction::build_instruction(Opcode::STA, AddressingMode::AbsoluteY, 5),
        0x81 => Instruction::build_instruction(Opcode::STA, AddressingMode::IndirectX, 6),
        0x91 => Instruction::build_instruction(Opcode::STA, AddressingMode::IndirectY, 6),

        // 0x86 => Instruction::build_instruction(Opcode::STX, AddressingMode::ZeroPage),
        // 0x96 => Instruction::build_instruction(Opcode::STX, AddressingMode::ZeroPageY),
        // 0x8e => Instruction::build_instruction(Opcode::STX, AddressingMode::Absolute),
        //
        // 0x84 => Instruction::build_instruction(Opcode::STY, AddressingMode::ZeroPage),
        // 0x94 => Instruction::build_instruction(Opcode::STY, AddressingMode::ZeroPageX),
        // 0x8c => Instruction::build_instruction(Opcode::STY, AddressingMode::Absolute),
        //
        // /* *************** set/clear flag ***************  */
        // 0x38 => Instruction::build_instruction(Opcode::SEC, AddressingMode::Implied),
        // 0xf8 => Instruction::build_instruction(Opcode::SED, AddressingMode::Implied),
        // 0x78 => Instruction::build_instruction(Opcode::SEI, AddressingMode::Implied),
        // 0x18 => Instruction::build_instruction(Opcode::CLC, AddressingMode::Implied),
        // 0xd8 => Instruction::build_instruction(Opcode::CLD, AddressingMode::Implied),
        // 0x58 => Instruction::build_instruction(Opcode::CLI, AddressingMode::Implied),
        // 0xb8 => Instruction::build_instruction(Opcode::CLV, AddressingMode::Implied),
        //
        // /* *************** compare ***************  */
        // 0xc9 => Instruction::build_instruction(Opcode::CMP, AddressingMode::Immediate),
        // 0xc5 => Instruction::build_instruction(Opcode::CMP, AddressingMode::ZeroPage),
        // 0xd5 => Instruction::build_instruction(Opcode::CMP, AddressingMode::ZeroPageX),
        // 0xcd => Instruction::build_instruction(Opcode::CMP, AddressingMode::Absolute),
        // 0xdd => Instruction::build_instruction(Opcode::CMP, AddressingMode::AbsoluteX),
        // 0xd9 => Instruction::build_instruction(Opcode::CMP, AddressingMode::AbsoluteY),
        // 0xc1 => Instruction::build_instruction(Opcode::CMP, AddressingMode::IndirectX),
        // 0xd1 => Instruction::build_instruction(Opcode::CMP, AddressingMode::IndirectY),
        //
        // 0xe0 => Instruction::build_instruction(Opcode::CPX, AddressingMode::Immediate),
        // 0xe4 => Instruction::build_instruction(Opcode::CPX, AddressingMode::ZeroPage),
        // 0xec => Instruction::build_instruction(Opcode::CPX, AddressingMode::Absolute),
        //
        // 0xc0 => Instruction::build_instruction(Opcode::CPY, AddressingMode::Immediate),
        // 0xc4 => Instruction::build_instruction(Opcode::CPY, AddressingMode::ZeroPage),
        // 0xcc => Instruction::build_instruction(Opcode::CPY, AddressingMode::Absolute),
        //
        // /* *************** jump/return ***************  */
        // 0x4c => Instruction::build_instruction(Opcode::JMP, AddressingMode::Absolute),
        // 0x6c => Instruction::build_instruction(Opcode::JMP, AddressingMode::Indirect),
        //
        // 0x20 => Instruction::build_instruction(Opcode::JSR, AddressingMode::Absolute),
        //
        // 0x40 => Instruction::build_instruction(Opcode::RTI, AddressingMode::Implied),
        // 0x60 => Instruction::build_instruction(Opcode::RTS, AddressingMode::Implied),
        //
        // /* *************** branch ***************  */
        // 0x90 => Instruction::build_instruction(Opcode::BCC, AddressingMode::Relative),
        // 0xb0 => Instruction::build_instruction(Opcode::BCS, AddressingMode::Relative),
        // 0xf0 => Instruction::build_instruction(Opcode::BEQ, AddressingMode::Relative),
        // 0xd0 => Instruction::build_instruction(Opcode::BNE, AddressingMode::Relative),
        // 0x30 => Instruction::build_instruction(Opcode::BMI, AddressingMode::Relative),
        // 0x10 => Instruction::build_instruction(Opcode::BPL, AddressingMode::Relative),
        // 0x50 => Instruction::build_instruction(Opcode::BVC, AddressingMode::Relative),
        // 0x70 => Instruction::build_instruction(Opcode::BVS, AddressingMode::Relative),
        //
        // /* *************** push/pop ***************  */
        // 0x48 => Instruction::build_instruction(Opcode::PHA, AddressingMode::Implied),
        // 0x08 => Instruction::build_instruction(Opcode::PHP, AddressingMode::Implied),
        // 0x68 => Instruction::build_instruction(Opcode::PLA, AddressingMode::Implied),
        // 0x28 => Instruction::build_instruction(Opcode::PLP, AddressingMode::Implied),
        //
        // /* *************** transfer ***************  */
        // 0xaa => Instruction::build_instruction(Opcode::TAX, AddressingMode::Implied),
        // 0xa8 => Instruction::build_instruction(Opcode::TAY, AddressingMode::Implied),
        // 0xba => Instruction::build_instruction(Opcode::TSX, AddressingMode::Implied),
        // 0x8a => Instruction::build_instruction(Opcode::TXA, AddressingMode::Implied),
        // 0x9a => Instruction::build_instruction(Opcode::TXS, AddressingMode::Implied),
        // 0x98 => Instruction::build_instruction(Opcode::TYA, AddressingMode::Implied),
        //
        // /* *************** other ***************  */
        // 0x00 => Instruction::build_instruction(Opcode::BRK, AddressingMode::Implied),
        //
        // 0x24 => Instruction::build_instruction(Opcode::BIT, AddressingMode::ZeroPage),
        // 0x2c => Instruction::build_instruction(Opcode::BIT, AddressingMode::Absolute),
        //
        // 0xea => Instruction::build_instruction(Opcode::NOP, AddressingMode::Implied),
        //
        // /* *************** unofficial1 ***************  */
        // 0x4b => Instruction::build_instruction(Opcode::ALR, AddressingMode::Immediate),
        // 0x0b => Instruction::build_instruction(Opcode::ANC, AddressingMode::Immediate),
        // 0x6b => Instruction::build_instruction(Opcode::ARR, AddressingMode::Immediate),
        // 0xcb => Instruction::build_instruction(Opcode::AXS, AddressingMode::Immediate),
        //
        // 0xa3 => Instruction::build_instruction(Opcode::LAX, AddressingMode::IndirectX),
        // 0xa7 => Instruction::build_instruction(Opcode::LAX, AddressingMode::ZeroPage),
        // 0xaf => Instruction::build_instruction(Opcode::LAX, AddressingMode::Absolute),
        // 0xb3 => Instruction::build_instruction(Opcode::LAX, AddressingMode::IndirectY),
        // 0xb7 => Instruction::build_instruction(Opcode::LAX, AddressingMode::ZeroPageY),
        // 0xbf => Instruction::build_instruction(Opcode::LAX, AddressingMode::AbsoluteY),
        //
        // 0x83 => Instruction::build_instruction(Opcode::SAX, AddressingMode::IndirectX),
        // 0x87 => Instruction::build_instruction(Opcode::SAX, AddressingMode::ZeroPage),
        // 0x8f => Instruction::build_instruction(Opcode::SAX, AddressingMode::Absolute),
        // 0x97 => Instruction::build_instruction(Opcode::SAX, AddressingMode::ZeroPageY),
        //
        // 0xc3 => Instruction::build_instruction(Opcode::DCP, AddressingMode::IndirectX),
        // 0xc7 => Instruction::build_instruction(Opcode::DCP, AddressingMode::ZeroPage),
        // 0xcf => Instruction::build_instruction(Opcode::DCP, AddressingMode::Absolute),
        // 0xd3 => Instruction::build_instruction(Opcode::DCP, AddressingMode::IndirectY),
        // 0xd7 => Instruction::build_instruction(Opcode::DCP, AddressingMode::ZeroPageX),
        // 0xdb => Instruction::build_instruction(Opcode::DCP, AddressingMode::AbsoluteY),
        // 0xdf => Instruction::build_instruction(Opcode::DCP, AddressingMode::AbsoluteX),
        //
        // 0xe3 => Instruction::build_instruction(Opcode::ISC, AddressingMode::IndirectX),
        // 0xe7 => Instruction::build_instruction(Opcode::ISC, AddressingMode::ZeroPage),
        // 0xef => Instruction::build_instruction(Opcode::ISC, AddressingMode::Absolute),
        // 0xf3 => Instruction::build_instruction(Opcode::ISC, AddressingMode::IndirectY),
        // 0xf7 => Instruction::build_instruction(Opcode::ISC, AddressingMode::ZeroPageX),
        // 0xfb => Instruction::build_instruction(Opcode::ISC, AddressingMode::AbsoluteY),
        // 0xff => Instruction::build_instruction(Opcode::ISC, AddressingMode::AbsoluteX),
        //
        // 0x23 => Instruction::build_instruction(Opcode::RLA, AddressingMode::IndirectX),
        // 0x27 => Instruction::build_instruction(Opcode::RLA, AddressingMode::ZeroPage),
        // 0x2f => Instruction::build_instruction(Opcode::RLA, AddressingMode::Absolute),
        // 0x33 => Instruction::build_instruction(Opcode::RLA, AddressingMode::IndirectY),
        // 0x37 => Instruction::build_instruction(Opcode::RLA, AddressingMode::ZeroPageX),
        // 0x3b => Instruction::build_instruction(Opcode::RLA, AddressingMode::AbsoluteY),
        // 0x3f => Instruction::build_instruction(Opcode::RLA, AddressingMode::AbsoluteX),
        //
        // 0x63 => Instruction::build_instruction(Opcode::RRA, AddressingMode::IndirectX),
        // 0x67 => Instruction::build_instruction(Opcode::RRA, AddressingMode::ZeroPage),
        // 0x6f => Instruction::build_instruction(Opcode::RRA, AddressingMode::Absolute),
        // 0x73 => Instruction::build_instruction(Opcode::RRA, AddressingMode::IndirectY),
        // 0x77 => Instruction::build_instruction(Opcode::RRA, AddressingMode::ZeroPageX),
        // 0x7b => Instruction::build_instruction(Opcode::RRA, AddressingMode::AbsoluteY),
        // 0x7f => Instruction::build_instruction(Opcode::RRA, AddressingMode::AbsoluteX),
        //
        // 0x03 => Instruction::build_instruction(Opcode::SLO, AddressingMode::IndirectX),
        // 0x07 => Instruction::build_instruction(Opcode::SLO, AddressingMode::ZeroPage),
        // 0x0f => Instruction::build_instruction(Opcode::SLO, AddressingMode::Absolute),
        // 0x13 => Instruction::build_instruction(Opcode::SLO, AddressingMode::IndirectY),
        // 0x17 => Instruction::build_instruction(Opcode::SLO, AddressingMode::ZeroPageX),
        // 0x1b => Instruction::build_instruction(Opcode::SLO, AddressingMode::AbsoluteY),
        // 0x1f => Instruction::build_instruction(Opcode::SLO, AddressingMode::AbsoluteX),
        //
        // 0x43 => Instruction::build_instruction(Opcode::SRE, AddressingMode::IndirectX),
        // 0x47 => Instruction::build_instruction(Opcode::SRE, AddressingMode::ZeroPage),
        // 0x4f => Instruction::build_instruction(Opcode::SRE, AddressingMode::Absolute),
        // 0x53 => Instruction::build_instruction(Opcode::SRE, AddressingMode::IndirectY),
        // 0x57 => Instruction::build_instruction(Opcode::SRE, AddressingMode::ZeroPageX),
        // 0x5b => Instruction::build_instruction(Opcode::SRE, AddressingMode::AbsoluteY),
        // 0x5f => Instruction::build_instruction(Opcode::SRE, AddressingMode::AbsoluteX),
        //
        // 0x80 => Instruction::build_instruction(Opcode::SKB, AddressingMode::Immediate),
        // 0x82 => Instruction::build_instruction(Opcode::SKB, AddressingMode::Immediate),
        // 0x89 => Instruction::build_instruction(Opcode::SKB, AddressingMode::Immediate),
        // 0xc2 => Instruction::build_instruction(Opcode::SKB, AddressingMode::Immediate),
        // 0xe2 => Instruction::build_instruction(Opcode::SKB, AddressingMode::Immediate),
        //
        // 0x0c => Instruction::build_instruction(Opcode::IGN, AddressingMode::Absolute),
        //
        // 0x1c => Instruction::build_instruction(Opcode::IGN, AddressingMode::AbsoluteX),
        // 0x3c => Instruction::build_instruction(Opcode::IGN, AddressingMode::AbsoluteX),
        // 0x5c => Instruction::build_instruction(Opcode::IGN, AddressingMode::AbsoluteX),
        // 0x7c => Instruction::build_instruction(Opcode::IGN, AddressingMode::AbsoluteX),
        // 0xdc => Instruction::build_instruction(Opcode::IGN, AddressingMode::AbsoluteX),
        // 0xfc => Instruction::build_instruction(Opcode::IGN, AddressingMode::AbsoluteX),
        //
        // 0x04 => Instruction::build_instruction(Opcode::IGN, AddressingMode::ZeroPage),
        // 0x44 => Instruction::build_instruction(Opcode::IGN, AddressingMode::ZeroPage),
        // 0x64 => Instruction::build_instruction(Opcode::IGN, AddressingMode::ZeroPage),
        //
        // 0x14 => Instruction::build_instruction(Opcode::IGN, AddressingMode::ZeroPageX),
        // 0x34 => Instruction::build_instruction(Opcode::IGN, AddressingMode::ZeroPageX),
        // 0x54 => Instruction::build_instruction(Opcode::IGN, AddressingMode::ZeroPageX),
        // 0x74 => Instruction::build_instruction(Opcode::IGN, AddressingMode::ZeroPageX),
        // 0xd4 => Instruction::build_instruction(Opcode::IGN, AddressingMode::ZeroPageX),
        // 0xf4 => Instruction::build_instruction(Opcode::IGN, AddressingMode::ZeroPageX),

        _ => panic!("Invalid inst_code:{:08x}", instruction_code),
    }
}