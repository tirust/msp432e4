#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MFBOC {
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
pub struct EMAC_MFBOC_MISFRMCNTR {
    bits: u16,
}
impl EMAC_MFBOC_MISFRMCNTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MFBOC_MISFRMCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MFBOC_MISFRMCNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(65535 << 0);
        self.w.bits |= ((value as u32) & 65535) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MFBOC_MISCNTOVFR {
    bits: bool,
}
impl EMAC_MFBOC_MISCNTOVFR {
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
pub struct _EMAC_MFBOC_MISCNTOVFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MFBOC_MISCNTOVFW<'a> {
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
        self.w.bits &= !(1 << 16);
        self.w.bits |= ((value as u32) & 1) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MFBOC_OVFFRMCNTR {
    bits: u16,
}
impl EMAC_MFBOC_OVFFRMCNTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MFBOC_OVFFRMCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MFBOC_OVFFRMCNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(2047 << 17);
        self.w.bits |= ((value as u32) & 2047) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MFBOC_OVFCNTOVFR {
    bits: bool,
}
impl EMAC_MFBOC_OVFCNTOVFR {
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
pub struct _EMAC_MFBOC_OVFCNTOVFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MFBOC_OVFCNTOVFW<'a> {
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
        self.w.bits &= !(1 << 28);
        self.w.bits |= ((value as u32) & 1) << 28;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Missed Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_misfrmcnt(&self) -> EMAC_MFBOC_MISFRMCNTR {
        let bits = ((self.bits >> 0) & 65535) as u16;
        EMAC_MFBOC_MISFRMCNTR { bits }
    }
    #[doc = "Bit 16 - Overflow bit for Missed Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_miscntovf(&self) -> EMAC_MFBOC_MISCNTOVFR {
        let bits = ((self.bits >> 16) & 1) != 0;
        EMAC_MFBOC_MISCNTOVFR { bits }
    }
    #[doc = "Bits 17:27 - Overflow Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_ovffrmcnt(&self) -> EMAC_MFBOC_OVFFRMCNTR {
        let bits = ((self.bits >> 17) & 2047) as u16;
        EMAC_MFBOC_OVFFRMCNTR { bits }
    }
    #[doc = "Bit 28 - Overflow Bit for FIFO Overflow Counter"]
    #[inline(always)]
    pub fn emac_mfboc_ovfcntovf(&self) -> EMAC_MFBOC_OVFCNTOVFR {
        let bits = ((self.bits >> 28) & 1) != 0;
        EMAC_MFBOC_OVFCNTOVFR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Missed Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_misfrmcnt(&mut self) -> _EMAC_MFBOC_MISFRMCNTW {
        _EMAC_MFBOC_MISFRMCNTW { w: self }
    }
    #[doc = "Bit 16 - Overflow bit for Missed Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_miscntovf(&mut self) -> _EMAC_MFBOC_MISCNTOVFW {
        _EMAC_MFBOC_MISCNTOVFW { w: self }
    }
    #[doc = "Bits 17:27 - Overflow Frame Counter"]
    #[inline(always)]
    pub fn emac_mfboc_ovffrmcnt(&mut self) -> _EMAC_MFBOC_OVFFRMCNTW {
        _EMAC_MFBOC_OVFFRMCNTW { w: self }
    }
    #[doc = "Bit 28 - Overflow Bit for FIFO Overflow Counter"]
    #[inline(always)]
    pub fn emac_mfboc_ovfcntovf(&mut self) -> _EMAC_MFBOC_OVFCNTOVFW {
        _EMAC_MFBOC_OVFCNTOVFW { w: self }
    }
}
