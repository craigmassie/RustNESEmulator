use crate::cpu::cpu_6502::Cpu6502;
use crate::traits::read::Read;
use crate::traits::write::Write;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Bus<'a> {
    cpu: RefCell<Option<&'a Cpu6502<'a>>>,
    ram: RefCell<[u8; 8192]>
}

impl<'a> Bus<'a>{

    pub fn new() -> Self {
        Bus {
            cpu: RefCell::new(None),
            ram: RefCell::new([0; 8192])
        }
    }

    pub fn attach_cpu(&self, cpu: &'a Cpu6502<'a>) {
        self.cpu.replace(Some(cpu));
    }
}

impl Read<u16, u8> for Bus<'_> {
    fn read(&self, address: u16) -> Option<u8>{
        if address >= 0x0000 && address <= 0xFFFF {
            return Some(self.ram.borrow()[address as usize]);
        }
        None
    }

    fn read_only(&self, address: u16) -> Option<u8>{
        if address >= 0x0000 && address <= 0xFFFF {
            return Some(self.ram.borrow()[address as usize]);
        }
        None
    }
}

impl Write<u16, u8> for Bus<'_>  {
    fn write(&self, address: u16, data: u8) {
        if address >= 0x0000 && address <= 0xFFFF {
            self.ram.borrow_mut()[address as usize] = data;
        }
    }
}