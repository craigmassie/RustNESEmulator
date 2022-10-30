use crate::bus::bus::Bus;
use crate::cpu::instruction::to_instruction;
use crate::traits::read::Read;
use crate::traits::write::Write;
use crate::cpu::addressing_mode::AddressingMode;
use std::fs::read;

#[derive(Debug)]
pub struct Cpu6502<'a>{
    bus: &'a Bus<'a>,
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    program_counter: u16,
    status_register: u8,
    address_absolute: u16,
    address_relative: u16,
    opcode: u8,
    cycles: u8,
    fetched: u8
}

impl<'a> Cpu6502<'a> {
    pub fn new(bus: &'a Bus<'a>) -> Cpu6502<'a> {
        Cpu6502 {
            bus,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: 0,
            program_counter: 0,
            status_register: 0,
            address_absolute: 0,
            address_relative: 0,
            opcode: 0,
            cycles: 0,
            fetched: 0
        }
    }

    pub fn clock(&mut self){
        if self.cycles == 0 {
            self.opcode = self.read(self.program_counter)
                .expect("Program counter address out of bounds");
            self.program_counter += 1;

            let instruction = to_instruction(self.opcode);
            self.cycles = instruction.cycles;
            let additional_addressing_clock_cycle = instruction.addressing_mode.execute(self);
            let additional_execution_clock_cycle = instruction.opcode.execute(self);
            self.cycles += additional_addressing_clock_cycle + additional_execution_clock_cycle;
        // execute by execution code, get additional clock cycle
        // add both to count
        }
        self.cycles -= 1;
    }

    pub fn fetch(&mut self) -> u8 {
        if to_instruction(self.opcode).addressing_mode != AddressingMode::Implied{
            self.fetched = self.read(self.address_absolute)
                .expect("Program counter address out of bounds");
        }
        self.fetched
    }

    pub fn reset(){}
    pub fn interrupt_request_signal(){}
    pub fn non_maskable_interrupt_request_signal(){}
}

impl Read<u16, u8> for Cpu6502<'_> {
    fn read(&self, address: u16) -> Option<u8>{
        self.bus.read(address)
    }

    fn read_only(&self, address: u16) -> Option<u8>{
        self.bus.read_only(address)
    }
}

impl Write<u16, u8> for Cpu6502<'_> {
    fn write(&self, address: u16, data: u8) {
        self.bus.write(address, data)
    }
}

enum ProcessorStatusRegister {
    Carry = 0x01,
    Zero = 0x02,
    DisableInterrupts = 0x03,
    DecimalModel = 0x04,
    Break = 0x05,
    Overflow = 0x07,
    Negative = 0x08
}

