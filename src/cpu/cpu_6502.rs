use crate::bus::bus::Bus;
use crate::traits::read::Read;
use crate::traits::write::Write;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Cpu6502<'a>{
    bus: &'a Bus<'a>
}

impl<'a> Cpu6502<'a> {
    pub fn new(bus: &'a Bus<'a>) -> Cpu6502<'a> {
        Cpu6502 {
            bus
        }
    }
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

