pub struct Cartridge {
    data: Vec<u8>,
}

pub struct CartridgeRegion {
    start: u16,
    end: u16,
}

pub enum CartridgeType {
    ROM_ONLY,
    MBC1,
    MBC1_RAM,
    MBC1_RAM_BATTERY,
    MBC2,
    MBC2_BATTERY,
    ROM_RAM,
    ROM_RAM_BATTERY,
    MMM01,
    MMM01_RAM,
    MMM01_RAM_BATTERY,
    MBC3_TIMER_BATTERY,
    MBC3_TIMER_RAM_BATTERY,
    MBC3,
    MBC3_RAM,
    MBC3_RAM_BATTERY,
    MBC5,
    MBC5_RAM,
    MBC5_RAM_BATTERY,
    MBC5_RUMBLE,
}

pub enum RomSize {
    32KByte,
    64KByte,
    128KByte,
    256KByte,
    512KByte,
    1MByte,
    2MByte,
    4MByte,
    1_1MByte,
    1_2MByte,
    1_5MByte,
}

const ENTRY_POINT: CartridgeRegion = CartridgeRegion {
    start: 0,
    end: 0x100,
};

const NINTENDO_LOGO: CartridgeRegion = CartridgeRegion {
    start: 0x104,
    end: 0x133,
};

const TITLE: CartridgeRegion = CartridgeRegion {
    start: 0x134,
    end: 0x143,
};

const MANUFACTURER: CartridgeRegion = CartridgeRegion {
    start: 0x13f,
    end: 0x142,
};

const LICENSEE_CODE: CartridgeRegion = CartridgeRegion {
    start: 0x144,
    end: 0x145,
};

const CARTRIDGE_TYPE: CartridgeRegion = CartridgeRegion {
    start: 0x147,
    end: 0x148,
};

const ROM_SIZE: CartridgeRegion = CartridgeRegion {
    start: 0x148,
    end: 0x148,
};

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            data: Vec::new(),
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        self.data = data;
    }

}