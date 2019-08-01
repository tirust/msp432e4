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
#[doc = "Possible values of the field `TIMER_PP_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_PP_SIZER {
    #[doc = "Timer A and Timer B counters are 16 bits each with an 8-bit prescale counter"]
    TIMER_PP_SIZE_16,
    #[doc = "Timer A and Timer B counters are 32 bits each with a 16-bit prescale counter"]
    TIMER_PP_SIZE_32,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl TIMER_PP_SIZER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_PP_SIZER::TIMER_PP_SIZE_16 => 0,
            TIMER_PP_SIZER::TIMER_PP_SIZE_32 => 1,
            TIMER_PP_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_PP_SIZER {
        match value {
            0 => TIMER_PP_SIZER::TIMER_PP_SIZE_16,
            1 => TIMER_PP_SIZER::TIMER_PP_SIZE_32,
            i => TIMER_PP_SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_PP_SIZE_16`"]
    #[inline(always)]
    pub fn is_timer_pp_size_16(&self) -> bool {
        *self == TIMER_PP_SIZER::TIMER_PP_SIZE_16
    }
    #[doc = "Checks if the value of the field is `TIMER_PP_SIZE_32`"]
    #[inline(always)]
    pub fn is_timer_pp_size_32(&self) -> bool {
        *self == TIMER_PP_SIZER::TIMER_PP_SIZE_32
    }
}
#[doc = "Values that can be written to the field `TIMER_PP_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_PP_SIZEW {
    #[doc = "Timer A and Timer B counters are 16 bits each with an 8-bit prescale counter"]
    TIMER_PP_SIZE_16,
    #[doc = "Timer A and Timer B counters are 32 bits each with a 16-bit prescale counter"]
    TIMER_PP_SIZE_32,
}
impl TIMER_PP_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_PP_SIZEW::TIMER_PP_SIZE_16 => 0,
            TIMER_PP_SIZEW::TIMER_PP_SIZE_32 => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_PP_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_PP_SIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_PP_SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer A and Timer B counters are 16 bits each with an 8-bit prescale counter"]
    #[inline(always)]
    pub fn timer_pp_size_16(self) -> &'a mut W {
        self.variant(TIMER_PP_SIZEW::TIMER_PP_SIZE_16)
    }
    #[doc = "Timer A and Timer B counters are 32 bits each with a 16-bit prescale counter"]
    #[inline(always)]
    pub fn timer_pp_size_32(self) -> &'a mut W {
        self.variant(TIMER_PP_SIZEW::TIMER_PP_SIZE_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct TIMER_PP_CHAINR {
    bits: bool,
}
impl TIMER_PP_CHAINR {
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
pub struct _TIMER_PP_CHAINW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_PP_CHAINW<'a> {
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
pub struct TIMER_PP_SYNCCNTR {
    bits: bool,
}
impl TIMER_PP_SYNCCNTR {
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
pub struct _TIMER_PP_SYNCCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_PP_SYNCCNTW<'a> {
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
    #[doc = "Bits 0:3 - Count Size"]
    #[inline(always)]
    pub fn timer_pp_size(&self) -> TIMER_PP_SIZER {
        TIMER_PP_SIZER::_from(((self.bits >> 0) & 15) as u8)
    }
    #[doc = "Bit 4 - Chain with Other Timers"]
    #[inline(always)]
    pub fn timer_pp_chain(&self) -> TIMER_PP_CHAINR {
        let bits = ((self.bits >> 4) & 1) != 0;
        TIMER_PP_CHAINR { bits }
    }
    #[doc = "Bit 5 - Synchronize Start"]
    #[inline(always)]
    pub fn timer_pp_synccnt(&self) -> TIMER_PP_SYNCCNTR {
        let bits = ((self.bits >> 5) & 1) != 0;
        TIMER_PP_SYNCCNTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Count Size"]
    #[inline(always)]
    pub fn timer_pp_size(&mut self) -> _TIMER_PP_SIZEW {
        _TIMER_PP_SIZEW { w: self }
    }
    #[doc = "Bit 4 - Chain with Other Timers"]
    #[inline(always)]
    pub fn timer_pp_chain(&mut self) -> _TIMER_PP_CHAINW {
        _TIMER_PP_CHAINW { w: self }
    }
    #[doc = "Bit 5 - Synchronize Start"]
    #[inline(always)]
    pub fn timer_pp_synccnt(&mut self) -> _TIMER_PP_SYNCCNTW {
        _TIMER_PP_SYNCCNTW { w: self }
    }
}
