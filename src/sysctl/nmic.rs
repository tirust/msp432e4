#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NMIC {
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
pub struct SYSCTL_NMIC_EXTERNALR {
    bits: bool,
}
impl SYSCTL_NMIC_EXTERNALR {
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
pub struct _SYSCTL_NMIC_EXTERNALW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_NMIC_EXTERNALW<'a> {
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
pub struct SYSCTL_NMIC_POWERR {
    bits: bool,
}
impl SYSCTL_NMIC_POWERR {
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
pub struct _SYSCTL_NMIC_POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_NMIC_POWERW<'a> {
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
pub struct SYSCTL_NMIC_WDT0R {
    bits: bool,
}
impl SYSCTL_NMIC_WDT0R {
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
pub struct _SYSCTL_NMIC_WDT0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_NMIC_WDT0W<'a> {
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
pub struct SYSCTL_NMIC_WDT1R {
    bits: bool,
}
impl SYSCTL_NMIC_WDT1R {
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
pub struct _SYSCTL_NMIC_WDT1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_NMIC_WDT1W<'a> {
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
#[doc = r"Value of the field"]
pub struct SYSCTL_NMIC_TAMPERR {
    bits: bool,
}
impl SYSCTL_NMIC_TAMPERR {
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
pub struct _SYSCTL_NMIC_TAMPERW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_NMIC_TAMPERW<'a> {
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
pub struct SYSCTL_NMIC_MOSCFAILR {
    bits: bool,
}
impl SYSCTL_NMIC_MOSCFAILR {
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
pub struct _SYSCTL_NMIC_MOSCFAILW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_NMIC_MOSCFAILW<'a> {
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
        self.w.bits &= !(1 << 16);
        self.w.bits |= ((value as u32) & 1) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - External Pin NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_external(&self) -> SYSCTL_NMIC_EXTERNALR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_NMIC_EXTERNALR { bits }
    }
    #[doc = "Bit 2 - Power/Brown Out Event NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_power(&self) -> SYSCTL_NMIC_POWERR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_NMIC_POWERR { bits }
    }
    #[doc = "Bit 3 - Watch Dog Timer (WDT) 0 NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_wdt0(&self) -> SYSCTL_NMIC_WDT0R {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_NMIC_WDT0R { bits }
    }
    #[doc = "Bit 5 - Watch Dog Timer (WDT) 1 NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_wdt1(&self) -> SYSCTL_NMIC_WDT1R {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSCTL_NMIC_WDT1R { bits }
    }
    #[doc = "Bit 9 - Tamper Event NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_tamper(&self) -> SYSCTL_NMIC_TAMPERR {
        let bits = ((self.bits >> 9) & 1) != 0;
        SYSCTL_NMIC_TAMPERR { bits }
    }
    #[doc = "Bit 16 - MOSC Failure NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_moscfail(&self) -> SYSCTL_NMIC_MOSCFAILR {
        let bits = ((self.bits >> 16) & 1) != 0;
        SYSCTL_NMIC_MOSCFAILR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - External Pin NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_external(&mut self) -> _SYSCTL_NMIC_EXTERNALW {
        _SYSCTL_NMIC_EXTERNALW { w: self }
    }
    #[doc = "Bit 2 - Power/Brown Out Event NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_power(&mut self) -> _SYSCTL_NMIC_POWERW {
        _SYSCTL_NMIC_POWERW { w: self }
    }
    #[doc = "Bit 3 - Watch Dog Timer (WDT) 0 NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_wdt0(&mut self) -> _SYSCTL_NMIC_WDT0W {
        _SYSCTL_NMIC_WDT0W { w: self }
    }
    #[doc = "Bit 5 - Watch Dog Timer (WDT) 1 NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_wdt1(&mut self) -> _SYSCTL_NMIC_WDT1W {
        _SYSCTL_NMIC_WDT1W { w: self }
    }
    #[doc = "Bit 9 - Tamper Event NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_tamper(&mut self) -> _SYSCTL_NMIC_TAMPERW {
        _SYSCTL_NMIC_TAMPERW { w: self }
    }
    #[doc = "Bit 16 - MOSC Failure NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_moscfail(&mut self) -> _SYSCTL_NMIC_MOSCFAILW {
        _SYSCTL_NMIC_MOSCFAILW { w: self }
    }
}
