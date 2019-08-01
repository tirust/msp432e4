#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INVERT {
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
pub struct PWM_INVERT_PWM0INVR {
    bits: bool,
}
impl PWM_INVERT_PWM0INVR {
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
pub struct _PWM_INVERT_PWM0INVW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INVERT_PWM0INVW<'a> {
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
pub struct PWM_INVERT_PWM1INVR {
    bits: bool,
}
impl PWM_INVERT_PWM1INVR {
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
pub struct _PWM_INVERT_PWM1INVW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INVERT_PWM1INVW<'a> {
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
pub struct PWM_INVERT_PWM2INVR {
    bits: bool,
}
impl PWM_INVERT_PWM2INVR {
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
pub struct _PWM_INVERT_PWM2INVW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INVERT_PWM2INVW<'a> {
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
pub struct PWM_INVERT_PWM3INVR {
    bits: bool,
}
impl PWM_INVERT_PWM3INVR {
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
pub struct _PWM_INVERT_PWM3INVW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INVERT_PWM3INVW<'a> {
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
pub struct PWM_INVERT_PWM4INVR {
    bits: bool,
}
impl PWM_INVERT_PWM4INVR {
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
pub struct _PWM_INVERT_PWM4INVW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INVERT_PWM4INVW<'a> {
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
pub struct PWM_INVERT_PWM5INVR {
    bits: bool,
}
impl PWM_INVERT_PWM5INVR {
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
pub struct _PWM_INVERT_PWM5INVW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INVERT_PWM5INVW<'a> {
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
pub struct PWM_INVERT_PWM6INVR {
    bits: bool,
}
impl PWM_INVERT_PWM6INVR {
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
pub struct _PWM_INVERT_PWM6INVW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INVERT_PWM6INVW<'a> {
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
pub struct PWM_INVERT_PWM7INVR {
    bits: bool,
}
impl PWM_INVERT_PWM7INVR {
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
pub struct _PWM_INVERT_PWM7INVW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INVERT_PWM7INVW<'a> {
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
        self.w.bits &= !(1 << 7);
        self.w.bits |= ((value as u32) & 1) << 7;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Invert MnPWM0 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm0inv(&self) -> PWM_INVERT_PWM0INVR {
        let bits = ((self.bits >> 0) & 1) != 0;
        PWM_INVERT_PWM0INVR { bits }
    }
    #[doc = "Bit 1 - Invert MnPWM1 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm1inv(&self) -> PWM_INVERT_PWM1INVR {
        let bits = ((self.bits >> 1) & 1) != 0;
        PWM_INVERT_PWM1INVR { bits }
    }
    #[doc = "Bit 2 - Invert MnPWM2 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm2inv(&self) -> PWM_INVERT_PWM2INVR {
        let bits = ((self.bits >> 2) & 1) != 0;
        PWM_INVERT_PWM2INVR { bits }
    }
    #[doc = "Bit 3 - Invert MnPWM3 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm3inv(&self) -> PWM_INVERT_PWM3INVR {
        let bits = ((self.bits >> 3) & 1) != 0;
        PWM_INVERT_PWM3INVR { bits }
    }
    #[doc = "Bit 4 - Invert MnPWM4 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm4inv(&self) -> PWM_INVERT_PWM4INVR {
        let bits = ((self.bits >> 4) & 1) != 0;
        PWM_INVERT_PWM4INVR { bits }
    }
    #[doc = "Bit 5 - Invert MnPWM5 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm5inv(&self) -> PWM_INVERT_PWM5INVR {
        let bits = ((self.bits >> 5) & 1) != 0;
        PWM_INVERT_PWM5INVR { bits }
    }
    #[doc = "Bit 6 - Invert MnPWM6 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm6inv(&self) -> PWM_INVERT_PWM6INVR {
        let bits = ((self.bits >> 6) & 1) != 0;
        PWM_INVERT_PWM6INVR { bits }
    }
    #[doc = "Bit 7 - Invert MnPWM7 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm7inv(&self) -> PWM_INVERT_PWM7INVR {
        let bits = ((self.bits >> 7) & 1) != 0;
        PWM_INVERT_PWM7INVR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Invert MnPWM0 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm0inv(&mut self) -> _PWM_INVERT_PWM0INVW {
        _PWM_INVERT_PWM0INVW { w: self }
    }
    #[doc = "Bit 1 - Invert MnPWM1 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm1inv(&mut self) -> _PWM_INVERT_PWM1INVW {
        _PWM_INVERT_PWM1INVW { w: self }
    }
    #[doc = "Bit 2 - Invert MnPWM2 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm2inv(&mut self) -> _PWM_INVERT_PWM2INVW {
        _PWM_INVERT_PWM2INVW { w: self }
    }
    #[doc = "Bit 3 - Invert MnPWM3 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm3inv(&mut self) -> _PWM_INVERT_PWM3INVW {
        _PWM_INVERT_PWM3INVW { w: self }
    }
    #[doc = "Bit 4 - Invert MnPWM4 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm4inv(&mut self) -> _PWM_INVERT_PWM4INVW {
        _PWM_INVERT_PWM4INVW { w: self }
    }
    #[doc = "Bit 5 - Invert MnPWM5 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm5inv(&mut self) -> _PWM_INVERT_PWM5INVW {
        _PWM_INVERT_PWM5INVW { w: self }
    }
    #[doc = "Bit 6 - Invert MnPWM6 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm6inv(&mut self) -> _PWM_INVERT_PWM6INVW {
        _PWM_INVERT_PWM6INVW { w: self }
    }
    #[doc = "Bit 7 - Invert MnPWM7 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm7inv(&mut self) -> _PWM_INVERT_PWM7INVW {
        _PWM_INVERT_PWM7INVW { w: self }
    }
}
