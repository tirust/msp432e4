#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cryptographic Modules Clock Gating Request"]
    pub ccmcgreq: CCMCGREQ,
}
#[doc = "Cryptographic Modules Clock Gating Request"]
pub struct CCMCGREQ {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Cryptographic Modules Clock Gating Request"]
pub mod ccmcgreq;
