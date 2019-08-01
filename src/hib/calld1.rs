#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CALLD1 {
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
pub struct _HIB_CALLD1_DOMW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CALLD1_DOMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 0);
        self.w.bits |= ((value as u32) & 31) << 0;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CALLD1_MONW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CALLD1_MONW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 8);
        self.w.bits |= ((value as u32) & 15) << 8;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CALLD1_YEARW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CALLD1_YEARW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(127 << 16);
        self.w.bits |= ((value as u32) & 127) << 16;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CALLD1_DOWW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CALLD1_DOWW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 24);
        self.w.bits |= ((value as u32) & 7) << 24;
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
    #[doc = "Bits 0:4 - Day of Month"]
    #[inline(always)]
    pub fn hib_calld1_dom(&mut self) -> _HIB_CALLD1_DOMW {
        _HIB_CALLD1_DOMW { w: self }
    }
    #[doc = "Bits 8:11 - Month"]
    #[inline(always)]
    pub fn hib_calld1_mon(&mut self) -> _HIB_CALLD1_MONW {
        _HIB_CALLD1_MONW { w: self }
    }
    #[doc = "Bits 16:22 - Year Value"]
    #[inline(always)]
    pub fn hib_calld1_year(&mut self) -> _HIB_CALLD1_YEARW {
        _HIB_CALLD1_YEARW { w: self }
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn hib_calld1_dow(&mut self) -> _HIB_CALLD1_DOWW {
        _HIB_CALLD1_DOWW { w: self }
    }
}
