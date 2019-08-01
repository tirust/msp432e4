#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
pub struct TIMER_CTL_TAENR {
    bits: bool,
}
impl TIMER_CTL_TAENR {
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
pub struct _TIMER_CTL_TAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CTL_TAENW<'a> {
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
pub struct TIMER_CTL_TASTALLR {
    bits: bool,
}
impl TIMER_CTL_TASTALLR {
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
pub struct _TIMER_CTL_TASTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CTL_TASTALLW<'a> {
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
#[doc = "Possible values of the field `TIMER_CTL_TAEVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_CTL_TAEVENTR {
    #[doc = "Positive edge"]
    TIMER_CTL_TAEVENT_POS,
    #[doc = "Negative edge"]
    TIMER_CTL_TAEVENT_NEG,
    #[doc = "Both edges"]
    TIMER_CTL_TAEVENT_BOTH,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl TIMER_CTL_TAEVENTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_CTL_TAEVENTR::TIMER_CTL_TAEVENT_POS => 0,
            TIMER_CTL_TAEVENTR::TIMER_CTL_TAEVENT_NEG => 1,
            TIMER_CTL_TAEVENTR::TIMER_CTL_TAEVENT_BOTH => 3,
            TIMER_CTL_TAEVENTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_CTL_TAEVENTR {
        match value {
            0 => TIMER_CTL_TAEVENTR::TIMER_CTL_TAEVENT_POS,
            1 => TIMER_CTL_TAEVENTR::TIMER_CTL_TAEVENT_NEG,
            3 => TIMER_CTL_TAEVENTR::TIMER_CTL_TAEVENT_BOTH,
            i => TIMER_CTL_TAEVENTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TAEVENT_POS`"]
    #[inline(always)]
    pub fn is_timer_ctl_taevent_pos(&self) -> bool {
        *self == TIMER_CTL_TAEVENTR::TIMER_CTL_TAEVENT_POS
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TAEVENT_NEG`"]
    #[inline(always)]
    pub fn is_timer_ctl_taevent_neg(&self) -> bool {
        *self == TIMER_CTL_TAEVENTR::TIMER_CTL_TAEVENT_NEG
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TAEVENT_BOTH`"]
    #[inline(always)]
    pub fn is_timer_ctl_taevent_both(&self) -> bool {
        *self == TIMER_CTL_TAEVENTR::TIMER_CTL_TAEVENT_BOTH
    }
}
#[doc = "Values that can be written to the field `TIMER_CTL_TAEVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_CTL_TAEVENTW {
    #[doc = "Positive edge"]
    TIMER_CTL_TAEVENT_POS,
    #[doc = "Negative edge"]
    TIMER_CTL_TAEVENT_NEG,
    #[doc = "Both edges"]
    TIMER_CTL_TAEVENT_BOTH,
}
impl TIMER_CTL_TAEVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_CTL_TAEVENTW::TIMER_CTL_TAEVENT_POS => 0,
            TIMER_CTL_TAEVENTW::TIMER_CTL_TAEVENT_NEG => 1,
            TIMER_CTL_TAEVENTW::TIMER_CTL_TAEVENT_BOTH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_CTL_TAEVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CTL_TAEVENTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_CTL_TAEVENTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn timer_ctl_taevent_pos(self) -> &'a mut W {
        self.variant(TIMER_CTL_TAEVENTW::TIMER_CTL_TAEVENT_POS)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn timer_ctl_taevent_neg(self) -> &'a mut W {
        self.variant(TIMER_CTL_TAEVENTW::TIMER_CTL_TAEVENT_NEG)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn timer_ctl_taevent_both(self) -> &'a mut W {
        self.variant(TIMER_CTL_TAEVENTW::TIMER_CTL_TAEVENT_BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct TIMER_CTL_RTCENR {
    bits: bool,
}
impl TIMER_CTL_RTCENR {
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
pub struct _TIMER_CTL_RTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CTL_RTCENW<'a> {
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
pub struct TIMER_CTL_TAOTER {
    bits: bool,
}
impl TIMER_CTL_TAOTER {
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
pub struct _TIMER_CTL_TAOTEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CTL_TAOTEW<'a> {
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
pub struct TIMER_CTL_TAPWMLR {
    bits: bool,
}
impl TIMER_CTL_TAPWMLR {
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
pub struct _TIMER_CTL_TAPWMLW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CTL_TAPWMLW<'a> {
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
pub struct TIMER_CTL_TBENR {
    bits: bool,
}
impl TIMER_CTL_TBENR {
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
pub struct _TIMER_CTL_TBENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CTL_TBENW<'a> {
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
pub struct TIMER_CTL_TBSTALLR {
    bits: bool,
}
impl TIMER_CTL_TBSTALLR {
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
pub struct _TIMER_CTL_TBSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CTL_TBSTALLW<'a> {
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
#[doc = "Possible values of the field `TIMER_CTL_TBEVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_CTL_TBEVENTR {
    #[doc = "Positive edge"]
    TIMER_CTL_TBEVENT_POS,
    #[doc = "Negative edge"]
    TIMER_CTL_TBEVENT_NEG,
    #[doc = "Both edges"]
    TIMER_CTL_TBEVENT_BOTH,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl TIMER_CTL_TBEVENTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_CTL_TBEVENTR::TIMER_CTL_TBEVENT_POS => 0,
            TIMER_CTL_TBEVENTR::TIMER_CTL_TBEVENT_NEG => 1,
            TIMER_CTL_TBEVENTR::TIMER_CTL_TBEVENT_BOTH => 3,
            TIMER_CTL_TBEVENTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_CTL_TBEVENTR {
        match value {
            0 => TIMER_CTL_TBEVENTR::TIMER_CTL_TBEVENT_POS,
            1 => TIMER_CTL_TBEVENTR::TIMER_CTL_TBEVENT_NEG,
            3 => TIMER_CTL_TBEVENTR::TIMER_CTL_TBEVENT_BOTH,
            i => TIMER_CTL_TBEVENTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TBEVENT_POS`"]
    #[inline(always)]
    pub fn is_timer_ctl_tbevent_pos(&self) -> bool {
        *self == TIMER_CTL_TBEVENTR::TIMER_CTL_TBEVENT_POS
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TBEVENT_NEG`"]
    #[inline(always)]
    pub fn is_timer_ctl_tbevent_neg(&self) -> bool {
        *self == TIMER_CTL_TBEVENTR::TIMER_CTL_TBEVENT_NEG
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TBEVENT_BOTH`"]
    #[inline(always)]
    pub fn is_timer_ctl_tbevent_both(&self) -> bool {
        *self == TIMER_CTL_TBEVENTR::TIMER_CTL_TBEVENT_BOTH
    }
}
#[doc = "Values that can be written to the field `TIMER_CTL_TBEVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_CTL_TBEVENTW {
    #[doc = "Positive edge"]
    TIMER_CTL_TBEVENT_POS,
    #[doc = "Negative edge"]
    TIMER_CTL_TBEVENT_NEG,
    #[doc = "Both edges"]
    TIMER_CTL_TBEVENT_BOTH,
}
impl TIMER_CTL_TBEVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_CTL_TBEVENTW::TIMER_CTL_TBEVENT_POS => 0,
            TIMER_CTL_TBEVENTW::TIMER_CTL_TBEVENT_NEG => 1,
            TIMER_CTL_TBEVENTW::TIMER_CTL_TBEVENT_BOTH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_CTL_TBEVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CTL_TBEVENTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_CTL_TBEVENTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn timer_ctl_tbevent_pos(self) -> &'a mut W {
        self.variant(TIMER_CTL_TBEVENTW::TIMER_CTL_TBEVENT_POS)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn timer_ctl_tbevent_neg(self) -> &'a mut W {
        self.variant(TIMER_CTL_TBEVENTW::TIMER_CTL_TBEVENT_NEG)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn timer_ctl_tbevent_both(self) -> &'a mut W {
        self.variant(TIMER_CTL_TBEVENTW::TIMER_CTL_TBEVENT_BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 10);
        self.w.bits |= ((value as u32) & 3) << 10;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct TIMER_CTL_TBOTER {
    bits: bool,
}
impl TIMER_CTL_TBOTER {
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
pub struct _TIMER_CTL_TBOTEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CTL_TBOTEW<'a> {
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
#[doc = r"Value of the field"]
pub struct TIMER_CTL_TBPWMLR {
    bits: bool,
}
impl TIMER_CTL_TBPWMLR {
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
pub struct _TIMER_CTL_TBPWMLW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_CTL_TBPWMLW<'a> {
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
        self.w.bits &= !(1 << 14);
        self.w.bits |= ((value as u32) & 1) << 14;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - GPTM Timer A Enable"]
    #[inline(always)]
    pub fn timer_ctl_taen(&self) -> TIMER_CTL_TAENR {
        let bits = ((self.bits >> 0) & 1) != 0;
        TIMER_CTL_TAENR { bits }
    }
    #[doc = "Bit 1 - GPTM Timer A Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tastall(&self) -> TIMER_CTL_TASTALLR {
        let bits = ((self.bits >> 1) & 1) != 0;
        TIMER_CTL_TASTALLR { bits }
    }
    #[doc = "Bits 2:3 - GPTM Timer A Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_taevent(&self) -> TIMER_CTL_TAEVENTR {
        TIMER_CTL_TAEVENTR::_from(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - GPTM RTC Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_rtcen(&self) -> TIMER_CTL_RTCENR {
        let bits = ((self.bits >> 4) & 1) != 0;
        TIMER_CTL_RTCENR { bits }
    }
    #[doc = "Bit 5 - GPTM Timer A Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_taote(&self) -> TIMER_CTL_TAOTER {
        let bits = ((self.bits >> 5) & 1) != 0;
        TIMER_CTL_TAOTER { bits }
    }
    #[doc = "Bit 6 - GPTM Timer A PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tapwml(&self) -> TIMER_CTL_TAPWMLR {
        let bits = ((self.bits >> 6) & 1) != 0;
        TIMER_CTL_TAPWMLR { bits }
    }
    #[doc = "Bit 8 - GPTM Timer B Enable"]
    #[inline(always)]
    pub fn timer_ctl_tben(&self) -> TIMER_CTL_TBENR {
        let bits = ((self.bits >> 8) & 1) != 0;
        TIMER_CTL_TBENR { bits }
    }
    #[doc = "Bit 9 - GPTM Timer B Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbstall(&self) -> TIMER_CTL_TBSTALLR {
        let bits = ((self.bits >> 9) & 1) != 0;
        TIMER_CTL_TBSTALLR { bits }
    }
    #[doc = "Bits 10:11 - GPTM Timer B Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_tbevent(&self) -> TIMER_CTL_TBEVENTR {
        TIMER_CTL_TBEVENTR::_from(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - GPTM Timer B Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbote(&self) -> TIMER_CTL_TBOTER {
        let bits = ((self.bits >> 13) & 1) != 0;
        TIMER_CTL_TBOTER { bits }
    }
    #[doc = "Bit 14 - GPTM Timer B PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tbpwml(&self) -> TIMER_CTL_TBPWMLR {
        let bits = ((self.bits >> 14) & 1) != 0;
        TIMER_CTL_TBPWMLR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPTM Timer A Enable"]
    #[inline(always)]
    pub fn timer_ctl_taen(&mut self) -> _TIMER_CTL_TAENW {
        _TIMER_CTL_TAENW { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tastall(&mut self) -> _TIMER_CTL_TASTALLW {
        _TIMER_CTL_TASTALLW { w: self }
    }
    #[doc = "Bits 2:3 - GPTM Timer A Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_taevent(&mut self) -> _TIMER_CTL_TAEVENTW {
        _TIMER_CTL_TAEVENTW { w: self }
    }
    #[doc = "Bit 4 - GPTM RTC Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_rtcen(&mut self) -> _TIMER_CTL_RTCENW {
        _TIMER_CTL_RTCENW { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_taote(&mut self) -> _TIMER_CTL_TAOTEW {
        _TIMER_CTL_TAOTEW { w: self }
    }
    #[doc = "Bit 6 - GPTM Timer A PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tapwml(&mut self) -> _TIMER_CTL_TAPWMLW {
        _TIMER_CTL_TAPWMLW { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Enable"]
    #[inline(always)]
    pub fn timer_ctl_tben(&mut self) -> _TIMER_CTL_TBENW {
        _TIMER_CTL_TBENW { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbstall(&mut self) -> _TIMER_CTL_TBSTALLW {
        _TIMER_CTL_TBSTALLW { w: self }
    }
    #[doc = "Bits 10:11 - GPTM Timer B Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_tbevent(&mut self) -> _TIMER_CTL_TBEVENTW {
        _TIMER_CTL_TBEVENTW { w: self }
    }
    #[doc = "Bit 13 - GPTM Timer B Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbote(&mut self) -> _TIMER_CTL_TBOTEW {
        _TIMER_CTL_TBOTEW { w: self }
    }
    #[doc = "Bit 14 - GPTM Timer B PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tbpwml(&mut self) -> _TIMER_CTL_TBPWMLW {
        _TIMER_CTL_TBPWMLW { w: self }
    }
}
