#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADDR1H {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_ADDR1H_ADDRHIR {
    bits: u16,
}
impl EMAC_ADDR1H_ADDRHIR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_ADDR1H_ADDRHIW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_ADDR1H_ADDRHIW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(65535 << 0);
        self.w.bits |= ((value as u32) & 65535) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_ADDR1H_MBCR {
    bits: u8,
}
impl EMAC_ADDR1H_MBCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_ADDR1H_MBCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_ADDR1H_MBCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 24);
        self.w.bits |= ((value as u32) & 63) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_ADDR1H_SAR {
    bits: bool,
}
impl EMAC_ADDR1H_SAR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_ADDR1H_SAW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_ADDR1H_SAW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 30);
        self.w.bits |= ((value as u32) & 1) << 30;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_ADDR1H_AER {
    bits: bool,
}
impl EMAC_ADDR1H_AER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_ADDR1H_AEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_ADDR1H_AEW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 31);
        self.w.bits |= ((value as u32) & 1) << 31;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]"]
    #[inline(always)]
    pub fn emac_addr1h_addrhi(&self) -> EMAC_ADDR1H_ADDRHIR {
        let bits = ((self.bits >> 0) & 65535) as u16;
        EMAC_ADDR1H_ADDRHIR { bits }
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    pub fn emac_addr1h_mbc(&self) -> EMAC_ADDR1H_MBCR {
        let bits = ((self.bits >> 24) & 63) as u8;
        EMAC_ADDR1H_MBCR { bits }
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    pub fn emac_addr1h_sa(&self) -> EMAC_ADDR1H_SAR {
        let bits = ((self.bits >> 30) & 1) != 0;
        EMAC_ADDR1H_SAR { bits }
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn emac_addr1h_ae(&self) -> EMAC_ADDR1H_AER {
        let bits = ((self.bits >> 31) & 1) != 0;
        EMAC_ADDR1H_AER { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\]"]
    #[inline(always)]
    pub fn emac_addr1h_addrhi(&mut self) -> _EMAC_ADDR1H_ADDRHIW {
        _EMAC_ADDR1H_ADDRHIW { w: self }
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    pub fn emac_addr1h_mbc(&mut self) -> _EMAC_ADDR1H_MBCW {
        _EMAC_ADDR1H_MBCW { w: self }
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    pub fn emac_addr1h_sa(&mut self) -> _EMAC_ADDR1H_SAW {
        _EMAC_ADDR1H_SAW { w: self }
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn emac_addr1h_ae(&mut self) -> _EMAC_ADDR1H_AEW {
        _EMAC_ADDR1H_AEW { w: self }
    }
}
