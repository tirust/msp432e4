#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PP {
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
pub struct PWM_PP_GCNTR {
    bits: u8,
}
impl PWM_PP_GCNTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _PWM_PP_GCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_PP_GCNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct PWM_PP_FCNTR {
    bits: u8,
}
impl PWM_PP_FCNTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _PWM_PP_FCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_PP_FCNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u32) & 15) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct PWM_PP_ESYNCR {
    bits: bool,
}
impl PWM_PP_ESYNCR {
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
pub struct _PWM_PP_ESYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_PP_ESYNCW<'a> {
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
#[doc = r"Value of the field"]
pub struct PWM_PP_EFAULTR {
    bits: bool,
}
impl PWM_PP_EFAULTR {
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
pub struct _PWM_PP_EFAULTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_PP_EFAULTW<'a> {
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
pub struct PWM_PP_ONER {
    bits: bool,
}
impl PWM_PP_ONER {
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
pub struct _PWM_PP_ONEW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_PP_ONEW<'a> {
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
        self.w.bits &= !(1 << 10);
        self.w.bits |= ((value as u32) & 1) << 10;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Generators"]
    #[inline(always)]
    pub fn pwm_pp_gcnt(&self) -> PWM_PP_GCNTR {
        let bits = ((self.bits >> 0) & 15) as u8;
        PWM_PP_GCNTR { bits }
    }
    #[doc = "Bits 4:7 - Fault Inputs (per PWM unit)"]
    #[inline(always)]
    pub fn pwm_pp_fcnt(&self) -> PWM_PP_FCNTR {
        let bits = ((self.bits >> 4) & 15) as u8;
        PWM_PP_FCNTR { bits }
    }
    #[doc = "Bit 8 - Extended Synchronization"]
    #[inline(always)]
    pub fn pwm_pp_esync(&self) -> PWM_PP_ESYNCR {
        let bits = ((self.bits >> 8) & 1) != 0;
        PWM_PP_ESYNCR { bits }
    }
    #[doc = "Bit 9 - Extended Fault"]
    #[inline(always)]
    pub fn pwm_pp_efault(&self) -> PWM_PP_EFAULTR {
        let bits = ((self.bits >> 9) & 1) != 0;
        PWM_PP_EFAULTR { bits }
    }
    #[doc = "Bit 10 - One-Shot Mode"]
    #[inline(always)]
    pub fn pwm_pp_one(&self) -> PWM_PP_ONER {
        let bits = ((self.bits >> 10) & 1) != 0;
        PWM_PP_ONER { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Generators"]
    #[inline(always)]
    pub fn pwm_pp_gcnt(&mut self) -> _PWM_PP_GCNTW {
        _PWM_PP_GCNTW { w: self }
    }
    #[doc = "Bits 4:7 - Fault Inputs (per PWM unit)"]
    #[inline(always)]
    pub fn pwm_pp_fcnt(&mut self) -> _PWM_PP_FCNTW {
        _PWM_PP_FCNTW { w: self }
    }
    #[doc = "Bit 8 - Extended Synchronization"]
    #[inline(always)]
    pub fn pwm_pp_esync(&mut self) -> _PWM_PP_ESYNCW {
        _PWM_PP_ESYNCW { w: self }
    }
    #[doc = "Bit 9 - Extended Fault"]
    #[inline(always)]
    pub fn pwm_pp_efault(&mut self) -> _PWM_PP_EFAULTW {
        _PWM_PP_EFAULTW { w: self }
    }
    #[doc = "Bit 10 - One-Shot Mode"]
    #[inline(always)]
    pub fn pwm_pp_one(&mut self) -> _PWM_PP_ONEW {
        _PWM_PP_ONEW { w: self }
    }
}
