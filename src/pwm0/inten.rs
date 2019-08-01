#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN {
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
pub struct PWM_INTEN_INTPWM0R {
    bits: bool,
}
impl PWM_INTEN_INTPWM0R {
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
pub struct _PWM_INTEN_INTPWM0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INTEN_INTPWM0W<'a> {
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
pub struct PWM_INTEN_INTPWM1R {
    bits: bool,
}
impl PWM_INTEN_INTPWM1R {
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
pub struct _PWM_INTEN_INTPWM1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INTEN_INTPWM1W<'a> {
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
pub struct PWM_INTEN_INTPWM2R {
    bits: bool,
}
impl PWM_INTEN_INTPWM2R {
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
pub struct _PWM_INTEN_INTPWM2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INTEN_INTPWM2W<'a> {
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
pub struct PWM_INTEN_INTPWM3R {
    bits: bool,
}
impl PWM_INTEN_INTPWM3R {
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
pub struct _PWM_INTEN_INTPWM3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INTEN_INTPWM3W<'a> {
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
pub struct PWM_INTEN_INTFAULT0R {
    bits: bool,
}
impl PWM_INTEN_INTFAULT0R {
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
pub struct _PWM_INTEN_INTFAULT0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INTEN_INTFAULT0W<'a> {
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
#[doc = r"Value of the field"]
pub struct PWM_INTEN_INTFAULT1R {
    bits: bool,
}
impl PWM_INTEN_INTFAULT1R {
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
pub struct _PWM_INTEN_INTFAULT1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INTEN_INTFAULT1W<'a> {
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
        self.w.bits &= !(1 << 17);
        self.w.bits |= ((value as u32) & 1) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct PWM_INTEN_INTFAULT2R {
    bits: bool,
}
impl PWM_INTEN_INTFAULT2R {
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
pub struct _PWM_INTEN_INTFAULT2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INTEN_INTFAULT2W<'a> {
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
        self.w.bits &= !(1 << 18);
        self.w.bits |= ((value as u32) & 1) << 18;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct PWM_INTEN_INTFAULT3R {
    bits: bool,
}
impl PWM_INTEN_INTFAULT3R {
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
pub struct _PWM_INTEN_INTFAULT3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_INTEN_INTFAULT3W<'a> {
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
        self.w.bits &= !(1 << 19);
        self.w.bits |= ((value as u32) & 1) << 19;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - PWM0 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm0(&self) -> PWM_INTEN_INTPWM0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        PWM_INTEN_INTPWM0R { bits }
    }
    #[doc = "Bit 1 - PWM1 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm1(&self) -> PWM_INTEN_INTPWM1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        PWM_INTEN_INTPWM1R { bits }
    }
    #[doc = "Bit 2 - PWM2 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm2(&self) -> PWM_INTEN_INTPWM2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        PWM_INTEN_INTPWM2R { bits }
    }
    #[doc = "Bit 3 - PWM3 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm3(&self) -> PWM_INTEN_INTPWM3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        PWM_INTEN_INTPWM3R { bits }
    }
    #[doc = "Bit 16 - Interrupt Fault 0"]
    #[inline(always)]
    pub fn pwm_inten_intfault0(&self) -> PWM_INTEN_INTFAULT0R {
        let bits = ((self.bits >> 16) & 1) != 0;
        PWM_INTEN_INTFAULT0R { bits }
    }
    #[doc = "Bit 17 - Interrupt Fault 1"]
    #[inline(always)]
    pub fn pwm_inten_intfault1(&self) -> PWM_INTEN_INTFAULT1R {
        let bits = ((self.bits >> 17) & 1) != 0;
        PWM_INTEN_INTFAULT1R { bits }
    }
    #[doc = "Bit 18 - Interrupt Fault 2"]
    #[inline(always)]
    pub fn pwm_inten_intfault2(&self) -> PWM_INTEN_INTFAULT2R {
        let bits = ((self.bits >> 18) & 1) != 0;
        PWM_INTEN_INTFAULT2R { bits }
    }
    #[doc = "Bit 19 - Interrupt Fault 3"]
    #[inline(always)]
    pub fn pwm_inten_intfault3(&self) -> PWM_INTEN_INTFAULT3R {
        let bits = ((self.bits >> 19) & 1) != 0;
        PWM_INTEN_INTFAULT3R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - PWM0 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm0(&mut self) -> _PWM_INTEN_INTPWM0W {
        _PWM_INTEN_INTPWM0W { w: self }
    }
    #[doc = "Bit 1 - PWM1 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm1(&mut self) -> _PWM_INTEN_INTPWM1W {
        _PWM_INTEN_INTPWM1W { w: self }
    }
    #[doc = "Bit 2 - PWM2 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm2(&mut self) -> _PWM_INTEN_INTPWM2W {
        _PWM_INTEN_INTPWM2W { w: self }
    }
    #[doc = "Bit 3 - PWM3 Interrupt Enable"]
    #[inline(always)]
    pub fn pwm_inten_intpwm3(&mut self) -> _PWM_INTEN_INTPWM3W {
        _PWM_INTEN_INTPWM3W { w: self }
    }
    #[doc = "Bit 16 - Interrupt Fault 0"]
    #[inline(always)]
    pub fn pwm_inten_intfault0(&mut self) -> _PWM_INTEN_INTFAULT0W {
        _PWM_INTEN_INTFAULT0W { w: self }
    }
    #[doc = "Bit 17 - Interrupt Fault 1"]
    #[inline(always)]
    pub fn pwm_inten_intfault1(&mut self) -> _PWM_INTEN_INTFAULT1W {
        _PWM_INTEN_INTFAULT1W { w: self }
    }
    #[doc = "Bit 18 - Interrupt Fault 2"]
    #[inline(always)]
    pub fn pwm_inten_intfault2(&mut self) -> _PWM_INTEN_INTFAULT2W {
        _PWM_INTEN_INTFAULT2W { w: self }
    }
    #[doc = "Bit 19 - Interrupt Fault 3"]
    #[inline(always)]
    pub fn pwm_inten_intfault3(&mut self) -> _PWM_INTEN_INTFAULT3W {
        _PWM_INTEN_INTFAULT3W { w: self }
    }
}
