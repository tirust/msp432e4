#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SHA Outer Digest A"]
    pub odigest_a: ODIGEST_A,
    #[doc = "0x04 - SHA Outer Digest B"]
    pub odigest_b: ODIGEST_B,
    #[doc = "0x08 - SHA Outer Digest C"]
    pub odigest_c: ODIGEST_C,
    #[doc = "0x0c - SHA Outer Digest D"]
    pub odigest_d: ODIGEST_D,
    #[doc = "0x10 - SHA Outer Digest E"]
    pub odigest_e: ODIGEST_E,
    #[doc = "0x14 - SHA Outer Digest F"]
    pub odigest_f: ODIGEST_F,
    #[doc = "0x18 - SHA Outer Digest G"]
    pub odigest_g: ODIGEST_G,
    #[doc = "0x1c - SHA Outer Digest H"]
    pub odigest_h: ODIGEST_H,
    #[doc = "0x20 - SHA Inner Digest A"]
    pub idigest_a: IDIGEST_A,
    #[doc = "0x24 - SHA Inner Digest B"]
    pub idigest_b: IDIGEST_B,
    #[doc = "0x28 - SHA Inner Digest C"]
    pub idigest_c: IDIGEST_C,
    #[doc = "0x2c - SHA Inner Digest D"]
    pub idigest_d: IDIGEST_D,
    #[doc = "0x30 - SHA Inner Digest E"]
    pub idigest_e: IDIGEST_E,
    #[doc = "0x34 - SHA Inner Digest F"]
    pub idigest_f: IDIGEST_F,
    #[doc = "0x38 - SHA Inner Digest G"]
    pub idigest_g: IDIGEST_G,
    #[doc = "0x3c - SHA Inner Digest H"]
    pub idigest_h: IDIGEST_H,
    #[doc = "0x40 - SHA Digest Count"]
    pub digest_count: DIGEST_COUNT,
    #[doc = "0x44 - SHA Mode"]
    pub mode: MODE,
    #[doc = "0x48 - SHA Length"]
    pub length: LENGTH,
    _reserved19: [u8; 52usize],
    #[doc = "0x80 - SHA Data 0 Input"]
    pub data_0_in: DATA_0_IN,
    #[doc = "0x84 - SHA Data 1 Input"]
    pub data_1_in: DATA_1_IN,
    #[doc = "0x88 - SHA Data 2 Input"]
    pub data_2_in: DATA_2_IN,
    #[doc = "0x8c - SHA Data 3 Input"]
    pub data_3_in: DATA_3_IN,
    #[doc = "0x90 - SHA Data 4 Input"]
    pub data_4_in: DATA_4_IN,
    #[doc = "0x94 - SHA Data 5 Input"]
    pub data_5_in: DATA_5_IN,
    #[doc = "0x98 - SHA Data 6 Input"]
    pub data_6_in: DATA_6_IN,
    #[doc = "0x9c - SHA Data 7 Input"]
    pub data_7_in: DATA_7_IN,
    #[doc = "0xa0 - SHA Data 8 Input"]
    pub data_8_in: DATA_8_IN,
    #[doc = "0xa4 - SHA Data 9 Input"]
    pub data_9_in: DATA_9_IN,
    #[doc = "0xa8 - SHA Data 10 Input"]
    pub data_10_in: DATA_10_IN,
    #[doc = "0xac - SHA Data 11 Input"]
    pub data_11_in: DATA_11_IN,
    #[doc = "0xb0 - SHA Data 12 Input"]
    pub data_12_in: DATA_12_IN,
    #[doc = "0xb4 - SHA Data 13 Input"]
    pub data_13_in: DATA_13_IN,
    #[doc = "0xb8 - SHA Data 14 Input"]
    pub data_14_in: DATA_14_IN,
    #[doc = "0xbc - SHA Data 15 Input"]
    pub data_15_in: DATA_15_IN,
    _reserved35: [u8; 64usize],
    #[doc = "0x100 - SHA Revision"]
    pub revision: REVISION,
    _reserved36: [u8; 12usize],
    #[doc = "0x110 - SHA System Configuration"]
    pub sysconfig: SYSCONFIG,
    #[doc = "0x114 - SHA System Status"]
    pub sysstatus: SYSSTATUS,
    #[doc = "0x118 - SHA Interrupt Status"]
    pub irqstatus: IRQSTATUS,
    #[doc = "0x11c - SHA Interrupt Enable"]
    pub irqenable: IRQENABLE,
}
#[doc = "SHA Outer Digest A"]
pub struct ODIGEST_A {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Outer Digest A"]
pub mod odigest_a;
#[doc = "SHA Outer Digest B"]
pub struct ODIGEST_B {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Outer Digest B"]
pub mod odigest_b;
#[doc = "SHA Outer Digest C"]
pub struct ODIGEST_C {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Outer Digest C"]
pub mod odigest_c;
#[doc = "SHA Outer Digest D"]
pub struct ODIGEST_D {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Outer Digest D"]
pub mod odigest_d;
#[doc = "SHA Outer Digest E"]
pub struct ODIGEST_E {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Outer Digest E"]
pub mod odigest_e;
#[doc = "SHA Outer Digest F"]
pub struct ODIGEST_F {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Outer Digest F"]
pub mod odigest_f;
#[doc = "SHA Outer Digest G"]
pub struct ODIGEST_G {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Outer Digest G"]
pub mod odigest_g;
#[doc = "SHA Outer Digest H"]
pub struct ODIGEST_H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Outer Digest H"]
pub mod odigest_h;
#[doc = "SHA Inner Digest A"]
pub struct IDIGEST_A {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Inner Digest A"]
pub mod idigest_a;
#[doc = "SHA Inner Digest B"]
pub struct IDIGEST_B {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Inner Digest B"]
pub mod idigest_b;
#[doc = "SHA Inner Digest C"]
pub struct IDIGEST_C {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Inner Digest C"]
pub mod idigest_c;
#[doc = "SHA Inner Digest D"]
pub struct IDIGEST_D {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Inner Digest D"]
pub mod idigest_d;
#[doc = "SHA Inner Digest E"]
pub struct IDIGEST_E {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Inner Digest E"]
pub mod idigest_e;
#[doc = "SHA Inner Digest F"]
pub struct IDIGEST_F {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Inner Digest F"]
pub mod idigest_f;
#[doc = "SHA Inner Digest G"]
pub struct IDIGEST_G {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Inner Digest G"]
pub mod idigest_g;
#[doc = "SHA Inner Digest H"]
pub struct IDIGEST_H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Inner Digest H"]
pub mod idigest_h;
#[doc = "SHA Digest Count"]
pub struct DIGEST_COUNT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Digest Count"]
pub mod digest_count;
#[doc = "SHA Mode"]
pub struct MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Mode"]
pub mod mode;
#[doc = "SHA Length"]
pub struct LENGTH {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Length"]
pub mod length;
#[doc = "SHA Data 0 Input"]
pub struct DATA_0_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 0 Input"]
pub mod data_0_in;
#[doc = "SHA Data 1 Input"]
pub struct DATA_1_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 1 Input"]
pub mod data_1_in;
#[doc = "SHA Data 2 Input"]
pub struct DATA_2_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 2 Input"]
pub mod data_2_in;
#[doc = "SHA Data 3 Input"]
pub struct DATA_3_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 3 Input"]
pub mod data_3_in;
#[doc = "SHA Data 4 Input"]
pub struct DATA_4_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 4 Input"]
pub mod data_4_in;
#[doc = "SHA Data 5 Input"]
pub struct DATA_5_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 5 Input"]
pub mod data_5_in;
#[doc = "SHA Data 6 Input"]
pub struct DATA_6_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 6 Input"]
pub mod data_6_in;
#[doc = "SHA Data 7 Input"]
pub struct DATA_7_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 7 Input"]
pub mod data_7_in;
#[doc = "SHA Data 8 Input"]
pub struct DATA_8_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 8 Input"]
pub mod data_8_in;
#[doc = "SHA Data 9 Input"]
pub struct DATA_9_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 9 Input"]
pub mod data_9_in;
#[doc = "SHA Data 10 Input"]
pub struct DATA_10_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 10 Input"]
pub mod data_10_in;
#[doc = "SHA Data 11 Input"]
pub struct DATA_11_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 11 Input"]
pub mod data_11_in;
#[doc = "SHA Data 12 Input"]
pub struct DATA_12_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 12 Input"]
pub mod data_12_in;
#[doc = "SHA Data 13 Input"]
pub struct DATA_13_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 13 Input"]
pub mod data_13_in;
#[doc = "SHA Data 14 Input"]
pub struct DATA_14_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 14 Input"]
pub mod data_14_in;
#[doc = "SHA Data 15 Input"]
pub struct DATA_15_IN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Data 15 Input"]
pub mod data_15_in;
#[doc = "SHA Revision"]
pub struct REVISION {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Revision"]
pub mod revision;
#[doc = "SHA System Configuration"]
pub struct SYSCONFIG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA System Configuration"]
pub mod sysconfig;
#[doc = "SHA System Status"]
pub struct SYSSTATUS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA System Status"]
pub mod sysstatus;
#[doc = "SHA Interrupt Status"]
pub struct IRQSTATUS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Interrupt Status"]
pub mod irqstatus;
#[doc = "SHA Interrupt Enable"]
pub struct IRQENABLE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHA Interrupt Enable"]
pub mod irqenable;
