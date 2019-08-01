#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BAUD2 {
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
pub struct EPI_BAUD2_COUNT0R {
    bits: u16,
}
impl EPI_BAUD2_COUNT0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EPI_BAUD2_COUNT0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_BAUD2_COUNT0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(65535 << 0);
        self.w.bits |= ((value as u32) & 65535) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_BAUD2_COUNT1R {
    bits: u16,
}
impl EPI_BAUD2_COUNT1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EPI_BAUD2_COUNT1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_BAUD2_COUNT1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(65535 << 16);
        self.w.bits |= ((value as u32) & 65535) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - CS2n Baud Rate Counter 0"]
    #[inline(always)]
    pub fn epi_baud2_count0(&self) -> EPI_BAUD2_COUNT0R {
        let bits = ((self.bits >> 0) & 65535) as u16;
        EPI_BAUD2_COUNT0R { bits }
    }
    #[doc = "Bits 16:31 - CS3n Baud Rate Counter 1"]
    #[inline(always)]
    pub fn epi_baud2_count1(&self) -> EPI_BAUD2_COUNT1R {
        let bits = ((self.bits >> 16) & 65535) as u16;
        EPI_BAUD2_COUNT1R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - CS2n Baud Rate Counter 0"]
    #[inline(always)]
    pub fn epi_baud2_count0(&mut self) -> _EPI_BAUD2_COUNT0W {
        _EPI_BAUD2_COUNT0W { w: self }
    }
    #[doc = "Bits 16:31 - CS3n Baud Rate Counter 1"]
    #[inline(always)]
    pub fn epi_baud2_count1(&mut self) -> _EPI_BAUD2_COUNT1W {
        _EPI_BAUD2_COUNT1W { w: self }
    }
}
