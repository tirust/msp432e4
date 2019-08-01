#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES Key 2_6"]
    pub key2_6: KEY2_6,
    #[doc = "0x04 - AES Key 2_7"]
    pub key2_7: KEY2_7,
    #[doc = "0x08 - AES Key 2_4"]
    pub key2_4: KEY2_4,
    #[doc = "0x0c - AES Key 2_5"]
    pub key2_5: KEY2_5,
    #[doc = "0x10 - AES Key 2_2"]
    pub key2_2: KEY2_2,
    #[doc = "0x14 - AES Key 2_3"]
    pub key2_3: KEY2_3,
    #[doc = "0x18 - AES Key 2_0"]
    pub key2_0: KEY2_0,
    #[doc = "0x1c - AES Key 2_1"]
    pub key2_1: KEY2_1,
    #[doc = "0x20 - AES Key 1_6"]
    pub key1_6: KEY1_6,
    #[doc = "0x24 - AES Key 1_7"]
    pub key1_7: KEY1_7,
    #[doc = "0x28 - AES Key 1_4"]
    pub key1_4: KEY1_4,
    #[doc = "0x2c - AES Key 1_5"]
    pub key1_5: KEY1_5,
    #[doc = "0x30 - AES Key 1_2"]
    pub key1_2: KEY1_2,
    #[doc = "0x34 - AES Key 1_3"]
    pub key1_3: KEY1_3,
    #[doc = "0x38 - AES Key 1_0"]
    pub key1_0: KEY1_0,
    #[doc = "0x3c - AES Key 1_1"]
    pub key1_1: KEY1_1,
    #[doc = "0x40 - AES Initialization Vector Input 0"]
    pub iv_in_0: IV_IN_0,
    #[doc = "0x44 - AES Initialization Vector Input 1"]
    pub iv_in_1: IV_IN_1,
    #[doc = "0x48 - AES Initialization Vector Input 2"]
    pub iv_in_2: IV_IN_2,
    #[doc = "0x4c - AES Initialization Vector Input 3"]
    pub iv_in_3: IV_IN_3,
    #[doc = "0x50 - AES Control"]
    pub ctrl: CTRL,
    #[doc = "0x54 - AES Crypto Data Length 0"]
    pub c_length_0: C_LENGTH_0,
    #[doc = "0x58 - AES Crypto Data Length 1"]
    pub c_length_1: C_LENGTH_1,
    #[doc = "0x5c - AES Authentication Data Length"]
    pub auth_length: AUTH_LENGTH,
    #[doc = "0x60 - AES Data RW Plaintext/Ciphertext 0"]
    pub data_in_0: DATA_IN_0,
    #[doc = "0x64 - AES Data RW Plaintext/Ciphertext 1"]
    pub data_in_1: DATA_IN_1,
    #[doc = "0x68 - AES Data RW Plaintext/Ciphertext 2"]
    pub data_in_2: DATA_IN_2,
    #[doc = "0x6c - AES Data RW Plaintext/Ciphertext 3"]
    pub data_in_3: DATA_IN_3,
    #[doc = "0x70 - AES Hash Tag Out 0"]
    pub tag_out_0: TAG_OUT_0,
    #[doc = "0x74 - AES Hash Tag Out 1"]
    pub tag_out_1: TAG_OUT_1,
    #[doc = "0x78 - AES Hash Tag Out 2"]
    pub tag_out_2: TAG_OUT_2,
    #[doc = "0x7c - AES Hash Tag Out 3"]
    pub tag_out_3: TAG_OUT_3,
    #[doc = "0x80 - AES IP Revision Identifier"]
    pub revision: REVISION,
    #[doc = "0x84 - AES System Configuration"]
    pub sysconfig: SYSCONFIG,
    #[doc = "0x88 - AES System Status"]
    pub sysstatus: SYSSTATUS,
    #[doc = "0x8c - AES Interrupt Status"]
    pub irqstatus: IRQSTATUS,
    #[doc = "0x90 - AES Interrupt Enable"]
    pub irqenable: IRQENABLE,
    #[doc = "0x94 - AES Dirty Bits"]
    pub dirtybits: DIRTYBITS,
}
#[doc = "AES Key 2_6"]
pub struct KEY2_6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 2_6"]
pub mod key2_6;
#[doc = "AES Key 2_7"]
pub struct KEY2_7 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 2_7"]
pub mod key2_7;
#[doc = "AES Key 2_4"]
pub struct KEY2_4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 2_4"]
pub mod key2_4;
#[doc = "AES Key 2_5"]
pub struct KEY2_5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 2_5"]
pub mod key2_5;
#[doc = "AES Key 2_2"]
pub struct KEY2_2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 2_2"]
pub mod key2_2;
#[doc = "AES Key 2_3"]
pub struct KEY2_3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 2_3"]
pub mod key2_3;
#[doc = "AES Key 2_0"]
pub struct KEY2_0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 2_0"]
pub mod key2_0;
#[doc = "AES Key 2_1"]
pub struct KEY2_1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 2_1"]
pub mod key2_1;
#[doc = "AES Key 1_6"]
pub struct KEY1_6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 1_6"]
pub mod key1_6;
#[doc = "AES Key 1_7"]
pub struct KEY1_7 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 1_7"]
pub mod key1_7;
#[doc = "AES Key 1_4"]
pub struct KEY1_4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 1_4"]
pub mod key1_4;
#[doc = "AES Key 1_5"]
pub struct KEY1_5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 1_5"]
pub mod key1_5;
#[doc = "AES Key 1_2"]
pub struct KEY1_2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 1_2"]
pub mod key1_2;
#[doc = "AES Key 1_3"]
pub struct KEY1_3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 1_3"]
pub mod key1_3;
#[doc = "AES Key 1_0"]
pub struct KEY1_0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 1_0"]
pub mod key1_0;
#[doc = "AES Key 1_1"]
pub struct KEY1_1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Key 1_1"]
pub mod key1_1;
#[doc = "AES Initialization Vector Input 0"]
pub struct IV_IN_0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Initialization Vector Input 0"]
pub mod iv_in_0;
#[doc = "AES Initialization Vector Input 1"]
pub struct IV_IN_1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Initialization Vector Input 1"]
pub mod iv_in_1;
#[doc = "AES Initialization Vector Input 2"]
pub struct IV_IN_2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Initialization Vector Input 2"]
pub mod iv_in_2;
#[doc = "AES Initialization Vector Input 3"]
pub struct IV_IN_3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Initialization Vector Input 3"]
pub mod iv_in_3;
#[doc = "AES Control"]
pub struct CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Control"]
pub mod ctrl;
#[doc = "AES Crypto Data Length 0"]
pub struct C_LENGTH_0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Crypto Data Length 0"]
pub mod c_length_0;
#[doc = "AES Crypto Data Length 1"]
pub struct C_LENGTH_1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Crypto Data Length 1"]
pub mod c_length_1;
#[doc = "AES Authentication Data Length"]
pub struct AUTH_LENGTH {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Authentication Data Length"]
pub mod auth_length;
#[doc = "AES Data RW Plaintext/Ciphertext 0"]
pub struct DATA_IN_0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Data RW Plaintext/Ciphertext 0"]
pub mod data_in_0;
#[doc = "AES Data RW Plaintext/Ciphertext 1"]
pub struct DATA_IN_1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Data RW Plaintext/Ciphertext 1"]
pub mod data_in_1;
#[doc = "AES Data RW Plaintext/Ciphertext 2"]
pub struct DATA_IN_2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Data RW Plaintext/Ciphertext 2"]
pub mod data_in_2;
#[doc = "AES Data RW Plaintext/Ciphertext 3"]
pub struct DATA_IN_3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Data RW Plaintext/Ciphertext 3"]
pub mod data_in_3;
#[doc = "AES Hash Tag Out 0"]
pub struct TAG_OUT_0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Hash Tag Out 0"]
pub mod tag_out_0;
#[doc = "AES Hash Tag Out 1"]
pub struct TAG_OUT_1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Hash Tag Out 1"]
pub mod tag_out_1;
#[doc = "AES Hash Tag Out 2"]
pub struct TAG_OUT_2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Hash Tag Out 2"]
pub mod tag_out_2;
#[doc = "AES Hash Tag Out 3"]
pub struct TAG_OUT_3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Hash Tag Out 3"]
pub mod tag_out_3;
#[doc = "AES IP Revision Identifier"]
pub struct REVISION {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES IP Revision Identifier"]
pub mod revision;
#[doc = "AES System Configuration"]
pub struct SYSCONFIG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES System Configuration"]
pub mod sysconfig;
#[doc = "AES System Status"]
pub struct SYSSTATUS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES System Status"]
pub mod sysstatus;
#[doc = "AES Interrupt Status"]
pub struct IRQSTATUS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Interrupt Status"]
pub mod irqstatus;
#[doc = "AES Interrupt Enable"]
pub struct IRQENABLE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Interrupt Enable"]
pub mod irqenable;
#[doc = "AES Dirty Bits"]
pub struct DIRTYBITS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES Dirty Bits"]
pub mod dirtybits;
