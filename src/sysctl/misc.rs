#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISC {
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
pub struct SYSCTL_MISC_BORMISR {
    bits: bool,
}
impl SYSCTL_MISC_BORMISR {
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
pub struct _SYSCTL_MISC_BORMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MISC_BORMISW<'a> {
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
pub struct SYSCTL_MISC_MOFMISR {
    bits: bool,
}
impl SYSCTL_MISC_MOFMISR {
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
pub struct _SYSCTL_MISC_MOFMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MISC_MOFMISW<'a> {
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
pub struct SYSCTL_MISC_PLLLMISR {
    bits: bool,
}
impl SYSCTL_MISC_PLLLMISR {
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
pub struct _SYSCTL_MISC_PLLLMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MISC_PLLLMISW<'a> {
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
        self.w.bits &= !(1 << 6);
        self.w.bits |= ((value as u32) & 1) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_MISC_MOSCPUPMISR {
    bits: bool,
}
impl SYSCTL_MISC_MOSCPUPMISR {
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
pub struct _SYSCTL_MISC_MOSCPUPMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MISC_MOSCPUPMISW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - BOR Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_bormis(&self) -> SYSCTL_MISC_BORMISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_MISC_BORMISR { bits }
    }
    #[doc = "Bit 3 - Main Oscillator Failure Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_mofmis(&self) -> SYSCTL_MISC_MOFMISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_MISC_MOFMISR { bits }
    }
    #[doc = "Bit 6 - PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_plllmis(&self) -> SYSCTL_MISC_PLLLMISR {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_MISC_PLLLMISR { bits }
    }
    #[doc = "Bit 8 - MOSC Power Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_moscpupmis(&self) -> SYSCTL_MISC_MOSCPUPMISR {
        let bits = ((self.bits >> 8) & 1) != 0;
        SYSCTL_MISC_MOSCPUPMISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - BOR Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_bormis(&mut self) -> _SYSCTL_MISC_BORMISW {
        _SYSCTL_MISC_BORMISW { w: self }
    }
    #[doc = "Bit 3 - Main Oscillator Failure Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_mofmis(&mut self) -> _SYSCTL_MISC_MOFMISW {
        _SYSCTL_MISC_MOFMISW { w: self }
    }
    #[doc = "Bit 6 - PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_plllmis(&mut self) -> _SYSCTL_MISC_PLLLMISW {
        _SYSCTL_MISC_PLLLMISW { w: self }
    }
    #[doc = "Bit 8 - MOSC Power Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_moscpupmis(&mut self) -> _SYSCTL_MISC_MOSCPUPMISW {
        _SYSCTL_MISC_MOSCPUPMISW { w: self }
    }
}
