use crate::cpu::cpu_6502::Cpu6502;

#[derive(PartialEq, Eq, Clone)]
pub enum Opcode {
    // binary op
    ADC,
    SBC,
    AND,
    EOR,
    ORA,
    // shift/rotate
    ASL,
    LSR,
    ROL,
    ROR,
    // inc/dec
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
    // load/store
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    // set/clear flag
    SEC,
    SED,
    SEI,
    CLC,
    CLD,
    CLI,
    CLV,
    // compare
    CMP,
    CPX,
    CPY,
    // jump return
    JMP,
    JSR,
    RTI,
    RTS,
    // branch
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    // push/pop
    PHA,
    PHP,
    PLA,
    PLP,
    // transfer
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    // other
    BRK,
    BIT,
    NOP,
    ALR,
    ANC,
    ARR,
    AXS,
    LAX,
    SAX,
    DCP,
    ISC,
    RLA,
    RRA,
    SLO,
    SRE,
    SKB,
    IGN,
}

impl Opcode {
    pub fn execute(&self, cpu: &mut Cpu6502) -> u8 {
        match self {
            Opcode::ADC => {
                0
            },
            Opcode::SBC => {
                0
            },
            Opcode::AND => {
                0
            },
            Opcode::EOR => {
                0
            },
            Opcode::ORA => {
                0
            },
            Opcode::ASL => {
                0
            },
            Opcode::LSR => {
                0
            },
            Opcode::ROL => {
                0
            },
            Opcode::ROR => {
                0
            },
            Opcode::INC => {
                0
            },
            Opcode::INX => {
                0
            },
            Opcode::INY => {
                0
            },
            Opcode::DEC => {
                0
            },
            Opcode::DEX => {
                0
            },
            Opcode::DEY => {
                0
            },
            Opcode::LDA => {
                0
            },
            Opcode::LDX => {
                0
            },
            Opcode::LDY => {
                0
            },
            Opcode::STA => {
                0
            },
            Opcode::STX => {
                0
            },
            Opcode::STY => {
                0
            },
            Opcode::SEC => {
                0
            },
            Opcode::SED => {
                0
            },
            Opcode::SEI => {
                0
            },
            Opcode::CLC => {
                0
            },
            Opcode::CLD => {
                0
            },
            Opcode::CLI => {
                0
            },
            Opcode::CLV => {
                0
            },
            Opcode::CMP => {
                0
            },
            Opcode::CPX => {
                0
            },
            Opcode::CPY => {
                0
            },
            Opcode::JMP => {
                0
            },
            Opcode::JSR => {
                0
            },
            Opcode::RTI => {
                0
            },
            Opcode::RTS => {
                0
            },
            Opcode::BCC => {
                0
            },
            Opcode::BCS => {
                0
            },
            Opcode::BEQ => {
                0
            },
            Opcode::BMI => {
                0
            },
            Opcode::BNE => {
                0
            },
            Opcode::BPL => {
                0
            },
            Opcode::BVC => {
                0
            },
            Opcode::BVS => {
                0
            },
            Opcode::PHA => {
                0
            },
            Opcode::PHP => {
                0
            },
            Opcode::PLA => {
                0
            },
            Opcode::PLP => {
                0
            },
            Opcode::TAX => {
                0
            },
            Opcode::TAY => {
                0
            },
            Opcode::TSX => {
                0
            },
            Opcode::TXA => {
                0
            },
            Opcode::TXS => {
                0
            },
            Opcode::TYA => {
                0
            },
            Opcode::BRK => {
                0
            },
            Opcode::BIT => {
                0
            },
            Opcode::NOP => {
                0
            },
            Opcode::ALR => {
                0
            },
            Opcode::ANC => {
                0
            },
            Opcode::ARR => {
                0
            },
            Opcode::AXS => {
                0
            },
            Opcode::LAX => {
                0
            },
            Opcode::SAX => {
                0
            },
            Opcode::DCP => {
                0
            },
            Opcode::ISC => {
                0
            },
            Opcode::RLA => {
                0
            },
            Opcode::RRA => {
                0
            },
            Opcode::SLO => {
                0
            },
            Opcode::SRE => {
                0
            },
            Opcode::SKB => {
                0
            },
            Opcode::IGN => {
                0
            },
        }
    }
}