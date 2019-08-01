#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MIS {
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
pub struct SYSEXC_MIS_FPIDCMISR {
    bits: bool,
}
impl SYSEXC_MIS_FPIDCMISR {
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
pub struct _SYSEXC_MIS_FPIDCMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_MIS_FPIDCMISW<'a> {
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
pub struct SYSEXC_MIS_FPDZCMISR {
    bits: bool,
}
impl SYSEXC_MIS_FPDZCMISR {
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
pub struct _SYSEXC_MIS_FPDZCMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_MIS_FPDZCMISW<'a> {
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
pub struct SYSEXC_MIS_FPIOCMISR {
    bits: bool,
}
impl SYSEXC_MIS_FPIOCMISR {
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
pub struct _SYSEXC_MIS_FPIOCMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_MIS_FPIOCMISW<'a> {
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
pub struct SYSEXC_MIS_FPUFCMISR {
    bits: bool,
}
impl SYSEXC_MIS_FPUFCMISR {
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
pub struct _SYSEXC_MIS_FPUFCMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_MIS_FPUFCMISW<'a> {
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
pub struct SYSEXC_MIS_FPOFCMISR {
    bits: bool,
}
impl SYSEXC_MIS_FPOFCMISR {
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
pub struct _SYSEXC_MIS_FPOFCMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_MIS_FPOFCMISW<'a> {
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
pub struct SYSEXC_MIS_FPIXCMISR {
    bits: bool,
}
impl SYSEXC_MIS_FPIXCMISR {
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
pub struct _SYSEXC_MIS_FPIXCMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSEXC_MIS_FPIXCMISW<'a> {
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
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpidcmis(&self) -> SYSEXC_MIS_FPIDCMISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSEXC_MIS_FPIDCMISR { bits }
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpdzcmis(&self) -> SYSEXC_MIS_FPDZCMISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSEXC_MIS_FPDZCMISR { bits }
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpiocmis(&self) -> SYSEXC_MIS_FPIOCMISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSEXC_MIS_FPIOCMISR { bits }
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpufcmis(&self) -> SYSEXC_MIS_FPUFCMISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSEXC_MIS_FPUFCMISR { bits }
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpofcmis(&self) -> SYSEXC_MIS_FPOFCMISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSEXC_MIS_FPOFCMISR { bits }
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpixcmis(&self) -> SYSEXC_MIS_FPIXCMISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSEXC_MIS_FPIXCMISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpidcmis(&mut self) -> _SYSEXC_MIS_FPIDCMISW {
        _SYSEXC_MIS_FPIDCMISW { w: self }
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpdzcmis(&mut self) -> _SYSEXC_MIS_FPDZCMISW {
        _SYSEXC_MIS_FPDZCMISW { w: self }
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpiocmis(&mut self) -> _SYSEXC_MIS_FPIOCMISW {
        _SYSEXC_MIS_FPIOCMISW { w: self }
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpufcmis(&mut self) -> _SYSEXC_MIS_FPUFCMISW {
        _SYSEXC_MIS_FPUFCMISW { w: self }
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpofcmis(&mut self) -> _SYSEXC_MIS_FPOFCMISW {
        _SYSEXC_MIS_FPOFCMISW { w: self }
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpixcmis(&mut self) -> _SYSEXC_MIS_FPIXCMISW {
        _SYSEXC_MIS_FPIXCMISW { w: self }
    }
}
