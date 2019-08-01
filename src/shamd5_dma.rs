#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SHAMD5 DMA Interrupt Mask"]
    pub shamd5_dmaim: SHAMD5_DMAIM,
    #[doc = "0x04 - SHAMD5 DMA Raw Interrupt Status"]
    pub shamd5_dmaris: SHAMD5_DMARIS,
    #[doc = "0x08 - SHAMD5 DMA Masked Interrupt Status"]
    pub shamd5_dmamis: SHAMD5_DMAMIS,
    #[doc = "0x0c - SHAMD5 DMA Interrupt Clear"]
    pub shamd5_dmaic: SHAMD5_DMAIC,
}
#[doc = "SHAMD5 DMA Interrupt Mask"]
pub struct SHAMD5_DMAIM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHAMD5 DMA Interrupt Mask"]
pub mod shamd5_dmaim;
#[doc = "SHAMD5 DMA Raw Interrupt Status"]
pub struct SHAMD5_DMARIS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHAMD5 DMA Raw Interrupt Status"]
pub mod shamd5_dmaris;
#[doc = "SHAMD5 DMA Masked Interrupt Status"]
pub struct SHAMD5_DMAMIS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHAMD5 DMA Masked Interrupt Status"]
pub mod shamd5_dmamis;
#[doc = "SHAMD5 DMA Interrupt Clear"]
pub struct SHAMD5_DMAIC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SHAMD5 DMA Interrupt Clear"]
pub mod shamd5_dmaic;
