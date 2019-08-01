#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::_3_ISC {
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
pub struct PWM_3_ISC_INTCNTZEROR {
    bits: bool,
}
impl PWM_3_ISC_INTCNTZEROR {
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
pub struct _PWM_3_ISC_INTCNTZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_ISC_INTCNTZEROW<'a> {
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
pub struct PWM_3_ISC_INTCNTLOADR {
    bits: bool,
}
impl PWM_3_ISC_INTCNTLOADR {
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
pub struct _PWM_3_ISC_INTCNTLOADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_ISC_INTCNTLOADW<'a> {
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
pub struct PWM_3_ISC_INTCMPAUR {
    bits: bool,
}
impl PWM_3_ISC_INTCMPAUR {
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
pub struct _PWM_3_ISC_INTCMPAUW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_ISC_INTCMPAUW<'a> {
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
pub struct PWM_3_ISC_INTCMPADR {
    bits: bool,
}
impl PWM_3_ISC_INTCMPADR {
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
pub struct _PWM_3_ISC_INTCMPADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_ISC_INTCMPADW<'a> {
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
pub struct PWM_3_ISC_INTCMPBUR {
    bits: bool,
}
impl PWM_3_ISC_INTCMPBUR {
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
pub struct _PWM_3_ISC_INTCMPBUW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_ISC_INTCMPBUW<'a> {
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
pub struct PWM_3_ISC_INTCMPBDR {
    bits: bool,
}
impl PWM_3_ISC_INTCMPBDR {
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
pub struct _PWM_3_ISC_INTCMPBDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_ISC_INTCMPBDW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Counter=0 Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcntzero(&self) -> PWM_3_ISC_INTCNTZEROR {
        let bits = ((self.bits >> 0) & 1) != 0;
        PWM_3_ISC_INTCNTZEROR { bits }
    }
    #[doc = "Bit 1 - Counter=Load Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcntload(&self) -> PWM_3_ISC_INTCNTLOADR {
        let bits = ((self.bits >> 1) & 1) != 0;
        PWM_3_ISC_INTCNTLOADR { bits }
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpau(&self) -> PWM_3_ISC_INTCMPAUR {
        let bits = ((self.bits >> 2) & 1) != 0;
        PWM_3_ISC_INTCMPAUR { bits }
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpad(&self) -> PWM_3_ISC_INTCMPADR {
        let bits = ((self.bits >> 3) & 1) != 0;
        PWM_3_ISC_INTCMPADR { bits }
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpbu(&self) -> PWM_3_ISC_INTCMPBUR {
        let bits = ((self.bits >> 4) & 1) != 0;
        PWM_3_ISC_INTCMPBUR { bits }
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpbd(&self) -> PWM_3_ISC_INTCMPBDR {
        let bits = ((self.bits >> 5) & 1) != 0;
        PWM_3_ISC_INTCMPBDR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Counter=0 Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcntzero(&mut self) -> _PWM_3_ISC_INTCNTZEROW {
        _PWM_3_ISC_INTCNTZEROW { w: self }
    }
    #[doc = "Bit 1 - Counter=Load Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcntload(&mut self) -> _PWM_3_ISC_INTCNTLOADW {
        _PWM_3_ISC_INTCNTLOADW { w: self }
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpau(&mut self) -> _PWM_3_ISC_INTCMPAUW {
        _PWM_3_ISC_INTCMPAUW { w: self }
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpad(&mut self) -> _PWM_3_ISC_INTCMPADW {
        _PWM_3_ISC_INTCMPADW { w: self }
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpbu(&mut self) -> _PWM_3_ISC_INTCMPBUW {
        _PWM_3_ISC_INTCMPBUW { w: self }
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt"]
    #[inline(always)]
    pub fn pwm_3_isc_intcmpbd(&mut self) -> _PWM_3_ISC_INTCMPBDW {
        _PWM_3_ISC_INTCMPBDW { w: self }
    }
}
