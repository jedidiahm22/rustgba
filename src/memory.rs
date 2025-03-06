pub enum MemoryTypes {
    Rom,
    VRam,
    SRam,
    WRam,
    OAm,
    HRam,
}

pub struct Ram {
    data: Vec<u8>,
    readable: bool,
    writable: bool,
}

impl Ram {
    pub fn new(size: u16) -> Ram {
        Ram {
            data: vec![0; size as usize],
            readable: true,
            writable: true,
        }
    }

    pub fn new(size: u16, ram_type: MemoryTypes) -> Ram {
        if ram_type == MemoryTypes::Rom {
            Ram {
                data: vec![0; size as usize],
                readable: true,
                writable: false,
            }
        } else {
            Ram {
                data: vec![0; size as usize],
                readable: true,
                writable: true,
            }
        }
    }

    pub fn set_readable(&mut self, readable: bool) {
        self.readable = readable;
    }

    pub fn set_writable(&mut self, writable: bool) {
        self.writable = writable;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    pub fn writeu16(&mut self, addr: u16, value: u16) {
        self.data[addr as usize] = (value & 0x00ff) as u8;
        self.data[(addr + 1) as usize] = (value & 0xff00) as u8;
    }

    pub fn readu16(&self, addr: u16) -> u16 {
        (self.data[addr as usize] as u16) | ((self.data[(addr + 1) as usize] as u16) << 8)
    }
}


pub struct MemmoryMapper {
    regions: Vec<(u16, u16, Ram)>,
}

impl MemmoryMapper {
    pub fn new() -> MemmoryMapper {
        MemmoryMapper {
            regions: Vec::new(),
        }
    }

    pub fn add_region(&mut self, start: u16, end: u16, ram: Ram) {
        self.regions.push((start, end, ram));
    }

    pub fn find_region(&self, addr: u16) -> Option<&Ram> {
        for region in &self.regions {
            if addr >= region.0 && addr <= region.1 {
                return Some(&region.2);
            }
        }
        None
    }

    pub fn read(&self, addr: u16) -> u8 {
        if let Some(ram) = self.find_region(addr) {
            if(ram.readable == true) {
                ram.read(addr)
            } else {
                return 0;
            }
        } else {
            0
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if (let Some(ram) = self.find_region(addr)) && (ram.writable == true) {
            ram.write(addr, value);
        }
    }

    pub fn readu16(&self, addr: u16) -> u16 {
        if let Some(ram) = self.find_region(addr) {
            ram.readu16(addr)
        } else {
            0
        }
    }

    pub fn writeu16(&mut self, addr: u16, value: u16) {
        if (let Some(ram) = self.find_region(addr)) && (ram.writable == true) {
            ram.writeu16(addr, value);
        }
    }
}