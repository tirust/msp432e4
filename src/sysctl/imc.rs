#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IMC {
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
pub struct SYSCTL_IMC_BORIMR {
    bits: bool,
}
impl SYSCTL_IMC_BORIMR {
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
pub struct _SYSCTL_IMC_BORIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_IMC_BORIMW<'a> {
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
pub struct SYSCTL_IMC_MOFIMR {
    bits: bool,
}
impl SYSCTL_IMC_MOFIMR {
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
pub struct _SYSCTL_IMC_MOFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_IMC_MOFIMW<'a> {
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
pub struct SYSCTL_IMC_PLLLIMR {
    bits: bool,
}
impl SYSCTL_IMC_PLLLIMR {
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
pub struct _SYSCTL_IMC_PLLLIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_IMC_PLLLIMW<'a> {
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
pub struct SYSCTL_IMC_MOSCPUPIMR {
    bits: bool,
}
impl SYSCTL_IMC_MOSCPUPIMR {
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
pub struct _SYSCTL_IMC_MOSCPUPIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_IMC_MOSCPUPIMW<'a> {
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
    #[doc = "Bit 1 - Brown-Out Reset Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_borim(&self) -> SYSCTL_IMC_BORIMR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_IMC_BORIMR { bits }
    }
    #[doc = "Bit 3 - Main Oscillator Failure Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_mofim(&self) -> SYSCTL_IMC_MOFIMR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_IMC_MOFIMR { bits }
    }
    #[doc = "Bit 6 - PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_plllim(&self) -> SYSCTL_IMC_PLLLIMR {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_IMC_PLLLIMR { bits }
    }
    #[doc = "Bit 8 - MOSC Power Up Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_moscpupim(&self) -> SYSCTL_IMC_MOSCPUPIMR {
        let bits = ((self.bits >> 8) & 1) != 0;
        SYSCTL_IMC_MOSCPUPIMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Brown-Out Reset Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_borim(&mut self) -> _SYSCTL_IMC_BORIMW {
        _SYSCTL_IMC_BORIMW { w: self }
    }
    #[doc = "Bit 3 - Main Oscillator Failure Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_mofim(&mut self) -> _SYSCTL_IMC_MOFIMW {
        _SYSCTL_IMC_MOFIMW { w: self }
    }
    #[doc = "Bit 6 - PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_plllim(&mut self) -> _SYSCTL_IMC_PLLLIMW {
        _SYSCTL_IMC_PLLLIMW { w: self }
    }
    #[doc = "Bit 8 - MOSC Power Up Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_moscpupim(&mut self) -> _SYSCTL_IMC_MOSCPUPIMW {
        _SYSCTL_IMC_MOSCPUPIMW { w: self }
    }
}
