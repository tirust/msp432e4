#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSCLKCFG {
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
pub struct SYSCTL_RSCLKCFG_PSYSDIVR {
    bits: u16,
}
impl SYSCTL_RSCLKCFG_PSYSDIVR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_RSCLKCFG_PSYSDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RSCLKCFG_PSYSDIVW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(1023 << 0);
        self.w.bits |= ((value as u32) & 1023) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_RSCLKCFG_OSYSDIVR {
    bits: u16,
}
impl SYSCTL_RSCLKCFG_OSYSDIVR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_RSCLKCFG_OSYSDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RSCLKCFG_OSYSDIVW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(1023 << 10);
        self.w.bits |= ((value as u32) & 1023) << 10;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_RSCLKCFG_OSCSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RSCLKCFG_OSCSRCR {
    #[doc = "PIOSC is oscillator source"]
    SYSCTL_RSCLKCFG_OSCSRC_PIOSC,
    #[doc = "LFIOSC is oscillator source"]
    SYSCTL_RSCLKCFG_OSCSRC_LFIOSC,
    #[doc = "MOSC is oscillator source"]
    SYSCTL_RSCLKCFG_OSCSRC_MOSC,
    #[doc = "Hibernation Module RTC Oscillator (RTCOSC)"]
    SYSCTL_RSCLKCFG_OSCSRC_RTC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_RSCLKCFG_OSCSRCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_PIOSC => 0,
            SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_LFIOSC => 2,
            SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_MOSC => 3,
            SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_RTC => 4,
            SYSCTL_RSCLKCFG_OSCSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_RSCLKCFG_OSCSRCR {
        match value {
            0 => SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_PIOSC,
            2 => SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_LFIOSC,
            3 => SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_MOSC,
            4 => SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_RTC,
            i => SYSCTL_RSCLKCFG_OSCSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_OSCSRC_PIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_oscsrc_piosc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_PIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_OSCSRC_LFIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_oscsrc_lfiosc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_LFIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_OSCSRC_MOSC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_oscsrc_mosc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_MOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_OSCSRC_RTC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_oscsrc_rtc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_OSCSRCR::SYSCTL_RSCLKCFG_OSCSRC_RTC
    }
}
#[doc = "Values that can be written to the field `SYSCTL_RSCLKCFG_OSCSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RSCLKCFG_OSCSRCW {
    #[doc = "PIOSC is oscillator source"]
    SYSCTL_RSCLKCFG_OSCSRC_PIOSC,
    #[doc = "LFIOSC is oscillator source"]
    SYSCTL_RSCLKCFG_OSCSRC_LFIOSC,
    #[doc = "MOSC is oscillator source"]
    SYSCTL_RSCLKCFG_OSCSRC_MOSC,
    #[doc = "Hibernation Module RTC Oscillator (RTCOSC)"]
    SYSCTL_RSCLKCFG_OSCSRC_RTC,
}
impl SYSCTL_RSCLKCFG_OSCSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_RSCLKCFG_OSCSRCW::SYSCTL_RSCLKCFG_OSCSRC_PIOSC => 0,
            SYSCTL_RSCLKCFG_OSCSRCW::SYSCTL_RSCLKCFG_OSCSRC_LFIOSC => 2,
            SYSCTL_RSCLKCFG_OSCSRCW::SYSCTL_RSCLKCFG_OSCSRC_MOSC => 3,
            SYSCTL_RSCLKCFG_OSCSRCW::SYSCTL_RSCLKCFG_OSCSRC_RTC => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_RSCLKCFG_OSCSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RSCLKCFG_OSCSRCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RSCLKCFG_OSCSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PIOSC is oscillator source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc_piosc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_OSCSRCW::SYSCTL_RSCLKCFG_OSCSRC_PIOSC)
    }
    #[doc = "LFIOSC is oscillator source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc_lfiosc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_OSCSRCW::SYSCTL_RSCLKCFG_OSCSRC_LFIOSC)
    }
    #[doc = "MOSC is oscillator source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc_mosc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_OSCSRCW::SYSCTL_RSCLKCFG_OSCSRC_MOSC)
    }
    #[doc = "Hibernation Module RTC Oscillator (RTCOSC)"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc_rtc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_OSCSRCW::SYSCTL_RSCLKCFG_OSCSRC_RTC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 20);
        self.w.bits |= ((value as u32) & 15) << 20;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_RSCLKCFG_PLLSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RSCLKCFG_PLLSRCR {
    #[doc = "PIOSC is PLL input clock source"]
    SYSCTL_RSCLKCFG_PLLSRC_PIOSC,
    #[doc = "MOSC is the PLL input clock source"]
    SYSCTL_RSCLKCFG_PLLSRC_MOSC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_RSCLKCFG_PLLSRCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_RSCLKCFG_PLLSRCR::SYSCTL_RSCLKCFG_PLLSRC_PIOSC => 0,
            SYSCTL_RSCLKCFG_PLLSRCR::SYSCTL_RSCLKCFG_PLLSRC_MOSC => 3,
            SYSCTL_RSCLKCFG_PLLSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_RSCLKCFG_PLLSRCR {
        match value {
            0 => SYSCTL_RSCLKCFG_PLLSRCR::SYSCTL_RSCLKCFG_PLLSRC_PIOSC,
            3 => SYSCTL_RSCLKCFG_PLLSRCR::SYSCTL_RSCLKCFG_PLLSRC_MOSC,
            i => SYSCTL_RSCLKCFG_PLLSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_PLLSRC_PIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_pllsrc_piosc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_PLLSRCR::SYSCTL_RSCLKCFG_PLLSRC_PIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_PLLSRC_MOSC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_pllsrc_mosc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_PLLSRCR::SYSCTL_RSCLKCFG_PLLSRC_MOSC
    }
}
#[doc = "Values that can be written to the field `SYSCTL_RSCLKCFG_PLLSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RSCLKCFG_PLLSRCW {
    #[doc = "PIOSC is PLL input clock source"]
    SYSCTL_RSCLKCFG_PLLSRC_PIOSC,
    #[doc = "MOSC is the PLL input clock source"]
    SYSCTL_RSCLKCFG_PLLSRC_MOSC,
}
impl SYSCTL_RSCLKCFG_PLLSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_RSCLKCFG_PLLSRCW::SYSCTL_RSCLKCFG_PLLSRC_PIOSC => 0,
            SYSCTL_RSCLKCFG_PLLSRCW::SYSCTL_RSCLKCFG_PLLSRC_MOSC => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_RSCLKCFG_PLLSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RSCLKCFG_PLLSRCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RSCLKCFG_PLLSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PIOSC is PLL input clock source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_pllsrc_piosc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_PLLSRCW::SYSCTL_RSCLKCFG_PLLSRC_PIOSC)
    }
    #[doc = "MOSC is the PLL input clock source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_pllsrc_mosc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_PLLSRCW::SYSCTL_RSCLKCFG_PLLSRC_MOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 24);
        self.w.bits |= ((value as u32) & 15) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_RSCLKCFG_USEPLLR {
    bits: bool,
}
impl SYSCTL_RSCLKCFG_USEPLLR {
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
pub struct _SYSCTL_RSCLKCFG_USEPLLW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RSCLKCFG_USEPLLW<'a> {
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
        self.w.bits &= !(1 << 28);
        self.w.bits |= ((value as u32) & 1) << 28;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_RSCLKCFG_ACGR {
    bits: bool,
}
impl SYSCTL_RSCLKCFG_ACGR {
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
pub struct _SYSCTL_RSCLKCFG_ACGW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RSCLKCFG_ACGW<'a> {
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
        self.w.bits &= !(1 << 29);
        self.w.bits |= ((value as u32) & 1) << 29;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_RSCLKCFG_NEWFREQR {
    bits: bool,
}
impl SYSCTL_RSCLKCFG_NEWFREQR {
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
pub struct _SYSCTL_RSCLKCFG_NEWFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RSCLKCFG_NEWFREQW<'a> {
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
pub struct SYSCTL_RSCLKCFG_MEMTIMUR {
    bits: bool,
}
impl SYSCTL_RSCLKCFG_MEMTIMUR {
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
pub struct _SYSCTL_RSCLKCFG_MEMTIMUW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_RSCLKCFG_MEMTIMUW<'a> {
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
    #[doc = "Bits 0:9 - PLL System Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_psysdiv(&self) -> SYSCTL_RSCLKCFG_PSYSDIVR {
        let bits = ((self.bits >> 0) & 1023) as u16;
        SYSCTL_RSCLKCFG_PSYSDIVR { bits }
    }
    #[doc = "Bits 10:19 - Oscillator System Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_osysdiv(&self) -> SYSCTL_RSCLKCFG_OSYSDIVR {
        let bits = ((self.bits >> 10) & 1023) as u16;
        SYSCTL_RSCLKCFG_OSYSDIVR { bits }
    }
    #[doc = "Bits 20:23 - Oscillator Source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc(&self) -> SYSCTL_RSCLKCFG_OSCSRCR {
        SYSCTL_RSCLKCFG_OSCSRCR::_from(((self.bits >> 20) & 15) as u8)
    }
    #[doc = "Bits 24:27 - PLL Source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_pllsrc(&self) -> SYSCTL_RSCLKCFG_PLLSRCR {
        SYSCTL_RSCLKCFG_PLLSRCR::_from(((self.bits >> 24) & 15) as u8)
    }
    #[doc = "Bit 28 - Use PLL"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_usepll(&self) -> SYSCTL_RSCLKCFG_USEPLLR {
        let bits = ((self.bits >> 28) & 1) != 0;
        SYSCTL_RSCLKCFG_USEPLLR { bits }
    }
    #[doc = "Bit 29 - Auto Clock Gating"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_acg(&self) -> SYSCTL_RSCLKCFG_ACGR {
        let bits = ((self.bits >> 29) & 1) != 0;
        SYSCTL_RSCLKCFG_ACGR { bits }
    }
    #[doc = "Bit 30 - New PLLFREQ Accept"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_newfreq(&self) -> SYSCTL_RSCLKCFG_NEWFREQR {
        let bits = ((self.bits >> 30) & 1) != 0;
        SYSCTL_RSCLKCFG_NEWFREQR { bits }
    }
    #[doc = "Bit 31 - Memory Timing Register Update"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_memtimu(&self) -> SYSCTL_RSCLKCFG_MEMTIMUR {
        let bits = ((self.bits >> 31) & 1) != 0;
        SYSCTL_RSCLKCFG_MEMTIMUR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - PLL System Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_psysdiv(&mut self) -> _SYSCTL_RSCLKCFG_PSYSDIVW {
        _SYSCTL_RSCLKCFG_PSYSDIVW { w: self }
    }
    #[doc = "Bits 10:19 - Oscillator System Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_osysdiv(&mut self) -> _SYSCTL_RSCLKCFG_OSYSDIVW {
        _SYSCTL_RSCLKCFG_OSYSDIVW { w: self }
    }
    #[doc = "Bits 20:23 - Oscillator Source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc(&mut self) -> _SYSCTL_RSCLKCFG_OSCSRCW {
        _SYSCTL_RSCLKCFG_OSCSRCW { w: self }
    }
    #[doc = "Bits 24:27 - PLL Source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_pllsrc(&mut self) -> _SYSCTL_RSCLKCFG_PLLSRCW {
        _SYSCTL_RSCLKCFG_PLLSRCW { w: self }
    }
    #[doc = "Bit 28 - Use PLL"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_usepll(&mut self) -> _SYSCTL_RSCLKCFG_USEPLLW {
        _SYSCTL_RSCLKCFG_USEPLLW { w: self }
    }
    #[doc = "Bit 29 - Auto Clock Gating"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_acg(&mut self) -> _SYSCTL_RSCLKCFG_ACGW {
        _SYSCTL_RSCLKCFG_ACGW { w: self }
    }
    #[doc = "Bit 30 - New PLLFREQ Accept"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_newfreq(&mut self) -> _SYSCTL_RSCLKCFG_NEWFREQW {
        _SYSCTL_RSCLKCFG_NEWFREQW { w: self }
    }
    #[doc = "Bit 31 - Memory Timing Register Update"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_memtimu(&mut self) -> _SYSCTL_RSCLKCFG_MEMTIMUW {
        _SYSCTL_RSCLKCFG_MEMTIMUW { w: self }
    }
}
