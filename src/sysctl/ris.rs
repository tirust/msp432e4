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
pub struct SYSCTL_RIS_BORRISR {
    bits: bool,
}
impl SYSCTL_RIS_BORRISR {
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
pub struct _SYSCTL_RIS_BORRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RIS_BORRISW<'a> {
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
pub struct SYSCTL_RIS_MOFRISR {
    bits: bool,
}
impl SYSCTL_RIS_MOFRISR {
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
pub struct _SYSCTL_RIS_MOFRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RIS_MOFRISW<'a> {
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
pub struct SYSCTL_RIS_PLLLRISR {
    bits: bool,
}
impl SYSCTL_RIS_PLLLRISR {
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
pub struct _SYSCTL_RIS_PLLLRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RIS_PLLLRISW<'a> {
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
pub struct SYSCTL_RIS_MOSCPUPRISR {
    bits: bool,
}
impl SYSCTL_RIS_MOSCPUPRISR {
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
pub struct _SYSCTL_RIS_MOSCPUPRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RIS_MOSCPUPRISW<'a> {
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
    #[doc = "Bit 1 - Brown-Out Reset Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_borris(&self) -> SYSCTL_RIS_BORRISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_RIS_BORRISR { bits }
    }
    #[doc = "Bit 3 - Main Oscillator Failure Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_mofris(&self) -> SYSCTL_RIS_MOFRISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_RIS_MOFRISR { bits }
    }
    #[doc = "Bit 6 - PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_plllris(&self) -> SYSCTL_RIS_PLLLRISR {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_RIS_PLLLRISR { bits }
    }
    #[doc = "Bit 8 - MOSC Power Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_moscpupris(&self) -> SYSCTL_RIS_MOSCPUPRISR {
        let bits = ((self.bits >> 8) & 1) != 0;
        SYSCTL_RIS_MOSCPUPRISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Brown-Out Reset Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_borris(&mut self) -> _SYSCTL_RIS_BORRISW {
        _SYSCTL_RIS_BORRISW { w: self }
    }
    #[doc = "Bit 3 - Main Oscillator Failure Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_mofris(&mut self) -> _SYSCTL_RIS_MOFRISW {
        _SYSCTL_RIS_MOFRISW { w: self }
    }
    #[doc = "Bit 6 - PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_plllris(&mut self) -> _SYSCTL_RIS_PLLLRISW {
        _SYSCTL_RIS_PLLLRISW { w: self }
    }
    #[doc = "Bit 8 - MOSC Power Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_moscpupris(&mut self) -> _SYSCTL_RIS_MOSCPUPRISW {
        _SYSCTL_RIS_MOSCPUPRISW { w: self }
    }
}
