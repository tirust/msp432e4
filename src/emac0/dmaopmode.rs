#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMAOPMODE {
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
pub struct EMAC_DMAOPMODE_SRR {
    bits: bool,
}
impl EMAC_DMAOPMODE_SRR {
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
pub struct _EMAC_DMAOPMODE_SRW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_SRW<'a> {
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
pub struct EMAC_DMAOPMODE_OSFR {
    bits: bool,
}
impl EMAC_DMAOPMODE_OSFR {
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
pub struct _EMAC_DMAOPMODE_OSFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_OSFW<'a> {
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
#[doc = "Possible values of the field `EMAC_DMAOPMODE_RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_DMAOPMODE_RTCR {
    #[doc = "64 bytes"]
    EMAC_DMAOPMODE_RTC_64,
    #[doc = "32 bytes"]
    EMAC_DMAOPMODE_RTC_32,
    #[doc = "96 bytes"]
    EMAC_DMAOPMODE_RTC_96,
    #[doc = "128 bytes"]
    EMAC_DMAOPMODE_RTC_128,
}
impl EMAC_DMAOPMODE_RTCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_64 => 0,
            EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_32 => 1,
            EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_96 => 2,
            EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_128 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_DMAOPMODE_RTCR {
        match value {
            0 => EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_64,
            1 => EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_32,
            2 => EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_96,
            3 => EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_RTC_64`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_rtc_64(&self) -> bool {
        *self == EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_64
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_RTC_32`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_rtc_32(&self) -> bool {
        *self == EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_32
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_RTC_96`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_rtc_96(&self) -> bool {
        *self == EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_96
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_RTC_128`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_rtc_128(&self) -> bool {
        *self == EMAC_DMAOPMODE_RTCR::EMAC_DMAOPMODE_RTC_128
    }
}
#[doc = "Values that can be written to the field `EMAC_DMAOPMODE_RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_DMAOPMODE_RTCW {
    #[doc = "64 bytes"]
    EMAC_DMAOPMODE_RTC_64,
    #[doc = "32 bytes"]
    EMAC_DMAOPMODE_RTC_32,
    #[doc = "96 bytes"]
    EMAC_DMAOPMODE_RTC_96,
    #[doc = "128 bytes"]
    EMAC_DMAOPMODE_RTC_128,
}
impl EMAC_DMAOPMODE_RTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_DMAOPMODE_RTCW::EMAC_DMAOPMODE_RTC_64 => 0,
            EMAC_DMAOPMODE_RTCW::EMAC_DMAOPMODE_RTC_32 => 1,
            EMAC_DMAOPMODE_RTCW::EMAC_DMAOPMODE_RTC_96 => 2,
            EMAC_DMAOPMODE_RTCW::EMAC_DMAOPMODE_RTC_128 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_DMAOPMODE_RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_RTCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_DMAOPMODE_RTCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc_64(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_RTCW::EMAC_DMAOPMODE_RTC_64)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc_32(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_RTCW::EMAC_DMAOPMODE_RTC_32)
    }
    #[doc = "96 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc_96(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_RTCW::EMAC_DMAOPMODE_RTC_96)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc_128(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_RTCW::EMAC_DMAOPMODE_RTC_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 3);
        self.w.bits |= ((value as u32) & 3) << 3;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMAOPMODE_DGFR {
    bits: bool,
}
impl EMAC_DMAOPMODE_DGFR {
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
pub struct _EMAC_DMAOPMODE_DGFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_DGFW<'a> {
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
pub struct EMAC_DMAOPMODE_FUFR {
    bits: bool,
}
impl EMAC_DMAOPMODE_FUFR {
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
pub struct _EMAC_DMAOPMODE_FUFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_FUFW<'a> {
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
pub struct EMAC_DMAOPMODE_FEFR {
    bits: bool,
}
impl EMAC_DMAOPMODE_FEFR {
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
pub struct _EMAC_DMAOPMODE_FEFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_FEFW<'a> {
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
pub struct EMAC_DMAOPMODE_STR {
    bits: bool,
}
impl EMAC_DMAOPMODE_STR {
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
pub struct _EMAC_DMAOPMODE_STW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_STW<'a> {
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
#[doc = "Possible values of the field `EMAC_DMAOPMODE_TTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_DMAOPMODE_TTCR {
    #[doc = "64 bytes"]
    EMAC_DMAOPMODE_TTC_64,
    #[doc = "128 bytes"]
    EMAC_DMAOPMODE_TTC_128,
    #[doc = "192 bytes"]
    EMAC_DMAOPMODE_TTC_192,
    #[doc = "256 bytes"]
    EMAC_DMAOPMODE_TTC_256,
    #[doc = "40 bytes"]
    EMAC_DMAOPMODE_TTC_40,
    #[doc = "32 bytes"]
    EMAC_DMAOPMODE_TTC_32,
    #[doc = "24 bytes"]
    EMAC_DMAOPMODE_TTC_24,
    #[doc = "16 bytes"]
    EMAC_DMAOPMODE_TTC_16,
}
impl EMAC_DMAOPMODE_TTCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_64 => 0,
            EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_128 => 1,
            EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_192 => 2,
            EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_256 => 3,
            EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_40 => 4,
            EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_32 => 5,
            EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_24 => 6,
            EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_16 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_DMAOPMODE_TTCR {
        match value {
            0 => EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_64,
            1 => EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_128,
            2 => EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_192,
            3 => EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_256,
            4 => EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_40,
            5 => EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_32,
            6 => EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_24,
            7 => EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_64`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_64(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_64
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_128`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_128(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_128
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_192`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_192(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_192
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_256`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_256(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_256
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_40`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_40(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_40
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_32`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_32(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_32
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_24`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_24(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_24
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_16`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_16(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTCR::EMAC_DMAOPMODE_TTC_16
    }
}
#[doc = "Values that can be written to the field `EMAC_DMAOPMODE_TTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_DMAOPMODE_TTCW {
    #[doc = "64 bytes"]
    EMAC_DMAOPMODE_TTC_64,
    #[doc = "128 bytes"]
    EMAC_DMAOPMODE_TTC_128,
    #[doc = "192 bytes"]
    EMAC_DMAOPMODE_TTC_192,
    #[doc = "256 bytes"]
    EMAC_DMAOPMODE_TTC_256,
    #[doc = "40 bytes"]
    EMAC_DMAOPMODE_TTC_40,
    #[doc = "32 bytes"]
    EMAC_DMAOPMODE_TTC_32,
    #[doc = "24 bytes"]
    EMAC_DMAOPMODE_TTC_24,
    #[doc = "16 bytes"]
    EMAC_DMAOPMODE_TTC_16,
}
impl EMAC_DMAOPMODE_TTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_64 => 0,
            EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_128 => 1,
            EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_192 => 2,
            EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_256 => 3,
            EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_40 => 4,
            EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_32 => 5,
            EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_24 => 6,
            EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_16 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_DMAOPMODE_TTCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_TTCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_DMAOPMODE_TTCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_64(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_128(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_128)
    }
    #[doc = "192 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_192(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_192)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_256(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_256)
    }
    #[doc = "40 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_40(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_40)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_32(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_32)
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_24(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_24)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_16(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTCW::EMAC_DMAOPMODE_TTC_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 14);
        self.w.bits |= ((value as u32) & 7) << 14;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMAOPMODE_FTFR {
    bits: bool,
}
impl EMAC_DMAOPMODE_FTFR {
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
pub struct _EMAC_DMAOPMODE_FTFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_FTFW<'a> {
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
        self.w.bits &= !(1 << 20);
        self.w.bits |= ((value as u32) & 1) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMAOPMODE_TSFR {
    bits: bool,
}
impl EMAC_DMAOPMODE_TSFR {
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
pub struct _EMAC_DMAOPMODE_TSFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_TSFW<'a> {
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
        self.w.bits &= !(1 << 21);
        self.w.bits |= ((value as u32) & 1) << 21;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMAOPMODE_DFFR {
    bits: bool,
}
impl EMAC_DMAOPMODE_DFFR {
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
pub struct _EMAC_DMAOPMODE_DFFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_DFFW<'a> {
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
        self.w.bits &= !(1 << 24);
        self.w.bits |= ((value as u32) & 1) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMAOPMODE_RSFR {
    bits: bool,
}
impl EMAC_DMAOPMODE_RSFR {
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
pub struct _EMAC_DMAOPMODE_RSFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_RSFW<'a> {
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
        self.w.bits &= !(1 << 25);
        self.w.bits |= ((value as u32) & 1) << 25;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_DMAOPMODE_DTR {
    bits: bool,
}
impl EMAC_DMAOPMODE_DTR {
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
pub struct _EMAC_DMAOPMODE_DTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_DMAOPMODE_DTW<'a> {
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
        self.w.bits &= !(1 << 26);
        self.w.bits |= ((value as u32) & 1) << 26;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline(always)]
    pub fn emac_dmaopmode_sr(&self) -> EMAC_DMAOPMODE_SRR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EMAC_DMAOPMODE_SRR { bits }
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline(always)]
    pub fn emac_dmaopmode_osf(&self) -> EMAC_DMAOPMODE_OSFR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EMAC_DMAOPMODE_OSFR { bits }
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc(&self) -> EMAC_DMAOPMODE_RTCR {
        EMAC_DMAOPMODE_RTCR::_from(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Drop Giant Frame Enable"]
    #[inline(always)]
    pub fn emac_dmaopmode_dgf(&self) -> EMAC_DMAOPMODE_DGFR {
        let bits = ((self.bits >> 5) & 1) != 0;
        EMAC_DMAOPMODE_DGFR { bits }
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_fuf(&self) -> EMAC_DMAOPMODE_FUFR {
        let bits = ((self.bits >> 6) & 1) != 0;
        EMAC_DMAOPMODE_FUFR { bits }
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_fef(&self) -> EMAC_DMAOPMODE_FEFR {
        let bits = ((self.bits >> 7) & 1) != 0;
        EMAC_DMAOPMODE_FEFR { bits }
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn emac_dmaopmode_st(&self) -> EMAC_DMAOPMODE_STR {
        let bits = ((self.bits >> 13) & 1) != 0;
        EMAC_DMAOPMODE_STR { bits }
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc(&self) -> EMAC_DMAOPMODE_TTCR {
        EMAC_DMAOPMODE_TTCR::_from(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    pub fn emac_dmaopmode_ftf(&self) -> EMAC_DMAOPMODE_FTFR {
        let bits = ((self.bits >> 20) & 1) != 0;
        EMAC_DMAOPMODE_FTFR { bits }
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn emac_dmaopmode_tsf(&self) -> EMAC_DMAOPMODE_TSFR {
        let bits = ((self.bits >> 21) & 1) != 0;
        EMAC_DMAOPMODE_TSFR { bits }
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_dff(&self) -> EMAC_DMAOPMODE_DFFR {
        let bits = ((self.bits >> 24) & 1) != 0;
        EMAC_DMAOPMODE_DFFR { bits }
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline(always)]
    pub fn emac_dmaopmode_rsf(&self) -> EMAC_DMAOPMODE_RSFR {
        let bits = ((self.bits >> 25) & 1) != 0;
        EMAC_DMAOPMODE_RSFR { bits }
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_dt(&self) -> EMAC_DMAOPMODE_DTR {
        let bits = ((self.bits >> 26) & 1) != 0;
        EMAC_DMAOPMODE_DTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline(always)]
    pub fn emac_dmaopmode_sr(&mut self) -> _EMAC_DMAOPMODE_SRW {
        _EMAC_DMAOPMODE_SRW { w: self }
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline(always)]
    pub fn emac_dmaopmode_osf(&mut self) -> _EMAC_DMAOPMODE_OSFW {
        _EMAC_DMAOPMODE_OSFW { w: self }
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc(&mut self) -> _EMAC_DMAOPMODE_RTCW {
        _EMAC_DMAOPMODE_RTCW { w: self }
    }
    #[doc = "Bit 5 - Drop Giant Frame Enable"]
    #[inline(always)]
    pub fn emac_dmaopmode_dgf(&mut self) -> _EMAC_DMAOPMODE_DGFW {
        _EMAC_DMAOPMODE_DGFW { w: self }
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_fuf(&mut self) -> _EMAC_DMAOPMODE_FUFW {
        _EMAC_DMAOPMODE_FUFW { w: self }
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_fef(&mut self) -> _EMAC_DMAOPMODE_FEFW {
        _EMAC_DMAOPMODE_FEFW { w: self }
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn emac_dmaopmode_st(&mut self) -> _EMAC_DMAOPMODE_STW {
        _EMAC_DMAOPMODE_STW { w: self }
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc(&mut self) -> _EMAC_DMAOPMODE_TTCW {
        _EMAC_DMAOPMODE_TTCW { w: self }
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    pub fn emac_dmaopmode_ftf(&mut self) -> _EMAC_DMAOPMODE_FTFW {
        _EMAC_DMAOPMODE_FTFW { w: self }
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn emac_dmaopmode_tsf(&mut self) -> _EMAC_DMAOPMODE_TSFW {
        _EMAC_DMAOPMODE_TSFW { w: self }
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_dff(&mut self) -> _EMAC_DMAOPMODE_DFFW {
        _EMAC_DMAOPMODE_DFFW { w: self }
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline(always)]
    pub fn emac_dmaopmode_rsf(&mut self) -> _EMAC_DMAOPMODE_RSFW {
        _EMAC_DMAOPMODE_RSFW { w: self }
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_dt(&mut self) -> _EMAC_DMAOPMODE_DTW {
        _EMAC_DMAOPMODE_DTW { w: self }
    }
}
