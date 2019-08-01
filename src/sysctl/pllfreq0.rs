#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLLFREQ0 {
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
pub struct SYSCTL_PLLFREQ0_MINTR {
    bits: u16,
}
impl SYSCTL_PLLFREQ0_MINTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_PLLFREQ0_MINTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PLLFREQ0_MINTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(1023 << 0);
        self.w.bits |= ((value as u32) & 1023) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_PLLFREQ0_MFRACR {
    bits: u16,
}
impl SYSCTL_PLLFREQ0_MFRACR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_PLLFREQ0_MFRACW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PLLFREQ0_MFRACW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(1023 << 10);
        self.w.bits |= ((value as u32) & 1023) << 10;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_PLLFREQ0_PLLPWRR {
    bits: bool,
}
impl SYSCTL_PLLFREQ0_PLLPWRR {
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
pub struct _SYSCTL_PLLFREQ0_PLLPWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PLLFREQ0_PLLPWRW<'a> {
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
        self.w.bits &= !(1 << 23);
        self.w.bits |= ((value as u32) & 1) << 23;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - PLL M Integer Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_mint(&self) -> SYSCTL_PLLFREQ0_MINTR {
        let bits = ((self.bits >> 0) & 1023) as u16;
        SYSCTL_PLLFREQ0_MINTR { bits }
    }
    #[doc = "Bits 10:19 - PLL M Fractional Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_mfrac(&self) -> SYSCTL_PLLFREQ0_MFRACR {
        let bits = ((self.bits >> 10) & 1023) as u16;
        SYSCTL_PLLFREQ0_MFRACR { bits }
    }
    #[doc = "Bit 23 - PLL Power"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_pllpwr(&self) -> SYSCTL_PLLFREQ0_PLLPWRR {
        let bits = ((self.bits >> 23) & 1) != 0;
        SYSCTL_PLLFREQ0_PLLPWRR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - PLL M Integer Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_mint(&mut self) -> _SYSCTL_PLLFREQ0_MINTW {
        _SYSCTL_PLLFREQ0_MINTW { w: self }
    }
    #[doc = "Bits 10:19 - PLL M Fractional Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_mfrac(&mut self) -> _SYSCTL_PLLFREQ0_MFRACW {
        _SYSCTL_PLLFREQ0_MFRACW { w: self }
    }
    #[doc = "Bit 23 - PLL Power"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_pllpwr(&mut self) -> _SYSCTL_PLLFREQ0_PLLPWRW {
        _SYSCTL_PLLFREQ0_PLLPWRW { w: self }
    }
}
