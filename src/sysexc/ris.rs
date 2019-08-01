#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RIS {
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
pub struct SYSEXC_RIS_FPIDCRISR {
    bits: bool,
}
impl SYSEXC_RIS_FPIDCRISR {
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
pub struct _SYSEXC_RIS_FPIDCRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_RIS_FPIDCRISW<'a> {
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
#[doc = r"Value of the field"]
pub struct SYSEXC_RIS_FPDZCRISR {
    bits: bool,
}
impl SYSEXC_RIS_FPDZCRISR {
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
pub struct _SYSEXC_RIS_FPDZCRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_RIS_FPDZCRISW<'a> {
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
#[doc = r"Value of the field"]
pub struct SYSEXC_RIS_FPIOCRISR {
    bits: bool,
}
impl SYSEXC_RIS_FPIOCRISR {
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
pub struct _SYSEXC_RIS_FPIOCRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_RIS_FPIOCRISW<'a> {
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
#[doc = r"Value of the field"]
pub struct SYSEXC_RIS_FPUFCRISR {
    bits: bool,
}
impl SYSEXC_RIS_FPUFCRISR {
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
pub struct _SYSEXC_RIS_FPUFCRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_RIS_FPUFCRISW<'a> {
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
#[doc = r"Value of the field"]
pub struct SYSEXC_RIS_FPOFCRISR {
    bits: bool,
}
impl SYSEXC_RIS_FPOFCRISR {
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
pub struct _SYSEXC_RIS_FPOFCRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_RIS_FPOFCRISW<'a> {
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
#[doc = r"Value of the field"]
pub struct SYSEXC_RIS_FPIXCRISR {
    bits: bool,
}
impl SYSEXC_RIS_FPIXCRISR {
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
pub struct _SYSEXC_RIS_FPIXCRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_RIS_FPIXCRISW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpidcris(&self) -> SYSEXC_RIS_FPIDCRISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSEXC_RIS_FPIDCRISR { bits }
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpdzcris(&self) -> SYSEXC_RIS_FPDZCRISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSEXC_RIS_FPDZCRISR { bits }
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpiocris(&self) -> SYSEXC_RIS_FPIOCRISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSEXC_RIS_FPIOCRISR { bits }
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpufcris(&self) -> SYSEXC_RIS_FPUFCRISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSEXC_RIS_FPUFCRISR { bits }
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpofcris(&self) -> SYSEXC_RIS_FPOFCRISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSEXC_RIS_FPOFCRISR { bits }
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpixcris(&self) -> SYSEXC_RIS_FPIXCRISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSEXC_RIS_FPIXCRISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpidcris(&mut self) -> _SYSEXC_RIS_FPIDCRISW {
        _SYSEXC_RIS_FPIDCRISW { w: self }
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpdzcris(&mut self) -> _SYSEXC_RIS_FPDZCRISW {
        _SYSEXC_RIS_FPDZCRISW { w: self }
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpiocris(&mut self) -> _SYSEXC_RIS_FPIOCRISW {
        _SYSEXC_RIS_FPIOCRISW { w: self }
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpufcris(&mut self) -> _SYSEXC_RIS_FPUFCRISW {
        _SYSEXC_RIS_FPUFCRISW { w: self }
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpofcris(&mut self) -> _SYSEXC_RIS_FPOFCRISW {
        _SYSEXC_RIS_FPOFCRISW { w: self }
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpixcris(&mut self) -> _SYSEXC_RIS_FPIXCRISW {
        _SYSEXC_RIS_FPIXCRISW { w: self }
    }
}
