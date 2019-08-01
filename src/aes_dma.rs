#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES DMA Interrupt Mask"]
    pub aes_dmaim: AES_DMAIM,
    #[doc = "0x04 - AES DMA Raw Interrupt Status"]
    pub aes_dmaris: AES_DMARIS,
    #[doc = "0x08 - AES DMA Masked Interrupt Status"]
    pub aes_dmamis: AES_DMAMIS,
    #[doc = "0x0c - AES DMA Interrupt Clear"]
    pub aes_dmaic: AES_DMAIC,
}
#[doc = "AES DMA Interrupt Mask"]
pub struct AES_DMAIM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES DMA Interrupt Mask"]
pub mod aes_dmaim;
#[doc = "AES DMA Raw Interrupt Status"]
pub struct AES_DMARIS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES DMA Raw Interrupt Status"]
pub mod aes_dmaris;
#[doc = "AES DMA Masked Interrupt Status"]
pub struct AES_DMAMIS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES DMA Masked Interrupt Status"]
pub mod aes_dmamis;
#[doc = "AES DMA Interrupt Clear"]
pub struct AES_DMAIC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AES DMA Interrupt Clear"]
pub mod aes_dmaic;
