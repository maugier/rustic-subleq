
use std::io::{self, Read, Write};
use rand::Rng;


type Word = u8;


pub struct Bus {
    regs: [Word; 16],
    ram: Vec<Word>,
}

impl Bus {

    pub fn new(ram: Vec<Word>) -> Self {
        Bus { regs: [0; 16], ram }
    }

    fn write(&mut self, addr: Word, val: Word) {

        match addr {
            0 if val == 0 => panic!("Halted"),
            reg @ 0x10 ..= 0x1f => self.regs[(reg as usize) - 0x10] = val,
            0x21 => io::stdout().write_all(&[val as u8]).unwrap(),
            0x22 => panic!("Illegal instruction (write to RD)"),
            0x23 => panic!("Illegal instruction (write to RNG)"),
            addr if addr >= 0x30 => self.ram[addr as usize - 0x30] = val,
            _ => panic!("Illegal instruction (reserved address)"),
        }

    }

    fn read(&mut self, addr: Word) -> Word {

        match addr {
            0    => 0,
            reg @ 0x10 ..= 0x1f => self.regs[reg as usize - 0x10],
            0x21 => panic!("Illegal Instruction (read from WR)"),
            0x22 => {
                let mut buf: [u8; 1] = [0];
                io::stdin().read_exact(&mut buf).unwrap();
                buf[0]
            },
            0x23 => rand::thread_rng().gen(),
            addr if addr >= 0x30 => self.ram[addr as usize - 0x30],
            _ => panic!("Illegal instruction (reserved address)"),
        }
        
    }

}

pub struct CPU {
    bus: Bus,
    pc: Word
}

impl CPU {

    pub fn new(bus: Bus) -> Self {
        CPU { bus, pc: 0x30 }
    }

    fn fetch(&mut self) -> Word {
        let r = self.bus.read(self.pc);
        self.pc += 1;
        r
    }

    pub fn tick(&mut self) {
        let a = self.fetch();
        let b = self.fetch();
        let c = self.fetch();

        let x = self.bus.read(a);
        let y = self.bus.read(b);
        let r = y - x;
        self.bus.write(b, r);

        if r <= 0 {
            self.pc = c
        }

    }

}
