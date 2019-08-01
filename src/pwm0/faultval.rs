#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FAULTVAL {
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
pub struct PWM_FAULTVAL_PWM0R {
    bits: bool,
}
impl PWM_FAULTVAL_PWM0R {
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
pub struct _PWM_FAULTVAL_PWM0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_FAULTVAL_PWM0W<'a> {
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
pub struct PWM_FAULTVAL_PWM1R {
    bits: bool,
}
impl PWM_FAULTVAL_PWM1R {
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
pub struct _PWM_FAULTVAL_PWM1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_FAULTVAL_PWM1W<'a> {
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
pub struct PWM_FAULTVAL_PWM2R {
    bits: bool,
}
impl PWM_FAULTVAL_PWM2R {
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
pub struct _PWM_FAULTVAL_PWM2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_FAULTVAL_PWM2W<'a> {
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
pub struct PWM_FAULTVAL_PWM3R {
    bits: bool,
}
impl PWM_FAULTVAL_PWM3R {
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
pub struct _PWM_FAULTVAL_PWM3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_FAULTVAL_PWM3W<'a> {
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
pub struct PWM_FAULTVAL_PWM4R {
    bits: bool,
}
impl PWM_FAULTVAL_PWM4R {
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
pub struct _PWM_FAULTVAL_PWM4W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_FAULTVAL_PWM4W<'a> {
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
pub struct PWM_FAULTVAL_PWM5R {
    bits: bool,
}
impl PWM_FAULTVAL_PWM5R {
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
pub struct _PWM_FAULTVAL_PWM5W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_FAULTVAL_PWM5W<'a> {
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
pub struct PWM_FAULTVAL_PWM6R {
    bits: bool,
}
impl PWM_FAULTVAL_PWM6R {
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
pub struct _PWM_FAULTVAL_PWM6W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_FAULTVAL_PWM6W<'a> {
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
pub struct PWM_FAULTVAL_PWM7R {
    bits: bool,
}
impl PWM_FAULTVAL_PWM7R {
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
pub struct _PWM_FAULTVAL_PWM7W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_FAULTVAL_PWM7W<'a> {
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
    #[doc = "Bit 0 - MnPWM0 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm0(&self) -> PWM_FAULTVAL_PWM0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        PWM_FAULTVAL_PWM0R { bits }
    }
    #[doc = "Bit 1 - MnPWM1 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm1(&self) -> PWM_FAULTVAL_PWM1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        PWM_FAULTVAL_PWM1R { bits }
    }
    #[doc = "Bit 2 - MnPWM2 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm2(&self) -> PWM_FAULTVAL_PWM2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        PWM_FAULTVAL_PWM2R { bits }
    }
    #[doc = "Bit 3 - MnPWM3 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm3(&self) -> PWM_FAULTVAL_PWM3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        PWM_FAULTVAL_PWM3R { bits }
    }
    #[doc = "Bit 4 - MnPWM4 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm4(&self) -> PWM_FAULTVAL_PWM4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        PWM_FAULTVAL_PWM4R { bits }
    }
    #[doc = "Bit 5 - MnPWM5 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm5(&self) -> PWM_FAULTVAL_PWM5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        PWM_FAULTVAL_PWM5R { bits }
    }
    #[doc = "Bit 6 - MnPWM6 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm6(&self) -> PWM_FAULTVAL_PWM6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        PWM_FAULTVAL_PWM6R { bits }
    }
    #[doc = "Bit 7 - MnPWM7 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm7(&self) -> PWM_FAULTVAL_PWM7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        PWM_FAULTVAL_PWM7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - MnPWM0 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm0(&mut self) -> _PWM_FAULTVAL_PWM0W {
        _PWM_FAULTVAL_PWM0W { w: self }
    }
    #[doc = "Bit 1 - MnPWM1 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm1(&mut self) -> _PWM_FAULTVAL_PWM1W {
        _PWM_FAULTVAL_PWM1W { w: self }
    }
    #[doc = "Bit 2 - MnPWM2 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm2(&mut self) -> _PWM_FAULTVAL_PWM2W {
        _PWM_FAULTVAL_PWM2W { w: self }
    }
    #[doc = "Bit 3 - MnPWM3 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm3(&mut self) -> _PWM_FAULTVAL_PWM3W {
        _PWM_FAULTVAL_PWM3W { w: self }
    }
    #[doc = "Bit 4 - MnPWM4 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm4(&mut self) -> _PWM_FAULTVAL_PWM4W {
        _PWM_FAULTVAL_PWM4W { w: self }
    }
    #[doc = "Bit 5 - MnPWM5 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm5(&mut self) -> _PWM_FAULTVAL_PWM5W {
        _PWM_FAULTVAL_PWM5W { w: self }
    }
    #[doc = "Bit 6 - MnPWM6 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm6(&mut self) -> _PWM_FAULTVAL_PWM6W {
        _PWM_FAULTVAL_PWM6W { w: self }
    }
    #[doc = "Bit 7 - MnPWM7 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm7(&mut self) -> _PWM_FAULTVAL_PWM7W {
        _PWM_FAULTVAL_PWM7W { w: self }
    }
}
