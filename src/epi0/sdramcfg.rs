#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMCFG {
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
#[doc = "Possible values of the field `EPI_SDRAMCFG_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_SDRAMCFG_SIZER {
    #[doc = "64 megabits (8MB)"]
    EPI_SDRAMCFG_SIZE_8MB,
    #[doc = "128 megabits (16MB)"]
    EPI_SDRAMCFG_SIZE_16MB,
    #[doc = "256 megabits (32MB)"]
    EPI_SDRAMCFG_SIZE_32MB,
    #[doc = "512 megabits (64MB)"]
    EPI_SDRAMCFG_SIZE_64MB,
}
impl EPI_SDRAMCFG_SIZER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_8MB => 0,
            EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_16MB => 1,
            EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_32MB => 2,
            EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_64MB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_SDRAMCFG_SIZER {
        match value {
            0 => EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_8MB,
            1 => EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_16MB,
            2 => EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_32MB,
            3 => EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_64MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_SIZE_8MB`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_size_8mb(&self) -> bool {
        *self == EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_8MB
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_SIZE_16MB`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_size_16mb(&self) -> bool {
        *self == EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_16MB
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_SIZE_32MB`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_size_32mb(&self) -> bool {
        *self == EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_32MB
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_SIZE_64MB`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_size_64mb(&self) -> bool {
        *self == EPI_SDRAMCFG_SIZER::EPI_SDRAMCFG_SIZE_64MB
    }
}
#[doc = "Values that can be written to the field `EPI_SDRAMCFG_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_SDRAMCFG_SIZEW {
    #[doc = "64 megabits (8MB)"]
    EPI_SDRAMCFG_SIZE_8MB,
    #[doc = "128 megabits (16MB)"]
    EPI_SDRAMCFG_SIZE_16MB,
    #[doc = "256 megabits (32MB)"]
    EPI_SDRAMCFG_SIZE_32MB,
    #[doc = "512 megabits (64MB)"]
    EPI_SDRAMCFG_SIZE_64MB,
}
impl EPI_SDRAMCFG_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_SDRAMCFG_SIZEW::EPI_SDRAMCFG_SIZE_8MB => 0,
            EPI_SDRAMCFG_SIZEW::EPI_SDRAMCFG_SIZE_16MB => 1,
            EPI_SDRAMCFG_SIZEW::EPI_SDRAMCFG_SIZE_32MB => 2,
            EPI_SDRAMCFG_SIZEW::EPI_SDRAMCFG_SIZE_64MB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_SDRAMCFG_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_SDRAMCFG_SIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_SDRAMCFG_SIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "64 megabits (8MB)"]
    #[inline(always)]
    pub fn epi_sdramcfg_size_8mb(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_SIZEW::EPI_SDRAMCFG_SIZE_8MB)
    }
    #[doc = "128 megabits (16MB)"]
    #[inline(always)]
    pub fn epi_sdramcfg_size_16mb(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_SIZEW::EPI_SDRAMCFG_SIZE_16MB)
    }
    #[doc = "256 megabits (32MB)"]
    #[inline(always)]
    pub fn epi_sdramcfg_size_32mb(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_SIZEW::EPI_SDRAMCFG_SIZE_32MB)
    }
    #[doc = "512 megabits (64MB)"]
    #[inline(always)]
    pub fn epi_sdramcfg_size_64mb(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_SIZEW::EPI_SDRAMCFG_SIZE_64MB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_SDRAMCFG_SLEEPR {
    bits: bool,
}
impl EPI_SDRAMCFG_SLEEPR {
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
pub struct _EPI_SDRAMCFG_SLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_SDRAMCFG_SLEEPW<'a> {
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
pub struct EPI_SDRAMCFG_RFSHR {
    bits: u16,
}
impl EPI_SDRAMCFG_RFSHR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EPI_SDRAMCFG_RFSHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_SDRAMCFG_RFSHW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(2047 << 16);
        self.w.bits |= ((value as u32) & 2047) << 16;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_SDRAMCFG_FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_SDRAMCFG_FREQR {
    #[doc = "0 - 15 MHz"]
    EPI_SDRAMCFG_FREQ_NONE,
    #[doc = "15 - 30 MHz"]
    EPI_SDRAMCFG_FREQ_15MHZ,
    #[doc = "30 - 50 MHz"]
    EPI_SDRAMCFG_FREQ_30MHZ,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EPI_SDRAMCFG_FREQR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_SDRAMCFG_FREQR::EPI_SDRAMCFG_FREQ_NONE => 0,
            EPI_SDRAMCFG_FREQR::EPI_SDRAMCFG_FREQ_15MHZ => 1,
            EPI_SDRAMCFG_FREQR::EPI_SDRAMCFG_FREQ_30MHZ => 2,
            EPI_SDRAMCFG_FREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_SDRAMCFG_FREQR {
        match value {
            0 => EPI_SDRAMCFG_FREQR::EPI_SDRAMCFG_FREQ_NONE,
            1 => EPI_SDRAMCFG_FREQR::EPI_SDRAMCFG_FREQ_15MHZ,
            2 => EPI_SDRAMCFG_FREQR::EPI_SDRAMCFG_FREQ_30MHZ,
            i => EPI_SDRAMCFG_FREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_FREQ_NONE`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_freq_none(&self) -> bool {
        *self == EPI_SDRAMCFG_FREQR::EPI_SDRAMCFG_FREQ_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_FREQ_15MHZ`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_freq_15mhz(&self) -> bool {
        *self == EPI_SDRAMCFG_FREQR::EPI_SDRAMCFG_FREQ_15MHZ
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_FREQ_30MHZ`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_freq_30mhz(&self) -> bool {
        *self == EPI_SDRAMCFG_FREQR::EPI_SDRAMCFG_FREQ_30MHZ
    }
}
#[doc = "Values that can be written to the field `EPI_SDRAMCFG_FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_SDRAMCFG_FREQW {
    #[doc = "0 - 15 MHz"]
    EPI_SDRAMCFG_FREQ_NONE,
    #[doc = "15 - 30 MHz"]
    EPI_SDRAMCFG_FREQ_15MHZ,
    #[doc = "30 - 50 MHz"]
    EPI_SDRAMCFG_FREQ_30MHZ,
}
impl EPI_SDRAMCFG_FREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_SDRAMCFG_FREQW::EPI_SDRAMCFG_FREQ_NONE => 0,
            EPI_SDRAMCFG_FREQW::EPI_SDRAMCFG_FREQ_15MHZ => 1,
            EPI_SDRAMCFG_FREQW::EPI_SDRAMCFG_FREQ_30MHZ => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_SDRAMCFG_FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_SDRAMCFG_FREQW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_SDRAMCFG_FREQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0 - 15 MHz"]
    #[inline(always)]
    pub fn epi_sdramcfg_freq_none(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_FREQW::EPI_SDRAMCFG_FREQ_NONE)
    }
    #[doc = "15 - 30 MHz"]
    #[inline(always)]
    pub fn epi_sdramcfg_freq_15mhz(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_FREQW::EPI_SDRAMCFG_FREQ_15MHZ)
    }
    #[doc = "30 - 50 MHz"]
    #[inline(always)]
    pub fn epi_sdramcfg_freq_30mhz(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_FREQW::EPI_SDRAMCFG_FREQ_30MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 30);
        self.w.bits |= ((value as u32) & 3) << 30;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Size of SDRAM"]
    #[inline(always)]
    pub fn epi_sdramcfg_size(&self) -> EPI_SDRAMCFG_SIZER {
        EPI_SDRAMCFG_SIZER::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bit 9 - Sleep Mode"]
    #[inline(always)]
    pub fn epi_sdramcfg_sleep(&self) -> EPI_SDRAMCFG_SLEEPR {
        let bits = ((self.bits >> 9) & 1) != 0;
        EPI_SDRAMCFG_SLEEPR { bits }
    }
    #[doc = "Bits 16:26 - Refresh Counter"]
    #[inline(always)]
    pub fn epi_sdramcfg_rfsh(&self) -> EPI_SDRAMCFG_RFSHR {
        let bits = ((self.bits >> 16) & 2047) as u16;
        EPI_SDRAMCFG_RFSHR { bits }
    }
    #[doc = "Bits 30:31 - EPI Frequency Range"]
    #[inline(always)]
    pub fn epi_sdramcfg_freq(&self) -> EPI_SDRAMCFG_FREQR {
        EPI_SDRAMCFG_FREQR::_from(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Size of SDRAM"]
    #[inline(always)]
    pub fn epi_sdramcfg_size(&mut self) -> _EPI_SDRAMCFG_SIZEW {
        _EPI_SDRAMCFG_SIZEW { w: self }
    }
    #[doc = "Bit 9 - Sleep Mode"]
    #[inline(always)]
    pub fn epi_sdramcfg_sleep(&mut self) -> _EPI_SDRAMCFG_SLEEPW {
        _EPI_SDRAMCFG_SLEEPW { w: self }
    }
    #[doc = "Bits 16:26 - Refresh Counter"]
    #[inline(always)]
    pub fn epi_sdramcfg_rfsh(&mut self) -> _EPI_SDRAMCFG_RFSHW {
        _EPI_SDRAMCFG_RFSHW { w: self }
    }
    #[doc = "Bits 30:31 - EPI Frequency Range"]
    #[inline(always)]
    pub fn epi_sdramcfg_freq(&mut self) -> _EPI_SDRAMCFG_FREQW {
        _EPI_SDRAMCFG_FREQW { w: self }
    }
}
