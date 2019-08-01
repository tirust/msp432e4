#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIOSCCAL {
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
pub struct SYSCTL_PIOSCCAL_UTR {
    bits: u8,
}
impl SYSCTL_PIOSCCAL_UTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_PIOSCCAL_UTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PIOSCCAL_UTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(127 << 0);
        self.w.bits |= ((value as u32) & 127) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_PIOSCCAL_UPDATER {
    bits: bool,
}
impl SYSCTL_PIOSCCAL_UPDATER {
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
pub struct _SYSCTL_PIOSCCAL_UPDATEW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PIOSCCAL_UPDATEW<'a> {
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
        self.w.bits &= !(1 << 8);
        self.w.bits |= ((value as u32) & 1) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_PIOSCCAL_CALR {
    bits: bool,
}
impl SYSCTL_PIOSCCAL_CALR {
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
pub struct _SYSCTL_PIOSCCAL_CALW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PIOSCCAL_CALW<'a> {
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
        self.w.bits &= !(1 << 9);
        self.w.bits |= ((value as u32) & 1) << 9;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_PIOSCCAL_UTENR {
    bits: bool,
}
impl SYSCTL_PIOSCCAL_UTENR {
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
pub struct _SYSCTL_PIOSCCAL_UTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PIOSCCAL_UTENW<'a> {
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
    #[doc = "Bits 0:6 - User Trim Value"]
    #[inline(always)]
    pub fn sysctl_piosccal_ut(&self) -> SYSCTL_PIOSCCAL_UTR {
        let bits = ((self.bits >> 0) & 127) as u8;
        SYSCTL_PIOSCCAL_UTR { bits }
    }
    #[doc = "Bit 8 - Update Trim"]
    #[inline(always)]
    pub fn sysctl_piosccal_update(&self) -> SYSCTL_PIOSCCAL_UPDATER {
        let bits = ((self.bits >> 8) & 1) != 0;
        SYSCTL_PIOSCCAL_UPDATER { bits }
    }
    #[doc = "Bit 9 - Start Calibration"]
    #[inline(always)]
    pub fn sysctl_piosccal_cal(&self) -> SYSCTL_PIOSCCAL_CALR {
        let bits = ((self.bits >> 9) & 1) != 0;
        SYSCTL_PIOSCCAL_CALR { bits }
    }
    #[doc = "Bit 31 - Use User Trim Value"]
    #[inline(always)]
    pub fn sysctl_piosccal_uten(&self) -> SYSCTL_PIOSCCAL_UTENR {
        let bits = ((self.bits >> 31) & 1) != 0;
        SYSCTL_PIOSCCAL_UTENR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - User Trim Value"]
    #[inline(always)]
    pub fn sysctl_piosccal_ut(&mut self) -> _SYSCTL_PIOSCCAL_UTW {
        _SYSCTL_PIOSCCAL_UTW { w: self }
    }
    #[doc = "Bit 8 - Update Trim"]
    #[inline(always)]
    pub fn sysctl_piosccal_update(&mut self) -> _SYSCTL_PIOSCCAL_UPDATEW {
        _SYSCTL_PIOSCCAL_UPDATEW { w: self }
    }
    #[doc = "Bit 9 - Start Calibration"]
    #[inline(always)]
    pub fn sysctl_piosccal_cal(&mut self) -> _SYSCTL_PIOSCCAL_CALW {
        _SYSCTL_PIOSCCAL_CALW { w: self }
    }
    #[doc = "Bit 31 - Use User Trim Value"]
    #[inline(always)]
    pub fn sysctl_piosccal_uten(&mut self) -> _SYSCTL_PIOSCCAL_UTENW {
        _SYSCTL_PIOSCCAL_UTENW { w: self }
    }
}
