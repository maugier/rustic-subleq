
use std::io::{self, Read, Write, ErrorKind};
use rand::Rng;


type Word = u8;

pub struct Bus {
    regs: [Word; 16],
    ram: Vec<Word>,
}

pub enum Error {
    IO(io::Error),
    IllegalInstruction(&'static str),
    Halted,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl Bus {

    pub fn new(ram: Vec<Word>) -> Self {
        Bus { regs: [0; 16], ram }
    }

    fn write(&mut self, addr: Word, val: Word) -> Result<(), Error> {

        match addr {
            0 if val == 0 => Err(Error::Halted),
            reg @ 0x10 ..= 0x1f => Ok(self.regs[(reg as usize) - 0x10] = val),
            0x21 => Ok(io::stdout().write_all(&[val as u8])?),
            0x22 => Err(Error::IllegalInstruction("write to RD")),
            0x23 => Err(Error::IllegalInstruction("write to RNG")),
            addr if addr >= 0x30 => Ok(self.ram[addr as usize - 0x30] = val),
            _ => Err(Error::IllegalInstruction("write to reserved")),
        }

    }

    fn read(&mut self, addr: Word) -> Result<Word, io::Error> {

        Ok(match addr {
            reg @ 0x10 ..= 0x1f => self.regs[reg as usize - 0x10],
            0x22 => {
                let mut buf: [u8; 1] = [0];
                io::stdin().read_exact(&mut buf)?;
                buf[0]
            },
            0x23 => rand::thread_rng().gen(),
            addr if addr >= 0x30 => self.ram[addr as usize - 0x30],
            _ => 0,
        })
        
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

    fn fetch(&mut self) -> Result<Word,Error> {
        let r = self.bus.read(self.pc)?;
        self.pc += 1;
        Ok(r)
    }

    pub fn tick(&mut self) -> Result<(), Error> {
        let a = self.fetch()?;
        let b = self.fetch()?;
        let c = self.fetch()?;

        let x = self.bus.read(a)?;
        let y = self.bus.read(b)?;
        let r = y.wrapping_sub(x);
        self.bus.write(b, r)?;

        if r <= 0 {
            self.pc = c
        }

        Ok(())

    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        loop {
            match self.tick() {
                Err(Error::Halted) => return Ok(()),
                Err(Error::IllegalInstruction(e)) => return Err(io::Error::new(ErrorKind::Other, e)),
                Err(Error::IO(ioe)) => return Err(ioe),
                Ok(()) => ()
            }
        }
    }

}
