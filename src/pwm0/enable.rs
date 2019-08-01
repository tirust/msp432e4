#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENABLE {
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
pub struct PWM_ENABLE_PWM0ENR {
    bits: bool,
}
impl PWM_ENABLE_PWM0ENR {
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
pub struct _PWM_ENABLE_PWM0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENABLE_PWM0ENW<'a> {
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
pub struct PWM_ENABLE_PWM1ENR {
    bits: bool,
}
impl PWM_ENABLE_PWM1ENR {
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
pub struct _PWM_ENABLE_PWM1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENABLE_PWM1ENW<'a> {
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
pub struct PWM_ENABLE_PWM2ENR {
    bits: bool,
}
impl PWM_ENABLE_PWM2ENR {
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
pub struct _PWM_ENABLE_PWM2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENABLE_PWM2ENW<'a> {
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
pub struct PWM_ENABLE_PWM3ENR {
    bits: bool,
}
impl PWM_ENABLE_PWM3ENR {
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
pub struct _PWM_ENABLE_PWM3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENABLE_PWM3ENW<'a> {
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
pub struct PWM_ENABLE_PWM4ENR {
    bits: bool,
}
impl PWM_ENABLE_PWM4ENR {
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
pub struct _PWM_ENABLE_PWM4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENABLE_PWM4ENW<'a> {
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
pub struct PWM_ENABLE_PWM5ENR {
    bits: bool,
}
impl PWM_ENABLE_PWM5ENR {
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
pub struct _PWM_ENABLE_PWM5ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENABLE_PWM5ENW<'a> {
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
pub struct PWM_ENABLE_PWM6ENR {
    bits: bool,
}
impl PWM_ENABLE_PWM6ENR {
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
pub struct _PWM_ENABLE_PWM6ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENABLE_PWM6ENW<'a> {
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
pub struct PWM_ENABLE_PWM7ENR {
    bits: bool,
}
impl PWM_ENABLE_PWM7ENR {
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
pub struct _PWM_ENABLE_PWM7ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENABLE_PWM7ENW<'a> {
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
    #[doc = "Bit 0 - MnPWM0 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm0en(&self) -> PWM_ENABLE_PWM0ENR {
        let bits = ((self.bits >> 0) & 1) != 0;
        PWM_ENABLE_PWM0ENR { bits }
    }
    #[doc = "Bit 1 - MnPWM1 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm1en(&self) -> PWM_ENABLE_PWM1ENR {
        let bits = ((self.bits >> 1) & 1) != 0;
        PWM_ENABLE_PWM1ENR { bits }
    }
    #[doc = "Bit 2 - MnPWM2 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm2en(&self) -> PWM_ENABLE_PWM2ENR {
        let bits = ((self.bits >> 2) & 1) != 0;
        PWM_ENABLE_PWM2ENR { bits }
    }
    #[doc = "Bit 3 - MnPWM3 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm3en(&self) -> PWM_ENABLE_PWM3ENR {
        let bits = ((self.bits >> 3) & 1) != 0;
        PWM_ENABLE_PWM3ENR { bits }
    }
    #[doc = "Bit 4 - MnPWM4 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm4en(&self) -> PWM_ENABLE_PWM4ENR {
        let bits = ((self.bits >> 4) & 1) != 0;
        PWM_ENABLE_PWM4ENR { bits }
    }
    #[doc = "Bit 5 - MnPWM5 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm5en(&self) -> PWM_ENABLE_PWM5ENR {
        let bits = ((self.bits >> 5) & 1) != 0;
        PWM_ENABLE_PWM5ENR { bits }
    }
    #[doc = "Bit 6 - MnPWM6 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm6en(&self) -> PWM_ENABLE_PWM6ENR {
        let bits = ((self.bits >> 6) & 1) != 0;
        PWM_ENABLE_PWM6ENR { bits }
    }
    #[doc = "Bit 7 - MnPWM7 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm7en(&self) -> PWM_ENABLE_PWM7ENR {
        let bits = ((self.bits >> 7) & 1) != 0;
        PWM_ENABLE_PWM7ENR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - MnPWM0 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm0en(&mut self) -> _PWM_ENABLE_PWM0ENW {
        _PWM_ENABLE_PWM0ENW { w: self }
    }
    #[doc = "Bit 1 - MnPWM1 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm1en(&mut self) -> _PWM_ENABLE_PWM1ENW {
        _PWM_ENABLE_PWM1ENW { w: self }
    }
    #[doc = "Bit 2 - MnPWM2 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm2en(&mut self) -> _PWM_ENABLE_PWM2ENW {
        _PWM_ENABLE_PWM2ENW { w: self }
    }
    #[doc = "Bit 3 - MnPWM3 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm3en(&mut self) -> _PWM_ENABLE_PWM3ENW {
        _PWM_ENABLE_PWM3ENW { w: self }
    }
    #[doc = "Bit 4 - MnPWM4 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm4en(&mut self) -> _PWM_ENABLE_PWM4ENW {
        _PWM_ENABLE_PWM4ENW { w: self }
    }
    #[doc = "Bit 5 - MnPWM5 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm5en(&mut self) -> _PWM_ENABLE_PWM5ENW {
        _PWM_ENABLE_PWM5ENW { w: self }
    }
    #[doc = "Bit 6 - MnPWM6 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm6en(&mut self) -> _PWM_ENABLE_PWM6ENW {
        _PWM_ENABLE_PWM6ENW { w: self }
    }
    #[doc = "Bit 7 - MnPWM7 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm7en(&mut self) -> _PWM_ENABLE_PWM7ENW {
        _PWM_ENABLE_PWM7ENW { w: self }
    }
}
