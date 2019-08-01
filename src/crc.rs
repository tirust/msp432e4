#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Control"]
    pub ctrl: CTRL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - CRC SEED/Context"]
    pub crcseed: CRCSEED,
    #[doc = "0x14 - CRC Data Input"]
    pub crcdin: CRCDIN,
    #[doc = "0x18 - CRC Post Processing Result"]
    pub crcrsltpp: CRCRSLTPP,
}
#[doc = "CRC Control"]
pub struct CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CRC Control"]
pub mod ctrl;
#[doc = "CRC SEED/Context"]
pub struct CRCSEED {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CRC SEED/Context"]
pub mod crcseed;
#[doc = "CRC Data Input"]
pub struct CRCDIN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CRC Data Input"]
pub mod crcdin;
#[doc = "CRC Post Processing Result"]
pub struct CRCRSLTPP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CRC Post Processing Result"]
pub mod crcrsltpp;
