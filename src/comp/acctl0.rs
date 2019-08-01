#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACCTL0 {
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
pub struct COMP_ACCTL0_CINVR {
    bits: bool,
}
impl COMP_ACCTL0_CINVR {
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
pub struct _COMP_ACCTL0_CINVW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_ACCTL0_CINVW<'a> {
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
#[doc = "Possible values of the field `COMP_ACCTL0_ISEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_ACCTL0_ISENR {
    #[doc = "Level sense, see ISLVAL"]
    COMP_ACCTL0_ISEN_LEVEL,
    #[doc = "Falling edge"]
    COMP_ACCTL0_ISEN_FALL,
    #[doc = "Rising edge"]
    COMP_ACCTL0_ISEN_RISE,
    #[doc = "Either edge"]
    COMP_ACCTL0_ISEN_BOTH,
}
impl COMP_ACCTL0_ISENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_LEVEL => 0,
            COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_FALL => 1,
            COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_RISE => 2,
            COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_BOTH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> COMP_ACCTL0_ISENR {
        match value {
            0 => COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_LEVEL,
            1 => COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_FALL,
            2 => COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_RISE,
            3 => COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL0_ISEN_LEVEL`"]
    #[inline(always)]
    pub fn is_comp_acctl0_isen_level(&self) -> bool {
        *self == COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_LEVEL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL0_ISEN_FALL`"]
    #[inline(always)]
    pub fn is_comp_acctl0_isen_fall(&self) -> bool {
        *self == COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_FALL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL0_ISEN_RISE`"]
    #[inline(always)]
    pub fn is_comp_acctl0_isen_rise(&self) -> bool {
        *self == COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_RISE
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL0_ISEN_BOTH`"]
    #[inline(always)]
    pub fn is_comp_acctl0_isen_both(&self) -> bool {
        *self == COMP_ACCTL0_ISENR::COMP_ACCTL0_ISEN_BOTH
    }
}
#[doc = "Values that can be written to the field `COMP_ACCTL0_ISEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_ACCTL0_ISENW {
    #[doc = "Level sense, see ISLVAL"]
    COMP_ACCTL0_ISEN_LEVEL,
    #[doc = "Falling edge"]
    COMP_ACCTL0_ISEN_FALL,
    #[doc = "Rising edge"]
    COMP_ACCTL0_ISEN_RISE,
    #[doc = "Either edge"]
    COMP_ACCTL0_ISEN_BOTH,
}
impl COMP_ACCTL0_ISENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMP_ACCTL0_ISENW::COMP_ACCTL0_ISEN_LEVEL => 0,
            COMP_ACCTL0_ISENW::COMP_ACCTL0_ISEN_FALL => 1,
            COMP_ACCTL0_ISENW::COMP_ACCTL0_ISEN_RISE => 2,
            COMP_ACCTL0_ISENW::COMP_ACCTL0_ISEN_BOTH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _COMP_ACCTL0_ISENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_ACCTL0_ISENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_ACCTL0_ISENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level sense, see ISLVAL"]
    #[inline(always)]
    pub fn comp_acctl0_isen_level(self) -> &'a mut W {
        self.variant(COMP_ACCTL0_ISENW::COMP_ACCTL0_ISEN_LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn comp_acctl0_isen_fall(self) -> &'a mut W {
        self.variant(COMP_ACCTL0_ISENW::COMP_ACCTL0_ISEN_FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn comp_acctl0_isen_rise(self) -> &'a mut W {
        self.variant(COMP_ACCTL0_ISENW::COMP_ACCTL0_ISEN_RISE)
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn comp_acctl0_isen_both(self) -> &'a mut W {
        self.variant(COMP_ACCTL0_ISENW::COMP_ACCTL0_ISEN_BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct COMP_ACCTL0_ISLVALR {
    bits: bool,
}
impl COMP_ACCTL0_ISLVALR {
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
pub struct _COMP_ACCTL0_ISLVALW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_ACCTL0_ISLVALW<'a> {
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
#[doc = "Possible values of the field `COMP_ACCTL0_TSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_ACCTL0_TSENR {
    #[doc = "Level sense, see TSLVAL"]
    COMP_ACCTL0_TSEN_LEVEL,
    #[doc = "Falling edge"]
    COMP_ACCTL0_TSEN_FALL,
    #[doc = "Rising edge"]
    COMP_ACCTL0_TSEN_RISE,
    #[doc = "Either edge"]
    COMP_ACCTL0_TSEN_BOTH,
}
impl COMP_ACCTL0_TSENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_LEVEL => 0,
            COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_FALL => 1,
            COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_RISE => 2,
            COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_BOTH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> COMP_ACCTL0_TSENR {
        match value {
            0 => COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_LEVEL,
            1 => COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_FALL,
            2 => COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_RISE,
            3 => COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL0_TSEN_LEVEL`"]
    #[inline(always)]
    pub fn is_comp_acctl0_tsen_level(&self) -> bool {
        *self == COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_LEVEL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL0_TSEN_FALL`"]
    #[inline(always)]
    pub fn is_comp_acctl0_tsen_fall(&self) -> bool {
        *self == COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_FALL
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL0_TSEN_RISE`"]
    #[inline(always)]
    pub fn is_comp_acctl0_tsen_rise(&self) -> bool {
        *self == COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_RISE
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL0_TSEN_BOTH`"]
    #[inline(always)]
    pub fn is_comp_acctl0_tsen_both(&self) -> bool {
        *self == COMP_ACCTL0_TSENR::COMP_ACCTL0_TSEN_BOTH
    }
}
#[doc = "Values that can be written to the field `COMP_ACCTL0_TSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_ACCTL0_TSENW {
    #[doc = "Level sense, see TSLVAL"]
    COMP_ACCTL0_TSEN_LEVEL,
    #[doc = "Falling edge"]
    COMP_ACCTL0_TSEN_FALL,
    #[doc = "Rising edge"]
    COMP_ACCTL0_TSEN_RISE,
    #[doc = "Either edge"]
    COMP_ACCTL0_TSEN_BOTH,
}
impl COMP_ACCTL0_TSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMP_ACCTL0_TSENW::COMP_ACCTL0_TSEN_LEVEL => 0,
            COMP_ACCTL0_TSENW::COMP_ACCTL0_TSEN_FALL => 1,
            COMP_ACCTL0_TSENW::COMP_ACCTL0_TSEN_RISE => 2,
            COMP_ACCTL0_TSENW::COMP_ACCTL0_TSEN_BOTH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _COMP_ACCTL0_TSENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_ACCTL0_TSENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_ACCTL0_TSENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level sense, see TSLVAL"]
    #[inline(always)]
    pub fn comp_acctl0_tsen_level(self) -> &'a mut W {
        self.variant(COMP_ACCTL0_TSENW::COMP_ACCTL0_TSEN_LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn comp_acctl0_tsen_fall(self) -> &'a mut W {
        self.variant(COMP_ACCTL0_TSENW::COMP_ACCTL0_TSEN_FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn comp_acctl0_tsen_rise(self) -> &'a mut W {
        self.variant(COMP_ACCTL0_TSENW::COMP_ACCTL0_TSEN_RISE)
    }
    #[doc = "Either edge"]
    #[inline(always)]
    pub fn comp_acctl0_tsen_both(self) -> &'a mut W {
        self.variant(COMP_ACCTL0_TSENW::COMP_ACCTL0_TSEN_BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 5);
        self.w.bits |= ((value as u32) & 3) << 5;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct COMP_ACCTL0_TSLVALR {
    bits: bool,
}
impl COMP_ACCTL0_TSLVALR {
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
pub struct _COMP_ACCTL0_TSLVALW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_ACCTL0_TSLVALW<'a> {
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
#[doc = "Possible values of the field `COMP_ACCTL0_ASRCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_ACCTL0_ASRCPR {
    #[doc = "Pin value of Cn+"]
    COMP_ACCTL0_ASRCP_PIN,
    #[doc = "Pin value of C0+"]
    COMP_ACCTL0_ASRCP_PIN0,
    #[doc = "Internal voltage reference"]
    COMP_ACCTL0_ASRCP_REF,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl COMP_ACCTL0_ASRCPR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            COMP_ACCTL0_ASRCPR::COMP_ACCTL0_ASRCP_PIN => 0,
            COMP_ACCTL0_ASRCPR::COMP_ACCTL0_ASRCP_PIN0 => 1,
            COMP_ACCTL0_ASRCPR::COMP_ACCTL0_ASRCP_REF => 2,
            COMP_ACCTL0_ASRCPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> COMP_ACCTL0_ASRCPR {
        match value {
            0 => COMP_ACCTL0_ASRCPR::COMP_ACCTL0_ASRCP_PIN,
            1 => COMP_ACCTL0_ASRCPR::COMP_ACCTL0_ASRCP_PIN0,
            2 => COMP_ACCTL0_ASRCPR::COMP_ACCTL0_ASRCP_REF,
            i => COMP_ACCTL0_ASRCPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL0_ASRCP_PIN`"]
    #[inline(always)]
    pub fn is_comp_acctl0_asrcp_pin(&self) -> bool {
        *self == COMP_ACCTL0_ASRCPR::COMP_ACCTL0_ASRCP_PIN
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL0_ASRCP_PIN0`"]
    #[inline(always)]
    pub fn is_comp_acctl0_asrcp_pin0(&self) -> bool {
        *self == COMP_ACCTL0_ASRCPR::COMP_ACCTL0_ASRCP_PIN0
    }
    #[doc = "Checks if the value of the field is `COMP_ACCTL0_ASRCP_REF`"]
    #[inline(always)]
    pub fn is_comp_acctl0_asrcp_ref(&self) -> bool {
        *self == COMP_ACCTL0_ASRCPR::COMP_ACCTL0_ASRCP_REF
    }
}
#[doc = "Values that can be written to the field `COMP_ACCTL0_ASRCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_ACCTL0_ASRCPW {
    #[doc = "Pin value of Cn+"]
    COMP_ACCTL0_ASRCP_PIN,
    #[doc = "Pin value of C0+"]
    COMP_ACCTL0_ASRCP_PIN0,
    #[doc = "Internal voltage reference"]
    COMP_ACCTL0_ASRCP_REF,
}
impl COMP_ACCTL0_ASRCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMP_ACCTL0_ASRCPW::COMP_ACCTL0_ASRCP_PIN => 0,
            COMP_ACCTL0_ASRCPW::COMP_ACCTL0_ASRCP_PIN0 => 1,
            COMP_ACCTL0_ASRCPW::COMP_ACCTL0_ASRCP_REF => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _COMP_ACCTL0_ASRCPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_ACCTL0_ASRCPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_ACCTL0_ASRCPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Pin value of Cn+"]
    #[inline(always)]
    pub fn comp_acctl0_asrcp_pin(self) -> &'a mut W {
        self.variant(COMP_ACCTL0_ASRCPW::COMP_ACCTL0_ASRCP_PIN)
    }
    #[doc = "Pin value of C0+"]
    #[inline(always)]
    pub fn comp_acctl0_asrcp_pin0(self) -> &'a mut W {
        self.variant(COMP_ACCTL0_ASRCPW::COMP_ACCTL0_ASRCP_PIN0)
    }
    #[doc = "Internal voltage reference"]
    #[inline(always)]
    pub fn comp_acctl0_asrcp_ref(self) -> &'a mut W {
        self.variant(COMP_ACCTL0_ASRCPW::COMP_ACCTL0_ASRCP_REF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 9);
        self.w.bits |= ((value as u32) & 3) << 9;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct COMP_ACCTL0_TOENR {
    bits: bool,
}
impl COMP_ACCTL0_TOENR {
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
pub struct _COMP_ACCTL0_TOENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_ACCTL0_TOENW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline(always)]
    pub fn comp_acctl0_cinv(&self) -> COMP_ACCTL0_CINVR {
        let bits = ((self.bits >> 1) & 1) != 0;
        COMP_ACCTL0_CINVR { bits }
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline(always)]
    pub fn comp_acctl0_isen(&self) -> COMP_ACCTL0_ISENR {
        COMP_ACCTL0_ISENR::_from(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl0_islval(&self) -> COMP_ACCTL0_ISLVALR {
        let bits = ((self.bits >> 4) & 1) != 0;
        COMP_ACCTL0_ISLVALR { bits }
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline(always)]
    pub fn comp_acctl0_tsen(&self) -> COMP_ACCTL0_TSENR {
        COMP_ACCTL0_TSENR::_from(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl0_tslval(&self) -> COMP_ACCTL0_TSLVALR {
        let bits = ((self.bits >> 7) & 1) != 0;
        COMP_ACCTL0_TSLVALR { bits }
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline(always)]
    pub fn comp_acctl0_asrcp(&self) -> COMP_ACCTL0_ASRCPR {
        COMP_ACCTL0_ASRCPR::_from(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline(always)]
    pub fn comp_acctl0_toen(&self) -> COMP_ACCTL0_TOENR {
        let bits = ((self.bits >> 11) & 1) != 0;
        COMP_ACCTL0_TOENR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline(always)]
    pub fn comp_acctl0_cinv(&mut self) -> _COMP_ACCTL0_CINVW {
        _COMP_ACCTL0_CINVW { w: self }
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline(always)]
    pub fn comp_acctl0_isen(&mut self) -> _COMP_ACCTL0_ISENW {
        _COMP_ACCTL0_ISENW { w: self }
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl0_islval(&mut self) -> _COMP_ACCTL0_ISLVALW {
        _COMP_ACCTL0_ISLVALW { w: self }
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline(always)]
    pub fn comp_acctl0_tsen(&mut self) -> _COMP_ACCTL0_TSENW {
        _COMP_ACCTL0_TSENW { w: self }
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline(always)]
    pub fn comp_acctl0_tslval(&mut self) -> _COMP_ACCTL0_TSLVALW {
        _COMP_ACCTL0_TSLVALW { w: self }
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline(always)]
    pub fn comp_acctl0_asrcp(&mut self) -> _COMP_ACCTL0_ASRCPW {
        _COMP_ACCTL0_ASRCPW { w: self }
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline(always)]
    pub fn comp_acctl0_toen(&mut self) -> _COMP_ACCTL0_TOENW {
        _COMP_ACCTL0_TOENW { w: self }
    }
}
