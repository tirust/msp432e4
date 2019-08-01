#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMNANOU {
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
pub struct EMAC_TIMNANOU_TSSSR {
    bits: u32,
}
impl EMAC_TIMNANOU_TSSSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_TIMNANOU_TSSSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMNANOU_TSSSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !(2147483647 << 0);
        self.w.bits |= ((value as u32) & 2147483647) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_TIMNANOU_ADDSUBR {
    bits: bool,
}
impl EMAC_TIMNANOU_ADDSUBR {
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
pub struct _EMAC_TIMNANOU_ADDSUBW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_TIMNANOU_ADDSUBW<'a> {
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
    #[doc = "Bits 0:30 - Timestamp Sub-Second"]
    #[inline(always)]
    pub fn emac_timnanou_tsss(&self) -> EMAC_TIMNANOU_TSSSR {
        let bits = ((self.bits >> 0) & 2147483647) as u32;
        EMAC_TIMNANOU_TSSSR { bits }
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    pub fn emac_timnanou_addsub(&self) -> EMAC_TIMNANOU_ADDSUBR {
        let bits = ((self.bits >> 31) & 1) != 0;
        EMAC_TIMNANOU_ADDSUBR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:30 - Timestamp Sub-Second"]
    #[inline(always)]
    pub fn emac_timnanou_tsss(&mut self) -> _EMAC_TIMNANOU_TSSSW {
        _EMAC_TIMNANOU_TSSSW { w: self }
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    pub fn emac_timnanou_addsub(&mut self) -> _EMAC_TIMNANOU_ADDSUBW {
        _EMAC_TIMNANOU_ADDSUBW { w: self }
    }
}