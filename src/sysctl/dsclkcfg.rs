#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DSCLKCFG {
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
pub struct SYSCTL_DSCLKCFG_DSSYSDIVR {
    bits: u16,
}
impl SYSCTL_DSCLKCFG_DSSYSDIVR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DSCLKCFG_DSSYSDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DSCLKCFG_DSSYSDIVW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(1023 << 0);
        self.w.bits |= ((value as u32) & 1023) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_DSCLKCFG_DSOSCSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DSCLKCFG_DSOSCSRCR {
    #[doc = "PIOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC,
    #[doc = "LFIOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC,
    #[doc = "MOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_MOSC,
    #[doc = "Hibernation Module RTCOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_RTC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DSCLKCFG_DSOSCSRCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC => 0,
            SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC => 2,
            SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_MOSC => 3,
            SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_RTC => 4,
            SYSCTL_DSCLKCFG_DSOSCSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DSCLKCFG_DSOSCSRCR {
        match value {
            0 => SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC,
            2 => SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC,
            3 => SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_MOSC,
            4 => SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_RTC,
            i => SYSCTL_DSCLKCFG_DSOSCSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_dsclkcfg_dsoscsrc_piosc(&self) -> bool {
        *self == SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_dsclkcfg_dsoscsrc_lfiosc(&self) -> bool {
        *self == SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSCLKCFG_DSOSCSRC_MOSC`"]
    #[inline(always)]
    pub fn is_sysctl_dsclkcfg_dsoscsrc_mosc(&self) -> bool {
        *self == SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_MOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSCLKCFG_DSOSCSRC_RTC`"]
    #[inline(always)]
    pub fn is_sysctl_dsclkcfg_dsoscsrc_rtc(&self) -> bool {
        *self == SYSCTL_DSCLKCFG_DSOSCSRCR::SYSCTL_DSCLKCFG_DSOSCSRC_RTC
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DSCLKCFG_DSOSCSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DSCLKCFG_DSOSCSRCW {
    #[doc = "PIOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC,
    #[doc = "LFIOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC,
    #[doc = "MOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_MOSC,
    #[doc = "Hibernation Module RTCOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_RTC,
}
impl SYSCTL_DSCLKCFG_DSOSCSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DSCLKCFG_DSOSCSRCW::SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC => 0,
            SYSCTL_DSCLKCFG_DSOSCSRCW::SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC => 2,
            SYSCTL_DSCLKCFG_DSOSCSRCW::SYSCTL_DSCLKCFG_DSOSCSRC_MOSC => 3,
            SYSCTL_DSCLKCFG_DSOSCSRCW::SYSCTL_DSCLKCFG_DSOSCSRC_RTC => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DSCLKCFG_DSOSCSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DSCLKCFG_DSOSCSRCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DSCLKCFG_DSOSCSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc_piosc(self) -> &'a mut W {
        self.variant(SYSCTL_DSCLKCFG_DSOSCSRCW::SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC)
    }
    #[doc = "LFIOSC"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc_lfiosc(self) -> &'a mut W {
        self.variant(SYSCTL_DSCLKCFG_DSOSCSRCW::SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc_mosc(self) -> &'a mut W {
        self.variant(SYSCTL_DSCLKCFG_DSOSCSRCW::SYSCTL_DSCLKCFG_DSOSCSRC_MOSC)
    }
    #[doc = "Hibernation Module RTCOSC"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc_rtc(self) -> &'a mut W {
        self.variant(SYSCTL_DSCLKCFG_DSOSCSRCW::SYSCTL_DSCLKCFG_DSOSCSRC_RTC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 20);
        self.w.bits |= ((value as u32) & 15) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_DSCLKCFG_MOSCDPDR {
    bits: bool,
}
impl SYSCTL_DSCLKCFG_MOSCDPDR {
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
pub struct _SYSCTL_DSCLKCFG_MOSCDPDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DSCLKCFG_MOSCDPDW<'a> {
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
        self.w.bits &= !(1 << 30);
        self.w.bits |= ((value as u32) & 1) << 30;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_DSCLKCFG_PIOSCPDR {
    bits: bool,
}
impl SYSCTL_DSCLKCFG_PIOSCPDR {
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
pub struct _SYSCTL_DSCLKCFG_PIOSCPDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DSCLKCFG_PIOSCPDW<'a> {
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
        self.w.bits &= !(1 << 31);
        self.w.bits |= ((value as u32) & 1) << 31;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - Deep Sleep Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dssysdiv(&self) -> SYSCTL_DSCLKCFG_DSSYSDIVR {
        let bits = ((self.bits >> 0) & 1023) as u16;
        SYSCTL_DSCLKCFG_DSSYSDIVR { bits }
    }
    #[doc = "Bits 20:23 - Deep Sleep Oscillator Source"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc(&self) -> SYSCTL_DSCLKCFG_DSOSCSRCR {
        SYSCTL_DSCLKCFG_DSOSCSRCR::_from(((self.bits >> 20) & 15) as u8)
    }
    #[doc = "Bit 30 - MOSC Disable Power Down"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_moscdpd(&self) -> SYSCTL_DSCLKCFG_MOSCDPDR {
        let bits = ((self.bits >> 30) & 1) != 0;
        SYSCTL_DSCLKCFG_MOSCDPDR { bits }
    }
    #[doc = "Bit 31 - PIOSC Power Down"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_pioscpd(&self) -> SYSCTL_DSCLKCFG_PIOSCPDR {
        let bits = ((self.bits >> 31) & 1) != 0;
        SYSCTL_DSCLKCFG_PIOSCPDR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Deep Sleep Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dssysdiv(&mut self) -> _SYSCTL_DSCLKCFG_DSSYSDIVW {
        _SYSCTL_DSCLKCFG_DSSYSDIVW { w: self }
    }
    #[doc = "Bits 20:23 - Deep Sleep Oscillator Source"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc(&mut self) -> _SYSCTL_DSCLKCFG_DSOSCSRCW {
        _SYSCTL_DSCLKCFG_DSOSCSRCW { w: self }
    }
    #[doc = "Bit 30 - MOSC Disable Power Down"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_moscdpd(&mut self) -> _SYSCTL_DSCLKCFG_MOSCDPDW {
        _SYSCTL_DSCLKCFG_MOSCDPDW { w: self }
    }
    #[doc = "Bit 31 - PIOSC Power Down"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_pioscpd(&mut self) -> _SYSCTL_DSCLKCFG_PIOSCPDW {
        _SYSCTL_DSCLKCFG_PIOSCPDW { w: self }
    }
}
