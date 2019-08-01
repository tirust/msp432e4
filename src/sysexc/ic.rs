#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IC {
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
pub struct _SYSEXC_IC_FPIDCICW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IC_FPIDCICW<'a> {
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
        self.w.bits &= !(1 << 0);
        self.w.bits |= ((value as u32) & 1) << 0;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _SYSEXC_IC_FPDZCICW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IC_FPDZCICW<'a> {
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
        self.w.bits &= !(1 << 1);
        self.w.bits |= ((value as u32) & 1) << 1;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _SYSEXC_IC_FPIOCICW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IC_FPIOCICW<'a> {
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
        self.w.bits &= !(1 << 2);
        self.w.bits |= ((value as u32) & 1) << 2;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _SYSEXC_IC_FPUFCICW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IC_FPUFCICW<'a> {
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
        self.w.bits &= !(1 << 3);
        self.w.bits |= ((value as u32) & 1) << 3;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _SYSEXC_IC_FPOFCICW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IC_FPOFCICW<'a> {
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
        self.w.bits &= !(1 << 4);
        self.w.bits |= ((value as u32) & 1) << 4;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _SYSEXC_IC_FPIXCICW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IC_FPIXCICW<'a> {
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
        self.w.bits &= !(1 << 5);
        self.w.bits |= ((value as u32) & 1) << 5;
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
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpidcic(&mut self) -> _SYSEXC_IC_FPIDCICW {
        _SYSEXC_IC_FPIDCICW { w: self }
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpdzcic(&mut self) -> _SYSEXC_IC_FPDZCICW {
        _SYSEXC_IC_FPDZCICW { w: self }
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpiocic(&mut self) -> _SYSEXC_IC_FPIOCICW {
        _SYSEXC_IC_FPIOCICW { w: self }
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpufcic(&mut self) -> _SYSEXC_IC_FPUFCICW {
        _SYSEXC_IC_FPUFCICW { w: self }
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpofcic(&mut self) -> _SYSEXC_IC_FPOFCICW {
        _SYSEXC_IC_FPOFCICW { w: self }
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpixcic(&mut self) -> _SYSEXC_IC_FPIXCICW {
        _SYSEXC_IC_FPIXCICW { w: self }
    }
}
