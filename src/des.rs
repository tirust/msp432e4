#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DES Key 3 LSW for 192-Bit Key"]
    pub key3_l: KEY3_L,
    #[doc = "0x04 - DES Key 3 MSW for 192-Bit Key"]
    pub key3_h: KEY3_H,
    #[doc = "0x08 - DES Key 2 LSW for 128-Bit Key"]
    pub key2_l: KEY2_L,
    #[doc = "0x0c - DES Key 2 MSW for 128-Bit Key"]
    pub key2_h: KEY2_H,
    #[doc = "0x10 - DES Key 1 LSW for 64-Bit Key"]
    pub key1_l: KEY1_L,
    #[doc = "0x14 - DES Key 1 MSW for 64-Bit Key"]
    pub key1_h: KEY1_H,
    #[doc = "0x18 - DES Initialization Vector"]
    pub iv_l: IV_L,
    #[doc = "0x1c - DES Initialization Vector"]
    pub iv_h: IV_H,
    #[doc = "0x20 - DES Control"]
    pub ctrl: CTRL,
    #[doc = "0x24 - DES Cryptographic Data Length"]
    pub length: LENGTH,
    #[doc = "0x28 - DES LSW Data RW"]
    pub data_l: DATA_L,
    #[doc = "0x2c - DES MSW Data RW"]
    pub data_h: DATA_H,
    #[doc = "0x30 - DES Revision Number"]
    pub revision: REVISION,
    #[doc = "0x34 - DES System Configuration"]
    pub sysconfig: SYSCONFIG,
    #[doc = "0x38 - DES System Status"]
    pub sysstatus: SYSSTATUS,
    #[doc = "0x3c - DES Interrupt Status"]
    pub irqstatus: IRQSTATUS,
    #[doc = "0x40 - DES Interrupt Enable"]
    pub irqenable: IRQENABLE,
    #[doc = "0x44 - DES Dirty Bits"]
    pub dirtybits: DIRTYBITS,
}
#[doc = "DES Key 3 LSW for 192-Bit Key"]
pub struct KEY3_L {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Key 3 LSW for 192-Bit Key"]
pub mod key3_l;
#[doc = "DES Key 3 MSW for 192-Bit Key"]
pub struct KEY3_H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Key 3 MSW for 192-Bit Key"]
pub mod key3_h;
#[doc = "DES Key 2 LSW for 128-Bit Key"]
pub struct KEY2_L {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Key 2 LSW for 128-Bit Key"]
pub mod key2_l;
#[doc = "DES Key 2 MSW for 128-Bit Key"]
pub struct KEY2_H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Key 2 MSW for 128-Bit Key"]
pub mod key2_h;
#[doc = "DES Key 1 LSW for 64-Bit Key"]
pub struct KEY1_L {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Key 1 LSW for 64-Bit Key"]
pub mod key1_l;
#[doc = "DES Key 1 MSW for 64-Bit Key"]
pub struct KEY1_H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Key 1 MSW for 64-Bit Key"]
pub mod key1_h;
#[doc = "DES Initialization Vector"]
pub struct IV_L {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Initialization Vector"]
pub mod iv_l;
#[doc = "DES Initialization Vector"]
pub struct IV_H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Initialization Vector"]
pub mod iv_h;
#[doc = "DES Control"]
pub struct CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Control"]
pub mod ctrl;
#[doc = "DES Cryptographic Data Length"]
pub struct LENGTH {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Cryptographic Data Length"]
pub mod length;
#[doc = "DES LSW Data RW"]
pub struct DATA_L {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES LSW Data RW"]
pub mod data_l;
#[doc = "DES MSW Data RW"]
pub struct DATA_H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES MSW Data RW"]
pub mod data_h;
#[doc = "DES Revision Number"]
pub struct REVISION {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Revision Number"]
pub mod revision;
#[doc = "DES System Configuration"]
pub struct SYSCONFIG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES System Configuration"]
pub mod sysconfig;
#[doc = "DES System Status"]
pub struct SYSSTATUS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES System Status"]
pub mod sysstatus;
#[doc = "DES Interrupt Status"]
pub struct IRQSTATUS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Interrupt Status"]
pub mod irqstatus;
#[doc = "DES Interrupt Enable"]
pub struct IRQENABLE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Interrupt Enable"]
pub mod irqenable;
#[doc = "DES Dirty Bits"]
pub struct DIRTYBITS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES Dirty Bits"]
pub mod dirtybits;
