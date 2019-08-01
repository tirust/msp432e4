#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MOSCCTL {
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
pub struct SYSCTL_MOSCCTL_CVALR {
    bits: bool,
}
impl SYSCTL_MOSCCTL_CVALR {
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
pub struct _SYSCTL_MOSCCTL_CVALW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MOSCCTL_CVALW<'a> {
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
pub struct SYSCTL_MOSCCTL_MOSCIMR {
    bits: bool,
}
impl SYSCTL_MOSCCTL_MOSCIMR {
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
pub struct _SYSCTL_MOSCCTL_MOSCIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MOSCCTL_MOSCIMW<'a> {
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
pub struct SYSCTL_MOSCCTL_NOXTALR {
    bits: bool,
}
impl SYSCTL_MOSCCTL_NOXTALR {
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
pub struct _SYSCTL_MOSCCTL_NOXTALW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MOSCCTL_NOXTALW<'a> {
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
pub struct SYSCTL_MOSCCTL_PWRDNR {
    bits: bool,
}
impl SYSCTL_MOSCCTL_PWRDNR {
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
pub struct _SYSCTL_MOSCCTL_PWRDNW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MOSCCTL_PWRDNW<'a> {
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
pub struct SYSCTL_MOSCCTL_OSCRNGR {
    bits: bool,
}
impl SYSCTL_MOSCCTL_OSCRNGR {
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
pub struct _SYSCTL_MOSCCTL_OSCRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_MOSCCTL_OSCRNGW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Clock Validation for MOSC"]
    #[inline(always)]
    pub fn sysctl_moscctl_cval(&self) -> SYSCTL_MOSCCTL_CVALR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_MOSCCTL_CVALR { bits }
    }
    #[doc = "Bit 1 - MOSC Failure Action"]
    #[inline(always)]
    pub fn sysctl_moscctl_moscim(&self) -> SYSCTL_MOSCCTL_MOSCIMR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_MOSCCTL_MOSCIMR { bits }
    }
    #[doc = "Bit 2 - No Crystal Connected"]
    #[inline(always)]
    pub fn sysctl_moscctl_noxtal(&self) -> SYSCTL_MOSCCTL_NOXTALR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_MOSCCTL_NOXTALR { bits }
    }
    #[doc = "Bit 3 - Power Down"]
    #[inline(always)]
    pub fn sysctl_moscctl_pwrdn(&self) -> SYSCTL_MOSCCTL_PWRDNR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_MOSCCTL_PWRDNR { bits }
    }
    #[doc = "Bit 4 - Oscillator Range"]
    #[inline(always)]
    pub fn sysctl_moscctl_oscrng(&self) -> SYSCTL_MOSCCTL_OSCRNGR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSCTL_MOSCCTL_OSCRNGR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Clock Validation for MOSC"]
    #[inline(always)]
    pub fn sysctl_moscctl_cval(&mut self) -> _SYSCTL_MOSCCTL_CVALW {
        _SYSCTL_MOSCCTL_CVALW { w: self }
    }
    #[doc = "Bit 1 - MOSC Failure Action"]
    #[inline(always)]
    pub fn sysctl_moscctl_moscim(&mut self) -> _SYSCTL_MOSCCTL_MOSCIMW {
        _SYSCTL_MOSCCTL_MOSCIMW { w: self }
    }
    #[doc = "Bit 2 - No Crystal Connected"]
    #[inline(always)]
    pub fn sysctl_moscctl_noxtal(&mut self) -> _SYSCTL_MOSCCTL_NOXTALW {
        _SYSCTL_MOSCCTL_NOXTALW { w: self }
    }
    #[doc = "Bit 3 - Power Down"]
    #[inline(always)]
    pub fn sysctl_moscctl_pwrdn(&mut self) -> _SYSCTL_MOSCCTL_PWRDNW {
        _SYSCTL_MOSCCTL_PWRDNW { w: self }
    }
    #[doc = "Bit 4 - Oscillator Range"]
    #[inline(always)]
    pub fn sysctl_moscctl_oscrng(&mut self) -> _SYSCTL_MOSCCTL_OSCRNGW {
        _SYSCTL_MOSCCTL_OSCRNGW { w: self }
    }
}
