#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DES DMA Interrupt Mask"]
    pub des_dmaim: DES_DMAIM,
    #[doc = "0x04 - DES DMA Raw Interrupt Status"]
    pub des_dmaris: DES_DMARIS,
    #[doc = "0x08 - DES DMA Masked Interrupt Status"]
    pub des_dmamis: DES_DMAMIS,
    #[doc = "0x0c - DES DMA Interrupt Clear"]
    pub des_dmaic: DES_DMAIC,
}
#[doc = "DES DMA Interrupt Mask"]
pub struct DES_DMAIM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES DMA Interrupt Mask"]
pub mod des_dmaim;
#[doc = "DES DMA Raw Interrupt Status"]
pub struct DES_DMARIS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES DMA Raw Interrupt Status"]
pub mod des_dmaris;
#[doc = "DES DMA Masked Interrupt Status"]
pub struct DES_DMAMIS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES DMA Masked Interrupt Status"]
pub mod des_dmamis;
#[doc = "DES DMA Interrupt Clear"]
pub struct DES_DMAIC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DES DMA Interrupt Clear"]
pub mod des_dmaic;
