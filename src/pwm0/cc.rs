#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CC {
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
#[doc = "Possible values of the field `PWM_CC_PWMDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_CC_PWMDIVR {
    #[doc = "/2"]
    PWM_CC_PWMDIV_2,
    #[doc = "/4"]
    PWM_CC_PWMDIV_4,
    #[doc = "/8"]
    PWM_CC_PWMDIV_8,
    #[doc = "/16"]
    PWM_CC_PWMDIV_16,
    #[doc = "/32"]
    PWM_CC_PWMDIV_32,
    #[doc = "/64"]
    PWM_CC_PWMDIV_64,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_CC_PWMDIVR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_CC_PWMDIVR::PWM_CC_PWMDIV_2 => 0,
            PWM_CC_PWMDIVR::PWM_CC_PWMDIV_4 => 1,
            PWM_CC_PWMDIVR::PWM_CC_PWMDIV_8 => 2,
            PWM_CC_PWMDIVR::PWM_CC_PWMDIV_16 => 3,
            PWM_CC_PWMDIVR::PWM_CC_PWMDIV_32 => 4,
            PWM_CC_PWMDIVR::PWM_CC_PWMDIV_64 => 5,
            PWM_CC_PWMDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_CC_PWMDIVR {
        match value {
            0 => PWM_CC_PWMDIVR::PWM_CC_PWMDIV_2,
            1 => PWM_CC_PWMDIVR::PWM_CC_PWMDIV_4,
            2 => PWM_CC_PWMDIVR::PWM_CC_PWMDIV_8,
            3 => PWM_CC_PWMDIVR::PWM_CC_PWMDIV_16,
            4 => PWM_CC_PWMDIVR::PWM_CC_PWMDIV_32,
            5 => PWM_CC_PWMDIVR::PWM_CC_PWMDIV_64,
            i => PWM_CC_PWMDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_2`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_2(&self) -> bool {
        *self == PWM_CC_PWMDIVR::PWM_CC_PWMDIV_2
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_4`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_4(&self) -> bool {
        *self == PWM_CC_PWMDIVR::PWM_CC_PWMDIV_4
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_8`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_8(&self) -> bool {
        *self == PWM_CC_PWMDIVR::PWM_CC_PWMDIV_8
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_16`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_16(&self) -> bool {
        *self == PWM_CC_PWMDIVR::PWM_CC_PWMDIV_16
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_32`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_32(&self) -> bool {
        *self == PWM_CC_PWMDIVR::PWM_CC_PWMDIV_32
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_64`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_64(&self) -> bool {
        *self == PWM_CC_PWMDIVR::PWM_CC_PWMDIV_64
    }
}
#[doc = "Values that can be written to the field `PWM_CC_PWMDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_CC_PWMDIVW {
    #[doc = "/2"]
    PWM_CC_PWMDIV_2,
    #[doc = "/4"]
    PWM_CC_PWMDIV_4,
    #[doc = "/8"]
    PWM_CC_PWMDIV_8,
    #[doc = "/16"]
    PWM_CC_PWMDIV_16,
    #[doc = "/32"]
    PWM_CC_PWMDIV_32,
    #[doc = "/64"]
    PWM_CC_PWMDIV_64,
}
impl PWM_CC_PWMDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_CC_PWMDIVW::PWM_CC_PWMDIV_2 => 0,
            PWM_CC_PWMDIVW::PWM_CC_PWMDIV_4 => 1,
            PWM_CC_PWMDIVW::PWM_CC_PWMDIV_8 => 2,
            PWM_CC_PWMDIVW::PWM_CC_PWMDIV_16 => 3,
            PWM_CC_PWMDIVW::PWM_CC_PWMDIV_32 => 4,
            PWM_CC_PWMDIVW::PWM_CC_PWMDIV_64 => 5,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_CC_PWMDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_CC_PWMDIVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_CC_PWMDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_2(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIVW::PWM_CC_PWMDIV_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_4(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIVW::PWM_CC_PWMDIV_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_8(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIVW::PWM_CC_PWMDIV_8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_16(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIVW::PWM_CC_PWMDIV_16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_32(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIVW::PWM_CC_PWMDIV_32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_64(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIVW::PWM_CC_PWMDIV_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 0);
        self.w.bits |= ((value as u32) & 7) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct PWM_CC_USEPWMR {
    bits: bool,
}
impl PWM_CC_USEPWMR {
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
pub struct _PWM_CC_USEPWMW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_CC_USEPWMW<'a> {
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
    #[doc = "Bits 0:2 - PWM Clock Divider"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv(&self) -> PWM_CC_PWMDIVR {
        PWM_CC_PWMDIVR::_from(((self.bits >> 0) & 7) as u8)
    }
    #[doc = "Bit 8 - Use PWM Clock Divisor"]
    #[inline(always)]
    pub fn pwm_cc_usepwm(&self) -> PWM_CC_USEPWMR {
        let bits = ((self.bits >> 8) & 1) != 0;
        PWM_CC_USEPWMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - PWM Clock Divider"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv(&mut self) -> _PWM_CC_PWMDIVW {
        _PWM_CC_PWMDIVW { w: self }
    }
    #[doc = "Bit 8 - Use PWM Clock Divisor"]
    #[inline(always)]
    pub fn pwm_cc_usepwm(&mut self) -> _PWM_CC_USEPWMW {
        _PWM_CC_USEPWMW { w: self }
    }
}
