#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::_1_FLTSEN {
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
pub struct PWM_1_FLTSEN_FAULT0R {
    bits: bool,
}
impl PWM_1_FLTSEN_FAULT0R {
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
pub struct _PWM_1_FLTSEN_FAULT0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_1_FLTSEN_FAULT0W<'a> {
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
pub struct PWM_1_FLTSEN_FAULT1R {
    bits: bool,
}
impl PWM_1_FLTSEN_FAULT1R {
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
pub struct _PWM_1_FLTSEN_FAULT1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_1_FLTSEN_FAULT1W<'a> {
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
pub struct PWM_1_FLTSEN_FAULT2R {
    bits: bool,
}
impl PWM_1_FLTSEN_FAULT2R {
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
pub struct _PWM_1_FLTSEN_FAULT2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_1_FLTSEN_FAULT2W<'a> {
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
pub struct PWM_1_FLTSEN_FAULT3R {
    bits: bool,
}
impl PWM_1_FLTSEN_FAULT3R {
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
pub struct _PWM_1_FLTSEN_FAULT3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_1_FLTSEN_FAULT3W<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Fault0 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault0(&self) -> PWM_1_FLTSEN_FAULT0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        PWM_1_FLTSEN_FAULT0R { bits }
    }
    #[doc = "Bit 1 - Fault1 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault1(&self) -> PWM_1_FLTSEN_FAULT1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        PWM_1_FLTSEN_FAULT1R { bits }
    }
    #[doc = "Bit 2 - Fault2 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault2(&self) -> PWM_1_FLTSEN_FAULT2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        PWM_1_FLTSEN_FAULT2R { bits }
    }
    #[doc = "Bit 3 - Fault3 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault3(&self) -> PWM_1_FLTSEN_FAULT3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        PWM_1_FLTSEN_FAULT3R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fault0 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault0(&mut self) -> _PWM_1_FLTSEN_FAULT0W {
        _PWM_1_FLTSEN_FAULT0W { w: self }
    }
    #[doc = "Bit 1 - Fault1 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault1(&mut self) -> _PWM_1_FLTSEN_FAULT1W {
        _PWM_1_FLTSEN_FAULT1W { w: self }
    }
    #[doc = "Bit 2 - Fault2 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault2(&mut self) -> _PWM_1_FLTSEN_FAULT2W {
        _PWM_1_FLTSEN_FAULT2W { w: self }
    }
    #[doc = "Bit 3 - Fault3 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault3(&mut self) -> _PWM_1_FLTSEN_FAULT3W {
        _PWM_1_FLTSEN_FAULT3W { w: self }
    }
}
