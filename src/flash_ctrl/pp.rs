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
#[doc = r"Value of the field"]
pub struct FLASH_PP_SIZER {
    bits: u16,
}
impl FLASH_PP_SIZER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_PP_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_PP_SIZEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(65535 << 0);
        self.w.bits |= ((value as u32) & 65535) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `FLASH_PP_MAINSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PP_MAINSSR {
    #[doc = "1 KB"]
    FLASH_PP_MAINSS_1KB,
    #[doc = "2 KB"]
    FLASH_PP_MAINSS_2KB,
    #[doc = "4 KB"]
    FLASH_PP_MAINSS_4KB,
    #[doc = "8 KB"]
    FLASH_PP_MAINSS_8KB,
    #[doc = "16 KB"]
    FLASH_PP_MAINSS_16KB,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl FLASH_PP_MAINSSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_PP_MAINSSR::FLASH_PP_MAINSS_1KB => 0,
            FLASH_PP_MAINSSR::FLASH_PP_MAINSS_2KB => 1,
            FLASH_PP_MAINSSR::FLASH_PP_MAINSS_4KB => 2,
            FLASH_PP_MAINSSR::FLASH_PP_MAINSS_8KB => 3,
            FLASH_PP_MAINSSR::FLASH_PP_MAINSS_16KB => 4,
            FLASH_PP_MAINSSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_PP_MAINSSR {
        match value {
            0 => FLASH_PP_MAINSSR::FLASH_PP_MAINSS_1KB,
            1 => FLASH_PP_MAINSSR::FLASH_PP_MAINSS_2KB,
            2 => FLASH_PP_MAINSSR::FLASH_PP_MAINSS_4KB,
            3 => FLASH_PP_MAINSSR::FLASH_PP_MAINSS_8KB,
            4 => FLASH_PP_MAINSSR::FLASH_PP_MAINSS_16KB,
            i => FLASH_PP_MAINSSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_MAINSS_1KB`"]
    #[inline(always)]
    pub fn is_flash_pp_mainss_1kb(&self) -> bool {
        *self == FLASH_PP_MAINSSR::FLASH_PP_MAINSS_1KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_MAINSS_2KB`"]
    #[inline(always)]
    pub fn is_flash_pp_mainss_2kb(&self) -> bool {
        *self == FLASH_PP_MAINSSR::FLASH_PP_MAINSS_2KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_MAINSS_4KB`"]
    #[inline(always)]
    pub fn is_flash_pp_mainss_4kb(&self) -> bool {
        *self == FLASH_PP_MAINSSR::FLASH_PP_MAINSS_4KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_MAINSS_8KB`"]
    #[inline(always)]
    pub fn is_flash_pp_mainss_8kb(&self) -> bool {
        *self == FLASH_PP_MAINSSR::FLASH_PP_MAINSS_8KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_MAINSS_16KB`"]
    #[inline(always)]
    pub fn is_flash_pp_mainss_16kb(&self) -> bool {
        *self == FLASH_PP_MAINSSR::FLASH_PP_MAINSS_16KB
    }
}
#[doc = "Values that can be written to the field `FLASH_PP_MAINSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PP_MAINSSW {
    #[doc = "1 KB"]
    FLASH_PP_MAINSS_1KB,
    #[doc = "2 KB"]
    FLASH_PP_MAINSS_2KB,
    #[doc = "4 KB"]
    FLASH_PP_MAINSS_4KB,
    #[doc = "8 KB"]
    FLASH_PP_MAINSS_8KB,
    #[doc = "16 KB"]
    FLASH_PP_MAINSS_16KB,
}
impl FLASH_PP_MAINSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_PP_MAINSSW::FLASH_PP_MAINSS_1KB => 0,
            FLASH_PP_MAINSSW::FLASH_PP_MAINSS_2KB => 1,
            FLASH_PP_MAINSSW::FLASH_PP_MAINSS_4KB => 2,
            FLASH_PP_MAINSSW::FLASH_PP_MAINSS_8KB => 3,
            FLASH_PP_MAINSSW::FLASH_PP_MAINSS_16KB => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_PP_MAINSSW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_PP_MAINSSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_PP_MAINSSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 KB"]
    #[inline(always)]
    pub fn flash_pp_mainss_1kb(self) -> &'a mut W {
        self.variant(FLASH_PP_MAINSSW::FLASH_PP_MAINSS_1KB)
    }
    #[doc = "2 KB"]
    #[inline(always)]
    pub fn flash_pp_mainss_2kb(self) -> &'a mut W {
        self.variant(FLASH_PP_MAINSSW::FLASH_PP_MAINSS_2KB)
    }
    #[doc = "4 KB"]
    #[inline(always)]
    pub fn flash_pp_mainss_4kb(self) -> &'a mut W {
        self.variant(FLASH_PP_MAINSSW::FLASH_PP_MAINSS_4KB)
    }
    #[doc = "8 KB"]
    #[inline(always)]
    pub fn flash_pp_mainss_8kb(self) -> &'a mut W {
        self.variant(FLASH_PP_MAINSSW::FLASH_PP_MAINSS_8KB)
    }
    #[doc = "16 KB"]
    #[inline(always)]
    pub fn flash_pp_mainss_16kb(self) -> &'a mut W {
        self.variant(FLASH_PP_MAINSSW::FLASH_PP_MAINSS_16KB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 16);
        self.w.bits |= ((value as u32) & 7) << 16;
        self.w
    }
}
#[doc = "Possible values of the field `FLASH_PP_EESS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PP_EESSR {
    #[doc = "1 KB"]
    FLASH_PP_EESS_1KB,
    #[doc = "2 KB"]
    FLASH_PP_EESS_2KB,
    #[doc = "4 KB"]
    FLASH_PP_EESS_4KB,
    #[doc = "8 KB"]
    FLASH_PP_EESS_8KB,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl FLASH_PP_EESSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_PP_EESSR::FLASH_PP_EESS_1KB => 0,
            FLASH_PP_EESSR::FLASH_PP_EESS_2KB => 1,
            FLASH_PP_EESSR::FLASH_PP_EESS_4KB => 2,
            FLASH_PP_EESSR::FLASH_PP_EESS_8KB => 3,
            FLASH_PP_EESSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_PP_EESSR {
        match value {
            0 => FLASH_PP_EESSR::FLASH_PP_EESS_1KB,
            1 => FLASH_PP_EESSR::FLASH_PP_EESS_2KB,
            2 => FLASH_PP_EESSR::FLASH_PP_EESS_4KB,
            3 => FLASH_PP_EESSR::FLASH_PP_EESS_8KB,
            i => FLASH_PP_EESSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_EESS_1KB`"]
    #[inline(always)]
    pub fn is_flash_pp_eess_1kb(&self) -> bool {
        *self == FLASH_PP_EESSR::FLASH_PP_EESS_1KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_EESS_2KB`"]
    #[inline(always)]
    pub fn is_flash_pp_eess_2kb(&self) -> bool {
        *self == FLASH_PP_EESSR::FLASH_PP_EESS_2KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_EESS_4KB`"]
    #[inline(always)]
    pub fn is_flash_pp_eess_4kb(&self) -> bool {
        *self == FLASH_PP_EESSR::FLASH_PP_EESS_4KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_EESS_8KB`"]
    #[inline(always)]
    pub fn is_flash_pp_eess_8kb(&self) -> bool {
        *self == FLASH_PP_EESSR::FLASH_PP_EESS_8KB
    }
}
#[doc = "Values that can be written to the field `FLASH_PP_EESS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PP_EESSW {
    #[doc = "1 KB"]
    FLASH_PP_EESS_1KB,
    #[doc = "2 KB"]
    FLASH_PP_EESS_2KB,
    #[doc = "4 KB"]
    FLASH_PP_EESS_4KB,
    #[doc = "8 KB"]
    FLASH_PP_EESS_8KB,
}
impl FLASH_PP_EESSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_PP_EESSW::FLASH_PP_EESS_1KB => 0,
            FLASH_PP_EESSW::FLASH_PP_EESS_2KB => 1,
            FLASH_PP_EESSW::FLASH_PP_EESS_4KB => 2,
            FLASH_PP_EESSW::FLASH_PP_EESS_8KB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_PP_EESSW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_PP_EESSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_PP_EESSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 KB"]
    #[inline(always)]
    pub fn flash_pp_eess_1kb(self) -> &'a mut W {
        self.variant(FLASH_PP_EESSW::FLASH_PP_EESS_1KB)
    }
    #[doc = "2 KB"]
    #[inline(always)]
    pub fn flash_pp_eess_2kb(self) -> &'a mut W {
        self.variant(FLASH_PP_EESSW::FLASH_PP_EESS_2KB)
    }
    #[doc = "4 KB"]
    #[inline(always)]
    pub fn flash_pp_eess_4kb(self) -> &'a mut W {
        self.variant(FLASH_PP_EESSW::FLASH_PP_EESS_4KB)
    }
    #[doc = "8 KB"]
    #[inline(always)]
    pub fn flash_pp_eess_8kb(self) -> &'a mut W {
        self.variant(FLASH_PP_EESSW::FLASH_PP_EESS_8KB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 19);
        self.w.bits |= ((value as u32) & 15) << 19;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct FLASH_PP_DFAR {
    bits: bool,
}
impl FLASH_PP_DFAR {
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
pub struct _FLASH_PP_DFAW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_PP_DFAW<'a> {
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
pub struct FLASH_PP_FMMR {
    bits: bool,
}
impl FLASH_PP_FMMR {
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
pub struct _FLASH_PP_FMMW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_PP_FMMW<'a> {
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
pub struct FLASH_PP_PFCR {
    bits: bool,
}
impl FLASH_PP_PFCR {
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
pub struct _FLASH_PP_PFCW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_PP_PFCW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn flash_pp_size(&self) -> FLASH_PP_SIZER {
        let bits = ((self.bits >> 0) & 65535) as u16;
        FLASH_PP_SIZER { bits }
    }
    #[doc = "Bits 16:18 - Flash Sector Size of the physical bank"]
    #[inline(always)]
    pub fn flash_pp_mainss(&self) -> FLASH_PP_MAINSSR {
        FLASH_PP_MAINSSR::_from(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - EEPROM Sector Size of the physical bank"]
    #[inline(always)]
    pub fn flash_pp_eess(&self) -> FLASH_PP_EESSR {
        FLASH_PP_EESSR::_from(((self.bits >> 19) & 15) as u8)
    }
    #[doc = "Bit 28 - DMA Flash Access"]
    #[inline(always)]
    pub fn flash_pp_dfa(&self) -> FLASH_PP_DFAR {
        let bits = ((self.bits >> 28) & 1) != 0;
        FLASH_PP_DFAR { bits }
    }
    #[doc = "Bit 29 - Flash Mirror Mode"]
    #[inline(always)]
    pub fn flash_pp_fmm(&self) -> FLASH_PP_FMMR {
        let bits = ((self.bits >> 29) & 1) != 0;
        FLASH_PP_FMMR { bits }
    }
    #[doc = "Bit 30 - Prefetch Buffer Mode"]
    #[inline(always)]
    pub fn flash_pp_pfc(&self) -> FLASH_PP_PFCR {
        let bits = ((self.bits >> 30) & 1) != 0;
        FLASH_PP_PFCR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn flash_pp_size(&mut self) -> _FLASH_PP_SIZEW {
        _FLASH_PP_SIZEW { w: self }
    }
    #[doc = "Bits 16:18 - Flash Sector Size of the physical bank"]
    #[inline(always)]
    pub fn flash_pp_mainss(&mut self) -> _FLASH_PP_MAINSSW {
        _FLASH_PP_MAINSSW { w: self }
    }
    #[doc = "Bits 19:22 - EEPROM Sector Size of the physical bank"]
    #[inline(always)]
    pub fn flash_pp_eess(&mut self) -> _FLASH_PP_EESSW {
        _FLASH_PP_EESSW { w: self }
    }
    #[doc = "Bit 28 - DMA Flash Access"]
    #[inline(always)]
    pub fn flash_pp_dfa(&mut self) -> _FLASH_PP_DFAW {
        _FLASH_PP_DFAW { w: self }
    }
    #[doc = "Bit 29 - Flash Mirror Mode"]
    #[inline(always)]
    pub fn flash_pp_fmm(&mut self) -> _FLASH_PP_FMMW {
        _FLASH_PP_FMMW { w: self }
    }
    #[doc = "Bit 30 - Prefetch Buffer Mode"]
    #[inline(always)]
    pub fn flash_pp_pfc(&mut self) -> _FLASH_PP_PFCW {
        _FLASH_PP_PFCW { w: self }
    }
}
