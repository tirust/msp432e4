#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TAMR {
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
#[doc = "Possible values of the field `TIMER_TAMR_TAMR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_TAMR_TAMRR {
    #[doc = "One-Shot Timer mode"]
    TIMER_TAMR_TAMR_1_SHOT,
    #[doc = "Periodic Timer mode"]
    TIMER_TAMR_TAMR_PERIOD,
    #[doc = "Capture mode"]
    TIMER_TAMR_TAMR_CAP,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl TIMER_TAMR_TAMRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_TAMR_TAMRR::TIMER_TAMR_TAMR_1_SHOT => 1,
            TIMER_TAMR_TAMRR::TIMER_TAMR_TAMR_PERIOD => 2,
            TIMER_TAMR_TAMRR::TIMER_TAMR_TAMR_CAP => 3,
            TIMER_TAMR_TAMRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_TAMR_TAMRR {
        match value {
            1 => TIMER_TAMR_TAMRR::TIMER_TAMR_TAMR_1_SHOT,
            2 => TIMER_TAMR_TAMRR::TIMER_TAMR_TAMR_PERIOD,
            3 => TIMER_TAMR_TAMRR::TIMER_TAMR_TAMR_CAP,
            i => TIMER_TAMR_TAMRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TAMR_1_SHOT`"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_1_shot(&self) -> bool {
        *self == TIMER_TAMR_TAMRR::TIMER_TAMR_TAMR_1_SHOT
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TAMR_PERIOD`"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_period(&self) -> bool {
        *self == TIMER_TAMR_TAMRR::TIMER_TAMR_TAMR_PERIOD
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TAMR_CAP`"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_cap(&self) -> bool {
        *self == TIMER_TAMR_TAMRR::TIMER_TAMR_TAMR_CAP
    }
}
#[doc = "Values that can be written to the field `TIMER_TAMR_TAMR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_TAMR_TAMRW {
    #[doc = "One-Shot Timer mode"]
    TIMER_TAMR_TAMR_1_SHOT,
    #[doc = "Periodic Timer mode"]
    TIMER_TAMR_TAMR_PERIOD,
    #[doc = "Capture mode"]
    TIMER_TAMR_TAMR_CAP,
}
impl TIMER_TAMR_TAMRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_TAMR_TAMRW::TIMER_TAMR_TAMR_1_SHOT => 1,
            TIMER_TAMR_TAMRW::TIMER_TAMR_TAMR_PERIOD => 2,
            TIMER_TAMR_TAMRW::TIMER_TAMR_TAMR_CAP => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_TAMR_TAMRW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TAMRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_TAMR_TAMRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_1_shot(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TAMRW::TIMER_TAMR_TAMR_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_period(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TAMRW::TIMER_TAMR_TAMR_PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_cap(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TAMRW::TIMER_TAMR_TAMR_CAP)
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
pub struct TIMER_TAMR_TACMRR {
    bits: bool,
}
impl TIMER_TAMR_TACMRR {
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
pub struct _TIMER_TAMR_TACMRW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TACMRW<'a> {
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
pub struct TIMER_TAMR_TAAMSR {
    bits: bool,
}
impl TIMER_TAMR_TAAMSR {
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
pub struct _TIMER_TAMR_TAAMSW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TAAMSW<'a> {
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
pub struct TIMER_TAMR_TACDIRR {
    bits: bool,
}
impl TIMER_TAMR_TACDIRR {
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
pub struct _TIMER_TAMR_TACDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TACDIRW<'a> {
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
pub struct TIMER_TAMR_TAMIER {
    bits: bool,
}
impl TIMER_TAMR_TAMIER {
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
pub struct _TIMER_TAMR_TAMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TAMIEW<'a> {
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
pub struct TIMER_TAMR_TAWOTR {
    bits: bool,
}
impl TIMER_TAMR_TAWOTR {
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
pub struct _TIMER_TAMR_TAWOTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TAWOTW<'a> {
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
pub struct TIMER_TAMR_TASNAPSR {
    bits: bool,
}
impl TIMER_TAMR_TASNAPSR {
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
pub struct _TIMER_TAMR_TASNAPSW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TASNAPSW<'a> {
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
pub struct TIMER_TAMR_TAILDR {
    bits: bool,
}
impl TIMER_TAMR_TAILDR {
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
pub struct _TIMER_TAMR_TAILDW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TAILDW<'a> {
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
pub struct TIMER_TAMR_TAPWMIER {
    bits: bool,
}
impl TIMER_TAMR_TAPWMIER {
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
pub struct _TIMER_TAMR_TAPWMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TAPWMIEW<'a> {
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
pub struct TIMER_TAMR_TAMRSUR {
    bits: bool,
}
impl TIMER_TAMR_TAMRSUR {
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
pub struct _TIMER_TAMR_TAMRSUW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TAMRSUW<'a> {
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
pub struct TIMER_TAMR_TAPLOR {
    bits: bool,
}
impl TIMER_TAMR_TAPLOR {
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
pub struct _TIMER_TAMR_TAPLOW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TAPLOW<'a> {
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
pub struct TIMER_TAMR_TACINTDR {
    bits: bool,
}
impl TIMER_TAMR_TACINTDR {
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
pub struct _TIMER_TAMR_TACINTDW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TACINTDW<'a> {
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
#[doc = "Possible values of the field `TIMER_TAMR_TCACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_TAMR_TCACTR {
    #[doc = "Disable compare operations"]
    TIMER_TAMR_TCACT_NONE,
    #[doc = "Toggle State on Time-Out"]
    TIMER_TAMR_TCACT_TOGGLE,
    #[doc = "Clear CCP on Time-Out"]
    TIMER_TAMR_TCACT_CLRTO,
    #[doc = "Set CCP on Time-Out"]
    TIMER_TAMR_TCACT_SETTO,
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    TIMER_TAMR_TCACT_SETTOGTO,
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    TIMER_TAMR_TCACT_CLRTOGTO,
    #[doc = "Set CCP immediately and clear on Time-Out"]
    TIMER_TAMR_TCACT_SETCLRTO,
    #[doc = "Clear CCP immediately and set on Time-Out"]
    TIMER_TAMR_TCACT_CLRSETTO,
}
impl TIMER_TAMR_TCACTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_NONE => 0,
            TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_TOGGLE => 1,
            TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_CLRTO => 2,
            TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_SETTO => 3,
            TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_SETTOGTO => 4,
            TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_CLRTOGTO => 5,
            TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_SETCLRTO => 6,
            TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_CLRSETTO => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> TIMER_TAMR_TCACTR {
        match value {
            0 => TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_NONE,
            1 => TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_TOGGLE,
            2 => TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_CLRTO,
            3 => TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_SETTO,
            4 => TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_SETTOGTO,
            5 => TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_CLRTOGTO,
            6 => TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_SETCLRTO,
            7 => TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_CLRSETTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_NONE`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_none(&self) -> bool {
        *self == TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_TOGGLE`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_toggle(&self) -> bool {
        *self == TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_TOGGLE
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_CLRTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_clrto(&self) -> bool {
        *self == TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_CLRTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_SETTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_setto(&self) -> bool {
        *self == TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_SETTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_SETTOGTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_settogto(&self) -> bool {
        *self == TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_SETTOGTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_CLRTOGTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_clrtogto(&self) -> bool {
        *self == TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_CLRTOGTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_SETCLRTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_setclrto(&self) -> bool {
        *self == TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_SETCLRTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_CLRSETTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_clrsetto(&self) -> bool {
        *self == TIMER_TAMR_TCACTR::TIMER_TAMR_TCACT_CLRSETTO
    }
}
#[doc = "Values that can be written to the field `TIMER_TAMR_TCACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_TAMR_TCACTW {
    #[doc = "Disable compare operations"]
    TIMER_TAMR_TCACT_NONE,
    #[doc = "Toggle State on Time-Out"]
    TIMER_TAMR_TCACT_TOGGLE,
    #[doc = "Clear CCP on Time-Out"]
    TIMER_TAMR_TCACT_CLRTO,
    #[doc = "Set CCP on Time-Out"]
    TIMER_TAMR_TCACT_SETTO,
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    TIMER_TAMR_TCACT_SETTOGTO,
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    TIMER_TAMR_TCACT_CLRTOGTO,
    #[doc = "Set CCP immediately and clear on Time-Out"]
    TIMER_TAMR_TCACT_SETCLRTO,
    #[doc = "Clear CCP immediately and set on Time-Out"]
    TIMER_TAMR_TCACT_CLRSETTO,
}
impl TIMER_TAMR_TCACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_NONE => 0,
            TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_TOGGLE => 1,
            TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_CLRTO => 2,
            TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_SETTO => 3,
            TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_SETTOGTO => 4,
            TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_CLRTOGTO => 5,
            TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_SETCLRTO => 6,
            TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_CLRSETTO => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMER_TAMR_TCACTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_TAMR_TCACTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_TAMR_TCACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable compare operations"]
    #[inline(always)]
    pub fn timer_tamr_tcact_none(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_NONE)
    }
    #[doc = "Toggle State on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_toggle(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_TOGGLE)
    }
    #[doc = "Clear CCP on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_clrto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_CLRTO)
    }
    #[doc = "Set CCP on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_setto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_SETTO)
    }
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_settogto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_SETTOGTO)
    }
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_clrtogto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_CLRTOGTO)
    }
    #[doc = "Set CCP immediately and clear on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_setclrto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_SETCLRTO)
    }
    #[doc = "Clear CCP immediately and set on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_clrsetto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACTW::TIMER_TAMR_TCACT_CLRSETTO)
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
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr(&self) -> TIMER_TAMR_TAMRR {
        TIMER_TAMR_TAMRR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline(always)]
    pub fn timer_tamr_tacmr(&self) -> TIMER_TAMR_TACMRR {
        let bits = ((self.bits >> 2) & 1) != 0;
        TIMER_TAMR_TACMRR { bits }
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tamr_taams(&self) -> TIMER_TAMR_TAAMSR {
        let bits = ((self.bits >> 3) & 1) != 0;
        TIMER_TAMR_TAAMSR { bits }
    }
    #[doc = "Bit 4 - GPTM Timer A Count Direction"]
    #[inline(always)]
    pub fn timer_tamr_tacdir(&self) -> TIMER_TAMR_TACDIRR {
        let bits = ((self.bits >> 4) & 1) != 0;
        TIMER_TAMR_TACDIRR { bits }
    }
    #[doc = "Bit 5 - GPTM Timer A Match Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tamr_tamie(&self) -> TIMER_TAMR_TAMIER {
        let bits = ((self.bits >> 5) & 1) != 0;
        TIMER_TAMR_TAMIER { bits }
    }
    #[doc = "Bit 6 - GPTM Timer A Wait-on-Trigger"]
    #[inline(always)]
    pub fn timer_tamr_tawot(&self) -> TIMER_TAMR_TAWOTR {
        let bits = ((self.bits >> 6) & 1) != 0;
        TIMER_TAMR_TAWOTR { bits }
    }
    #[doc = "Bit 7 - GPTM Timer A Snap-Shot Mode"]
    #[inline(always)]
    pub fn timer_tamr_tasnaps(&self) -> TIMER_TAMR_TASNAPSR {
        let bits = ((self.bits >> 7) & 1) != 0;
        TIMER_TAMR_TASNAPSR { bits }
    }
    #[doc = "Bit 8 - GPTM Timer A Interval Load Write"]
    #[inline(always)]
    pub fn timer_tamr_taild(&self) -> TIMER_TAMR_TAILDR {
        let bits = ((self.bits >> 8) & 1) != 0;
        TIMER_TAMR_TAILDR { bits }
    }
    #[doc = "Bit 9 - GPTM Timer A PWM Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tamr_tapwmie(&self) -> TIMER_TAMR_TAPWMIER {
        let bits = ((self.bits >> 9) & 1) != 0;
        TIMER_TAMR_TAPWMIER { bits }
    }
    #[doc = "Bit 10 - GPTM Timer A Match Register Update"]
    #[inline(always)]
    pub fn timer_tamr_tamrsu(&self) -> TIMER_TAMR_TAMRSUR {
        let bits = ((self.bits >> 10) & 1) != 0;
        TIMER_TAMR_TAMRSUR { bits }
    }
    #[doc = "Bit 11 - GPTM Timer A PWM Legacy Operation"]
    #[inline(always)]
    pub fn timer_tamr_taplo(&self) -> TIMER_TAMR_TAPLOR {
        let bits = ((self.bits >> 11) & 1) != 0;
        TIMER_TAMR_TAPLOR { bits }
    }
    #[doc = "Bit 12 - One-shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn timer_tamr_tacintd(&self) -> TIMER_TAMR_TACINTDR {
        let bits = ((self.bits >> 12) & 1) != 0;
        TIMER_TAMR_TACINTDR { bits }
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn timer_tamr_tcact(&self) -> TIMER_TAMR_TCACTR {
        TIMER_TAMR_TCACTR::_from(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr(&mut self) -> _TIMER_TAMR_TAMRW {
        _TIMER_TAMR_TAMRW { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline(always)]
    pub fn timer_tamr_tacmr(&mut self) -> _TIMER_TAMR_TACMRW {
        _TIMER_TAMR_TACMRW { w: self }
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tamr_taams(&mut self) -> _TIMER_TAMR_TAAMSW {
        _TIMER_TAMR_TAAMSW { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer A Count Direction"]
    #[inline(always)]
    pub fn timer_tamr_tacdir(&mut self) -> _TIMER_TAMR_TACDIRW {
        _TIMER_TAMR_TACDIRW { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A Match Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tamr_tamie(&mut self) -> _TIMER_TAMR_TAMIEW {
        _TIMER_TAMR_TAMIEW { w: self }
    }
    #[doc = "Bit 6 - GPTM Timer A Wait-on-Trigger"]
    #[inline(always)]
    pub fn timer_tamr_tawot(&mut self) -> _TIMER_TAMR_TAWOTW {
        _TIMER_TAMR_TAWOTW { w: self }
    }
    #[doc = "Bit 7 - GPTM Timer A Snap-Shot Mode"]
    #[inline(always)]
    pub fn timer_tamr_tasnaps(&mut self) -> _TIMER_TAMR_TASNAPSW {
        _TIMER_TAMR_TASNAPSW { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer A Interval Load Write"]
    #[inline(always)]
    pub fn timer_tamr_taild(&mut self) -> _TIMER_TAMR_TAILDW {
        _TIMER_TAMR_TAILDW { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer A PWM Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tamr_tapwmie(&mut self) -> _TIMER_TAMR_TAPWMIEW {
        _TIMER_TAMR_TAPWMIEW { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer A Match Register Update"]
    #[inline(always)]
    pub fn timer_tamr_tamrsu(&mut self) -> _TIMER_TAMR_TAMRSUW {
        _TIMER_TAMR_TAMRSUW { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer A PWM Legacy Operation"]
    #[inline(always)]
    pub fn timer_tamr_taplo(&mut self) -> _TIMER_TAMR_TAPLOW {
        _TIMER_TAMR_TAPLOW { w: self }
    }
    #[doc = "Bit 12 - One-shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn timer_tamr_tacintd(&mut self) -> _TIMER_TAMR_TACINTDW {
        _TIMER_TAMR_TACINTDW { w: self }
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn timer_tamr_tcact(&mut self) -> _TIMER_TAMR_TCACTW {
        _TIMER_TAMR_TCACTW { w: self }
    }
}
