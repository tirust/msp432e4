#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::_3_INTEN {
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
pub struct PWM_3_INTEN_INTCNTZEROR {
    bits: bool,
}
impl PWM_3_INTEN_INTCNTZEROR {
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
pub struct _PWM_3_INTEN_INTCNTZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_INTCNTZEROW<'a> {
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
pub struct PWM_3_INTEN_INTCNTLOADR {
    bits: bool,
}
impl PWM_3_INTEN_INTCNTLOADR {
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
pub struct _PWM_3_INTEN_INTCNTLOADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_INTCNTLOADW<'a> {
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
pub struct PWM_3_INTEN_INTCMPAUR {
    bits: bool,
}
impl PWM_3_INTEN_INTCMPAUR {
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
pub struct _PWM_3_INTEN_INTCMPAUW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_INTCMPAUW<'a> {
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
pub struct PWM_3_INTEN_INTCMPADR {
    bits: bool,
}
impl PWM_3_INTEN_INTCMPADR {
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
pub struct _PWM_3_INTEN_INTCMPADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_INTCMPADW<'a> {
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
pub struct PWM_3_INTEN_INTCMPBUR {
    bits: bool,
}
impl PWM_3_INTEN_INTCMPBUR {
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
pub struct _PWM_3_INTEN_INTCMPBUW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_INTCMPBUW<'a> {
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
pub struct PWM_3_INTEN_INTCMPBDR {
    bits: bool,
}
impl PWM_3_INTEN_INTCMPBDR {
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
pub struct _PWM_3_INTEN_INTCMPBDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_INTCMPBDW<'a> {
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
pub struct PWM_3_INTEN_TRCNTZEROR {
    bits: bool,
}
impl PWM_3_INTEN_TRCNTZEROR {
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
pub struct _PWM_3_INTEN_TRCNTZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_TRCNTZEROW<'a> {
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
pub struct PWM_3_INTEN_TRCNTLOADR {
    bits: bool,
}
impl PWM_3_INTEN_TRCNTLOADR {
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
pub struct _PWM_3_INTEN_TRCNTLOADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_TRCNTLOADW<'a> {
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
pub struct PWM_3_INTEN_TRCMPAUR {
    bits: bool,
}
impl PWM_3_INTEN_TRCMPAUR {
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
pub struct _PWM_3_INTEN_TRCMPAUW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_TRCMPAUW<'a> {
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
#[doc = r"Value of the field"]
pub struct PWM_3_INTEN_TRCMPADR {
    bits: bool,
}
impl PWM_3_INTEN_TRCMPADR {
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
pub struct _PWM_3_INTEN_TRCMPADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_TRCMPADW<'a> {
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
        self.w.bits &= !(1 << 11);
        self.w.bits |= ((value as u32) & 1) << 11;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct PWM_3_INTEN_TRCMPBUR {
    bits: bool,
}
impl PWM_3_INTEN_TRCMPBUR {
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
pub struct _PWM_3_INTEN_TRCMPBUW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_TRCMPBUW<'a> {
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
        self.w.bits &= !(1 << 12);
        self.w.bits |= ((value as u32) & 1) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct PWM_3_INTEN_TRCMPBDR {
    bits: bool,
}
impl PWM_3_INTEN_TRCMPBDR {
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
pub struct _PWM_3_INTEN_TRCMPBDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_INTEN_TRCMPBDW<'a> {
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
        self.w.bits &= !(1 << 13);
        self.w.bits |= ((value as u32) & 1) << 13;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt for Counter=0"]
    #[inline(always)]
    pub fn pwm_3_inten_intcntzero(&self) -> PWM_3_INTEN_INTCNTZEROR {
        let bits = ((self.bits >> 0) & 1) != 0;
        PWM_3_INTEN_INTCNTZEROR { bits }
    }
    #[doc = "Bit 1 - Interrupt for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_3_inten_intcntload(&self) -> PWM_3_INTEN_INTCNTLOADR {
        let bits = ((self.bits >> 1) & 1) != 0;
        PWM_3_INTEN_INTCNTLOADR { bits }
    }
    #[doc = "Bit 2 - Interrupt for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_3_inten_intcmpau(&self) -> PWM_3_INTEN_INTCMPAUR {
        let bits = ((self.bits >> 2) & 1) != 0;
        PWM_3_INTEN_INTCMPAUR { bits }
    }
    #[doc = "Bit 3 - Interrupt for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_3_inten_intcmpad(&self) -> PWM_3_INTEN_INTCMPADR {
        let bits = ((self.bits >> 3) & 1) != 0;
        PWM_3_INTEN_INTCMPADR { bits }
    }
    #[doc = "Bit 4 - Interrupt for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_3_inten_intcmpbu(&self) -> PWM_3_INTEN_INTCMPBUR {
        let bits = ((self.bits >> 4) & 1) != 0;
        PWM_3_INTEN_INTCMPBUR { bits }
    }
    #[doc = "Bit 5 - Interrupt for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_3_inten_intcmpbd(&self) -> PWM_3_INTEN_INTCMPBDR {
        let bits = ((self.bits >> 5) & 1) != 0;
        PWM_3_INTEN_INTCMPBDR { bits }
    }
    #[doc = "Bit 8 - Trigger for Counter=0"]
    #[inline(always)]
    pub fn pwm_3_inten_trcntzero(&self) -> PWM_3_INTEN_TRCNTZEROR {
        let bits = ((self.bits >> 8) & 1) != 0;
        PWM_3_INTEN_TRCNTZEROR { bits }
    }
    #[doc = "Bit 9 - Trigger for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_3_inten_trcntload(&self) -> PWM_3_INTEN_TRCNTLOADR {
        let bits = ((self.bits >> 9) & 1) != 0;
        PWM_3_INTEN_TRCNTLOADR { bits }
    }
    #[doc = "Bit 10 - Trigger for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_3_inten_trcmpau(&self) -> PWM_3_INTEN_TRCMPAUR {
        let bits = ((self.bits >> 10) & 1) != 0;
        PWM_3_INTEN_TRCMPAUR { bits }
    }
    #[doc = "Bit 11 - Trigger for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_3_inten_trcmpad(&self) -> PWM_3_INTEN_TRCMPADR {
        let bits = ((self.bits >> 11) & 1) != 0;
        PWM_3_INTEN_TRCMPADR { bits }
    }
    #[doc = "Bit 12 - Trigger for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_3_inten_trcmpbu(&self) -> PWM_3_INTEN_TRCMPBUR {
        let bits = ((self.bits >> 12) & 1) != 0;
        PWM_3_INTEN_TRCMPBUR { bits }
    }
    #[doc = "Bit 13 - Trigger for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_3_inten_trcmpbd(&self) -> PWM_3_INTEN_TRCMPBDR {
        let bits = ((self.bits >> 13) & 1) != 0;
        PWM_3_INTEN_TRCMPBDR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt for Counter=0"]
    #[inline(always)]
    pub fn pwm_3_inten_intcntzero(&mut self) -> _PWM_3_INTEN_INTCNTZEROW {
        _PWM_3_INTEN_INTCNTZEROW { w: self }
    }
    #[doc = "Bit 1 - Interrupt for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_3_inten_intcntload(&mut self) -> _PWM_3_INTEN_INTCNTLOADW {
        _PWM_3_INTEN_INTCNTLOADW { w: self }
    }
    #[doc = "Bit 2 - Interrupt for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_3_inten_intcmpau(&mut self) -> _PWM_3_INTEN_INTCMPAUW {
        _PWM_3_INTEN_INTCMPAUW { w: self }
    }
    #[doc = "Bit 3 - Interrupt for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_3_inten_intcmpad(&mut self) -> _PWM_3_INTEN_INTCMPADW {
        _PWM_3_INTEN_INTCMPADW { w: self }
    }
    #[doc = "Bit 4 - Interrupt for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_3_inten_intcmpbu(&mut self) -> _PWM_3_INTEN_INTCMPBUW {
        _PWM_3_INTEN_INTCMPBUW { w: self }
    }
    #[doc = "Bit 5 - Interrupt for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_3_inten_intcmpbd(&mut self) -> _PWM_3_INTEN_INTCMPBDW {
        _PWM_3_INTEN_INTCMPBDW { w: self }
    }
    #[doc = "Bit 8 - Trigger for Counter=0"]
    #[inline(always)]
    pub fn pwm_3_inten_trcntzero(&mut self) -> _PWM_3_INTEN_TRCNTZEROW {
        _PWM_3_INTEN_TRCNTZEROW { w: self }
    }
    #[doc = "Bit 9 - Trigger for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_3_inten_trcntload(&mut self) -> _PWM_3_INTEN_TRCNTLOADW {
        _PWM_3_INTEN_TRCNTLOADW { w: self }
    }
    #[doc = "Bit 10 - Trigger for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_3_inten_trcmpau(&mut self) -> _PWM_3_INTEN_TRCMPAUW {
        _PWM_3_INTEN_TRCMPAUW { w: self }
    }
    #[doc = "Bit 11 - Trigger for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_3_inten_trcmpad(&mut self) -> _PWM_3_INTEN_TRCMPADW {
        _PWM_3_INTEN_TRCMPADW { w: self }
    }
    #[doc = "Bit 12 - Trigger for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_3_inten_trcmpbu(&mut self) -> _PWM_3_INTEN_TRCMPBUW {
        _PWM_3_INTEN_TRCMPBUW { w: self }
    }
    #[doc = "Bit 13 - Trigger for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_3_inten_trcmpbd(&mut self) -> _PWM_3_INTEN_TRCMPBDW {
        _PWM_3_INTEN_TRCMPBDW { w: self }
    }
}
