#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USEBURSTCLR {
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
#[doc = r"Proxy"]
pub struct _UDMA_USEBURSTCLR_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMA_USEBURSTCLR_CLRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !(4294967295 << 0);
        self.w.bits |= ((value as u32) & 4294967295) << 0;
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - Channel \\[n\\] Useburst Clear"]
    #[inline(always)]
    pub fn udma_useburstclr_clr(&mut self) -> _UDMA_USEBURSTCLR_CLRW {
        _UDMA_USEBURSTCLR_CLRW { w: self }
    }
}