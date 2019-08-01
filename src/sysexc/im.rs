#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IM {
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
pub struct SYSEXC_IM_FPIDCIMR {
    bits: bool,
}
impl SYSEXC_IM_FPIDCIMR {
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
pub struct _SYSEXC_IM_FPIDCIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IM_FPIDCIMW<'a> {
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
pub struct SYSEXC_IM_FPDZCIMR {
    bits: bool,
}
impl SYSEXC_IM_FPDZCIMR {
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
pub struct _SYSEXC_IM_FPDZCIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IM_FPDZCIMW<'a> {
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
pub struct SYSEXC_IM_FPIOCIMR {
    bits: bool,
}
impl SYSEXC_IM_FPIOCIMR {
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
pub struct _SYSEXC_IM_FPIOCIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IM_FPIOCIMW<'a> {
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
pub struct SYSEXC_IM_FPUFCIMR {
    bits: bool,
}
impl SYSEXC_IM_FPUFCIMR {
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
pub struct _SYSEXC_IM_FPUFCIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IM_FPUFCIMW<'a> {
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
pub struct SYSEXC_IM_FPOFCIMR {
    bits: bool,
}
impl SYSEXC_IM_FPOFCIMR {
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
pub struct _SYSEXC_IM_FPOFCIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IM_FPOFCIMW<'a> {
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
pub struct SYSEXC_IM_FPIXCIMR {
    bits: bool,
}
impl SYSEXC_IM_FPIXCIMR {
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
pub struct _SYSEXC_IM_FPIXCIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_IM_FPIXCIMW<'a> {
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
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpidcim(&self) -> SYSEXC_IM_FPIDCIMR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSEXC_IM_FPIDCIMR { bits }
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpdzcim(&self) -> SYSEXC_IM_FPDZCIMR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSEXC_IM_FPDZCIMR { bits }
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpiocim(&self) -> SYSEXC_IM_FPIOCIMR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSEXC_IM_FPIOCIMR { bits }
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpufcim(&self) -> SYSEXC_IM_FPUFCIMR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSEXC_IM_FPUFCIMR { bits }
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpofcim(&self) -> SYSEXC_IM_FPOFCIMR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSEXC_IM_FPOFCIMR { bits }
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpixcim(&self) -> SYSEXC_IM_FPIXCIMR {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSEXC_IM_FPIXCIMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpidcim(&mut self) -> _SYSEXC_IM_FPIDCIMW {
        _SYSEXC_IM_FPIDCIMW { w: self }
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpdzcim(&mut self) -> _SYSEXC_IM_FPDZCIMW {
        _SYSEXC_IM_FPDZCIMW { w: self }
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpiocim(&mut self) -> _SYSEXC_IM_FPIOCIMW {
        _SYSEXC_IM_FPIOCIMW { w: self }
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpufcim(&mut self) -> _SYSEXC_IM_FPUFCIMW {
        _SYSEXC_IM_FPUFCIMW { w: self }
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpofcim(&mut self) -> _SYSEXC_IM_FPOFCIMW {
        _SYSEXC_IM_FPOFCIMW { w: self }
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpixcim(&mut self) -> _SYSEXC_IM_FPIXCIMW {
        _SYSEXC_IM_FPIXCIMW { w: self }
    }
}
