#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Data"]
    pub dr: DR,
    _reserved_1_ecr: [u8; 4usize],
    _reserved2: [u8; 16usize],
    #[doc = "0x18 - UART Flag"]
    pub fr: FR,
    _reserved3: [u8; 4usize],
    #[doc = "0x20 - UART IrDA Low-Power Register"]
    pub ilpr: ILPR,
    #[doc = "0x24 - UART Integer Baud-Rate Divisor"]
    pub ibrd: IBRD,
    #[doc = "0x28 - UART Fractional Baud-Rate Divisor"]
    pub fbrd: FBRD,
    #[doc = "0x2c - UART Line Control"]
    pub lcrh: LCRH,
    #[doc = "0x30 - UART Control"]
    pub ctl: CTL,
    #[doc = "0x34 - UART Interrupt FIFO Level Select"]
    pub ifls: IFLS,
    #[doc = "0x38 - UART Interrupt Mask"]
    pub im: IM,
    #[doc = "0x3c - UART Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x40 - UART Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x44 - UART Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x48 - UART DMA Control"]
    pub dmactl: DMACTL,
    _reserved14: [u8; 88usize],
    #[doc = "0xa4 - UART 9-Bit Self Address"]
    pub _9bitaddr: _9BITADDR,
    #[doc = "0xa8 - UART 9-Bit Self Address Mask"]
    pub _9bitamask: _9BITAMASK,
    _reserved16: [u8; 3860usize],
    #[doc = "0xfc0 - UART Peripheral Properties"]
    pub pp: PP,
    _reserved17: [u8; 4usize],
    #[doc = "0xfc8 - UART Clock Configuration"]
    pub cc: CC,
}
impl RegisterBlock {
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub fn ecr(&self) -> &ECR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const ECR) }
    }
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub fn ecr_mut(&self) -> &mut ECR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut ECR) }
    }
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub fn rsr(&self) -> &RSR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const RSR) }
    }
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub fn rsr_mut(&self) -> &mut RSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut RSR) }
    }
}
#[doc = "UART Data"]
pub struct DR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Data"]
pub mod dr;
#[doc = "UART Receive Status/Error Clear"]
pub struct RSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Receive Status/Error Clear"]
pub mod rsr;
#[doc = "UART Receive Status/Error Clear"]
pub struct ECR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Receive Status/Error Clear"]
pub mod ecr;
#[doc = "UART Flag"]
pub struct FR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Flag"]
pub mod fr;
#[doc = "UART IrDA Low-Power Register"]
pub struct ILPR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART IrDA Low-Power Register"]
pub mod ilpr;
#[doc = "UART Integer Baud-Rate Divisor"]
pub struct IBRD {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Integer Baud-Rate Divisor"]
pub mod ibrd;
#[doc = "UART Fractional Baud-Rate Divisor"]
pub struct FBRD {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Fractional Baud-Rate Divisor"]
pub mod fbrd;
#[doc = "UART Line Control"]
pub struct LCRH {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Line Control"]
pub mod lcrh;
#[doc = "UART Control"]
pub struct CTL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Control"]
pub mod ctl;
#[doc = "UART Interrupt FIFO Level Select"]
pub struct IFLS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Interrupt FIFO Level Select"]
pub mod ifls;
#[doc = "UART Interrupt Mask"]
pub struct IM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Interrupt Mask"]
pub mod im;
#[doc = "UART Raw Interrupt Status"]
pub struct RIS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Raw Interrupt Status"]
pub mod ris;
#[doc = "UART Masked Interrupt Status"]
pub struct MIS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Masked Interrupt Status"]
pub mod mis;
#[doc = "UART Interrupt Clear"]
pub struct ICR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Interrupt Clear"]
pub mod icr;
#[doc = "UART DMA Control"]
pub struct DMACTL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART DMA Control"]
pub mod dmactl;
#[doc = "UART 9-Bit Self Address"]
pub struct _9BITADDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART 9-Bit Self Address"]
pub mod _9bitaddr;
#[doc = "UART 9-Bit Self Address Mask"]
pub struct _9BITAMASK {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART 9-Bit Self Address Mask"]
pub mod _9bitamask;
#[doc = "UART Peripheral Properties"]
pub struct PP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Peripheral Properties"]
pub mod pp;
#[doc = "UART Clock Configuration"]
pub struct CC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UART Clock Configuration"]
pub mod cc;
