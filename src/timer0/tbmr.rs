#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TBMR {
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
#[doc = "Possible values of the field `TIMER_TBMR_TBMR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_TBMR_TBMRR {
    #[doc = "One-Shot Timer mode"]
    TIMER_TBMR_TBMR_1_SHOT,
    #[doc = "Periodic Timer mode"]
    TIMER_TBMR_TBMR_PERIOD,
    #[doc = "Capture mode"]
    TIMER_TBMR_TBMR_CAP,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl TIMER_TBMR_TBMRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_TBMR_TBMRR::TIMER_TBMR_TBMR_1_SHOT => 1,
            TIMER_TBMR_TBMRR::TIMER_TBMR_TBMR_PERIOD => 2,
            TIMER_TBMR_TBMRR::TIMER_TBMR_TBMR_CAP => 3,
            TIMER_TBMR_TBMRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_TBMR_TBMRR {
        match value {
            1 => TIMER_TBMR_TBMRR::TIMER_TBMR_TBMR_1_SHOT,
            2 => TIMER_TBMR_TBMRR::TIMER_TBMR_TBMR_PERIOD,
            3 => TIMER_TBMR_TBMRR::TIMER_TBMR_TBMR_CAP,
            i => TIMER_TBMR_TBMRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TBMR_1_SHOT`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_1_shot(&self) -> bool {
        *self == TIMER_TBMR_TBMRR::TIMER_TBMR_TBMR_1_SHOT
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TBMR_PERIOD`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_period(&self) -> bool {
        *self == TIMER_TBMR_TBMRR::TIMER_TBMR_TBMR_PERIOD
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TBMR_CAP`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_cap(&self) -> bool {
        *self == TIMER_TBMR_TBMRR::TIMER_TBMR_TBMR_CAP
    }
}
#[doc = "Values that can be written to the field `TIMER_TBMR_TBMR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_TBMR_TBMRW {
    #[doc = "One-Shot Timer mode"]
    TIMER_TBMR_TBMR_1_SHOT,
    #[doc = "Periodic Timer mode"]
    TIMER_TBMR_TBMR_PERIOD,
    #[doc = "Capture mode"]
    TIMER_TBMR_TBMR_CAP,
}
impl TIMER_TBMR_TBMRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_TBMR_TBMRW::TIMER_TBMR_TBMR_1_SHOT => 1,
            TIMER_TBMR_TBMRW::TIMER_TBMR_TBMR_PERIOD => 2,
            TIMER_TBMR_TBMRW::TIMER_TBMR_TBMR_CAP => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_TBMR_TBMRW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBMRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_TBMR_TBMRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_1_shot(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TBMRW::TIMER_TBMR_TBMR_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_period(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TBMRW::TIMER_TBMR_TBMR_PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_cap(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TBMRW::TIMER_TBMR_TBMR_CAP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct TIMER_TBMR_TBCMRR {
    bits: bool,
}
impl TIMER_TBMR_TBCMRR {
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
pub struct _TIMER_TBMR_TBCMRW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBCMRW<'a> {
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
pub struct TIMER_TBMR_TBAMSR {
    bits: bool,
}
impl TIMER_TBMR_TBAMSR {
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
pub struct _TIMER_TBMR_TBAMSW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBAMSW<'a> {
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
pub struct TIMER_TBMR_TBCDIRR {
    bits: bool,
}
impl TIMER_TBMR_TBCDIRR {
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
pub struct _TIMER_TBMR_TBCDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBCDIRW<'a> {
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
pub struct TIMER_TBMR_TBMIER {
    bits: bool,
}
impl TIMER_TBMR_TBMIER {
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
pub struct _TIMER_TBMR_TBMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBMIEW<'a> {
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
pub struct TIMER_TBMR_TBWOTR {
    bits: bool,
}
impl TIMER_TBMR_TBWOTR {
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
pub struct _TIMER_TBMR_TBWOTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBWOTW<'a> {
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
pub struct TIMER_TBMR_TBSNAPSR {
    bits: bool,
}
impl TIMER_TBMR_TBSNAPSR {
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
pub struct _TIMER_TBMR_TBSNAPSW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBSNAPSW<'a> {
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
#[doc = r"Value of the field"]
pub struct TIMER_TBMR_TBILDR {
    bits: bool,
}
impl TIMER_TBMR_TBILDR {
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
pub struct _TIMER_TBMR_TBILDW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBILDW<'a> {
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
pub struct TIMER_TBMR_TBPWMIER {
    bits: bool,
}
impl TIMER_TBMR_TBPWMIER {
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
pub struct _TIMER_TBMR_TBPWMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBPWMIEW<'a> {
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
pub struct TIMER_TBMR_TBMRSUR {
    bits: bool,
}
impl TIMER_TBMR_TBMRSUR {
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
pub struct _TIMER_TBMR_TBMRSUW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBMRSUW<'a> {
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
pub struct TIMER_TBMR_TBPLOR {
    bits: bool,
}
impl TIMER_TBMR_TBPLOR {
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
pub struct _TIMER_TBMR_TBPLOW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBPLOW<'a> {
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
pub struct TIMER_TBMR_TBCINTDR {
    bits: bool,
}
impl TIMER_TBMR_TBCINTDR {
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
pub struct _TIMER_TBMR_TBCINTDW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TBCINTDW<'a> {
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
#[doc = "Possible values of the field `TIMER_TBMR_TCACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_TBMR_TCACTR {
    #[doc = "Disable compare operations"]
    TIMER_TBMR_TCACT_NONE,
    #[doc = "Toggle State on Time-Out"]
    TIMER_TBMR_TCACT_TOGGLE,
    #[doc = "Clear CCP on Time-Out"]
    TIMER_TBMR_TCACT_CLRTO,
    #[doc = "Set CCP on Time-Out"]
    TIMER_TBMR_TCACT_SETTO,
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    TIMER_TBMR_TCACT_SETTOGTO,
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    TIMER_TBMR_TCACT_CLRTOGTO,
    #[doc = "Set CCP immediately and clear on Time-Out"]
    TIMER_TBMR_TCACT_SETCLRTO,
    #[doc = "Clear CCP immediately and set on Time-Out"]
    TIMER_TBMR_TCACT_CLRSETTO,
}
impl TIMER_TBMR_TCACTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_NONE => 0,
            TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_TOGGLE => 1,
            TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_CLRTO => 2,
            TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_SETTO => 3,
            TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_SETTOGTO => 4,
            TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_CLRTOGTO => 5,
            TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_SETCLRTO => 6,
            TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_CLRSETTO => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_TBMR_TCACTR {
        match value {
            0 => TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_NONE,
            1 => TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_TOGGLE,
            2 => TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_CLRTO,
            3 => TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_SETTO,
            4 => TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_SETTOGTO,
            5 => TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_CLRTOGTO,
            6 => TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_SETCLRTO,
            7 => TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_CLRSETTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_NONE`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_none(&self) -> bool {
        *self == TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_TOGGLE`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_toggle(&self) -> bool {
        *self == TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_TOGGLE
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_CLRTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_clrto(&self) -> bool {
        *self == TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_CLRTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_SETTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_setto(&self) -> bool {
        *self == TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_SETTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_SETTOGTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_settogto(&self) -> bool {
        *self == TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_SETTOGTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_CLRTOGTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_clrtogto(&self) -> bool {
        *self == TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_CLRTOGTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_SETCLRTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_setclrto(&self) -> bool {
        *self == TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_SETCLRTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_CLRSETTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_clrsetto(&self) -> bool {
        *self == TIMER_TBMR_TCACTR::TIMER_TBMR_TCACT_CLRSETTO
    }
}
#[doc = "Values that can be written to the field `TIMER_TBMR_TCACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_TBMR_TCACTW {
    #[doc = "Disable compare operations"]
    TIMER_TBMR_TCACT_NONE,
    #[doc = "Toggle State on Time-Out"]
    TIMER_TBMR_TCACT_TOGGLE,
    #[doc = "Clear CCP on Time-Out"]
    TIMER_TBMR_TCACT_CLRTO,
    #[doc = "Set CCP on Time-Out"]
    TIMER_TBMR_TCACT_SETTO,
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    TIMER_TBMR_TCACT_SETTOGTO,
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    TIMER_TBMR_TCACT_CLRTOGTO,
    #[doc = "Set CCP immediately and clear on Time-Out"]
    TIMER_TBMR_TCACT_SETCLRTO,
    #[doc = "Clear CCP immediately and set on Time-Out"]
    TIMER_TBMR_TCACT_CLRSETTO,
}
impl TIMER_TBMR_TCACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_NONE => 0,
            TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_TOGGLE => 1,
            TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_CLRTO => 2,
            TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_SETTO => 3,
            TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_SETTOGTO => 4,
            TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_CLRTOGTO => 5,
            TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_SETCLRTO => 6,
            TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_CLRSETTO => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_TBMR_TCACTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TBMR_TCACTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_TBMR_TCACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable compare operations"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_none(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_NONE)
    }
    #[doc = "Toggle State on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_toggle(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_TOGGLE)
    }
    #[doc = "Clear CCP on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_clrto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_CLRTO)
    }
    #[doc = "Set CCP on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_setto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_SETTO)
    }
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_settogto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_SETTOGTO)
    }
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_clrtogto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_CLRTOGTO)
    }
    #[doc = "Set CCP immediately and clear on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_setclrto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_SETCLRTO)
    }
    #[doc = "Clear CCP immediately and set on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_clrsetto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACTW::TIMER_TBMR_TCACT_CLRSETTO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 13);
        self.w.bits |= ((value as u32) & 7) << 13;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr(&self) -> TIMER_TBMR_TBMRR {
        TIMER_TBMR_TBMRR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbcmr(&self) -> TIMER_TBMR_TBCMRR {
        let bits = ((self.bits >> 2) & 1) != 0;
        TIMER_TBMR_TBCMRR { bits }
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tbmr_tbams(&self) -> TIMER_TBMR_TBAMSR {
        let bits = ((self.bits >> 3) & 1) != 0;
        TIMER_TBMR_TBAMSR { bits }
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline(always)]
    pub fn timer_tbmr_tbcdir(&self) -> TIMER_TBMR_TBCDIRR {
        let bits = ((self.bits >> 4) & 1) != 0;
        TIMER_TBMR_TBCDIRR { bits }
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tbmr_tbmie(&self) -> TIMER_TBMR_TBMIER {
        let bits = ((self.bits >> 5) & 1) != 0;
        TIMER_TBMR_TBMIER { bits }
    }
    #[doc = "Bit 6 - GPTM Timer B Wait-on-Trigger"]
    #[inline(always)]
    pub fn timer_tbmr_tbwot(&self) -> TIMER_TBMR_TBWOTR {
        let bits = ((self.bits >> 6) & 1) != 0;
        TIMER_TBMR_TBWOTR { bits }
    }
    #[doc = "Bit 7 - GPTM Timer B Snap-Shot Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbsnaps(&self) -> TIMER_TBMR_TBSNAPSR {
        let bits = ((self.bits >> 7) & 1) != 0;
        TIMER_TBMR_TBSNAPSR { bits }
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline(always)]
    pub fn timer_tbmr_tbild(&self) -> TIMER_TBMR_TBILDR {
        let bits = ((self.bits >> 8) & 1) != 0;
        TIMER_TBMR_TBILDR { bits }
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tbmr_tbpwmie(&self) -> TIMER_TBMR_TBPWMIER {
        let bits = ((self.bits >> 9) & 1) != 0;
        TIMER_TBMR_TBPWMIER { bits }
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline(always)]
    pub fn timer_tbmr_tbmrsu(&self) -> TIMER_TBMR_TBMRSUR {
        let bits = ((self.bits >> 10) & 1) != 0;
        TIMER_TBMR_TBMRSUR { bits }
    }
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy Operation"]
    #[inline(always)]
    pub fn timer_tbmr_tbplo(&self) -> TIMER_TBMR_TBPLOR {
        let bits = ((self.bits >> 11) & 1) != 0;
        TIMER_TBMR_TBPLOR { bits }
    }
    #[doc = "Bit 12 - One-Shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn timer_tbmr_tbcintd(&self) -> TIMER_TBMR_TBCINTDR {
        let bits = ((self.bits >> 12) & 1) != 0;
        TIMER_TBMR_TBCINTDR { bits }
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn timer_tbmr_tcact(&self) -> TIMER_TBMR_TCACTR {
        TIMER_TBMR_TCACTR::_from(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr(&mut self) -> _TIMER_TBMR_TBMRW {
        _TIMER_TBMR_TBMRW { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbcmr(&mut self) -> _TIMER_TBMR_TBCMRW {
        _TIMER_TBMR_TBCMRW { w: self }
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tbmr_tbams(&mut self) -> _TIMER_TBMR_TBAMSW {
        _TIMER_TBMR_TBAMSW { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline(always)]
    pub fn timer_tbmr_tbcdir(&mut self) -> _TIMER_TBMR_TBCDIRW {
        _TIMER_TBMR_TBCDIRW { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tbmr_tbmie(&mut self) -> _TIMER_TBMR_TBMIEW {
        _TIMER_TBMR_TBMIEW { w: self }
    }
    #[doc = "Bit 6 - GPTM Timer B Wait-on-Trigger"]
    #[inline(always)]
    pub fn timer_tbmr_tbwot(&mut self) -> _TIMER_TBMR_TBWOTW {
        _TIMER_TBMR_TBWOTW { w: self }
    }
    #[doc = "Bit 7 - GPTM Timer B Snap-Shot Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbsnaps(&mut self) -> _TIMER_TBMR_TBSNAPSW {
        _TIMER_TBMR_TBSNAPSW { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline(always)]
    pub fn timer_tbmr_tbild(&mut self) -> _TIMER_TBMR_TBILDW {
        _TIMER_TBMR_TBILDW { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tbmr_tbpwmie(&mut self) -> _TIMER_TBMR_TBPWMIEW {
        _TIMER_TBMR_TBPWMIEW { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline(always)]
    pub fn timer_tbmr_tbmrsu(&mut self) -> _TIMER_TBMR_TBMRSUW {
        _TIMER_TBMR_TBMRSUW { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy Operation"]
    #[inline(always)]
    pub fn timer_tbmr_tbplo(&mut self) -> _TIMER_TBMR_TBPLOW {
        _TIMER_TBMR_TBPLOW { w: self }
    }
    #[doc = "Bit 12 - One-Shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn timer_tbmr_tbcintd(&mut self) -> _TIMER_TBMR_TBCINTDW {
        _TIMER_TBMR_TBCINTDW { w: self }
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn timer_tbmr_tcact(&mut self) -> _TIMER_TBMR_TCACTW {
        _TIMER_TBMR_TCACTW { w: self }
    }
}
