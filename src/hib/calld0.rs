#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CALLD0 {
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
pub struct _HIB_CALLD0_SECW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CALLD0_SECW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 0);
        self.w.bits |= ((value as u32) & 63) << 0;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CALLD0_MINW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CALLD0_MINW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 8);
        self.w.bits |= ((value as u32) & 63) << 8;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CALLD0_HRW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CALLD0_HRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 16);
        self.w.bits |= ((value as u32) & 31) << 16;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CALLD0_AMPMW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CALLD0_AMPMW<'a> {
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
        self.w.bits &= !(1 << 22);
        self.w.bits |= ((value as u32) & 1) << 22;
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
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn hib_calld0_sec(&mut self) -> _HIB_CALLD0_SECW {
        _HIB_CALLD0_SECW { w: self }
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn hib_calld0_min(&mut self) -> _HIB_CALLD0_MINW {
        _HIB_CALLD0_MINW { w: self }
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hib_calld0_hr(&mut self) -> _HIB_CALLD0_HRW {
        _HIB_CALLD0_HRW { w: self }
    }
    #[doc = "Bit 22 - AM/PM Designation"]
    #[inline(always)]
    pub fn hib_calld0_ampm(&mut self) -> _HIB_CALLD0_AMPMW {
        _HIB_CALLD0_AMPMW { w: self }
    }
}
