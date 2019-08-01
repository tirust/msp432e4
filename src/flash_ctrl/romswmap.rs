#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROMSWMAP {
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
#[doc = "Possible values of the field `FLASH_ROMSWMAP_SW0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW0ENR {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW0EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW0EN_CORE,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl FLASH_ROMSWMAP_SW0ENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW0ENR::FLASH_ROMSWMAP_SW0EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW0ENR::FLASH_ROMSWMAP_SW0EN_CORE => 1,
            FLASH_ROMSWMAP_SW0ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_ROMSWMAP_SW0ENR {
        match value {
            0 => FLASH_ROMSWMAP_SW0ENR::FLASH_ROMSWMAP_SW0EN_NOTVIS,
            1 => FLASH_ROMSWMAP_SW0ENR::FLASH_ROMSWMAP_SW0EN_CORE,
            i => FLASH_ROMSWMAP_SW0ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW0EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw0en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW0ENR::FLASH_ROMSWMAP_SW0EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW0EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw0en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW0ENR::FLASH_ROMSWMAP_SW0EN_CORE
    }
}
#[doc = "Values that can be written to the field `FLASH_ROMSWMAP_SW0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW0ENW {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW0EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW0EN_CORE,
}
impl FLASH_ROMSWMAP_SW0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW0ENW::FLASH_ROMSWMAP_SW0EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW0ENW::FLASH_ROMSWMAP_SW0EN_CORE => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_ROMSWMAP_SW0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_ROMSWMAP_SW0ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_ROMSWMAP_SW0ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw0en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW0ENW::FLASH_ROMSWMAP_SW0EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw0en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW0ENW::FLASH_ROMSWMAP_SW0EN_CORE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `FLASH_ROMSWMAP_SW1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW1ENR {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW1EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW1EN_CORE,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl FLASH_ROMSWMAP_SW1ENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW1ENR::FLASH_ROMSWMAP_SW1EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW1ENR::FLASH_ROMSWMAP_SW1EN_CORE => 1,
            FLASH_ROMSWMAP_SW1ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_ROMSWMAP_SW1ENR {
        match value {
            0 => FLASH_ROMSWMAP_SW1ENR::FLASH_ROMSWMAP_SW1EN_NOTVIS,
            1 => FLASH_ROMSWMAP_SW1ENR::FLASH_ROMSWMAP_SW1EN_CORE,
            i => FLASH_ROMSWMAP_SW1ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW1EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw1en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW1ENR::FLASH_ROMSWMAP_SW1EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW1EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw1en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW1ENR::FLASH_ROMSWMAP_SW1EN_CORE
    }
}
#[doc = "Values that can be written to the field `FLASH_ROMSWMAP_SW1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW1ENW {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW1EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW1EN_CORE,
}
impl FLASH_ROMSWMAP_SW1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW1ENW::FLASH_ROMSWMAP_SW1EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW1ENW::FLASH_ROMSWMAP_SW1EN_CORE => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_ROMSWMAP_SW1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_ROMSWMAP_SW1ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_ROMSWMAP_SW1ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw1en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW1ENW::FLASH_ROMSWMAP_SW1EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw1en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW1ENW::FLASH_ROMSWMAP_SW1EN_CORE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
#[doc = "Possible values of the field `FLASH_ROMSWMAP_SW2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW2ENR {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW2EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW2EN_CORE,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl FLASH_ROMSWMAP_SW2ENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW2ENR::FLASH_ROMSWMAP_SW2EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW2ENR::FLASH_ROMSWMAP_SW2EN_CORE => 1,
            FLASH_ROMSWMAP_SW2ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_ROMSWMAP_SW2ENR {
        match value {
            0 => FLASH_ROMSWMAP_SW2ENR::FLASH_ROMSWMAP_SW2EN_NOTVIS,
            1 => FLASH_ROMSWMAP_SW2ENR::FLASH_ROMSWMAP_SW2EN_CORE,
            i => FLASH_ROMSWMAP_SW2ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW2EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw2en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW2ENR::FLASH_ROMSWMAP_SW2EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW2EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw2en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW2ENR::FLASH_ROMSWMAP_SW2EN_CORE
    }
}
#[doc = "Values that can be written to the field `FLASH_ROMSWMAP_SW2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW2ENW {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW2EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW2EN_CORE,
}
impl FLASH_ROMSWMAP_SW2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW2ENW::FLASH_ROMSWMAP_SW2EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW2ENW::FLASH_ROMSWMAP_SW2EN_CORE => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_ROMSWMAP_SW2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_ROMSWMAP_SW2ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_ROMSWMAP_SW2ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw2en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW2ENW::FLASH_ROMSWMAP_SW2EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw2en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW2ENW::FLASH_ROMSWMAP_SW2EN_CORE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `FLASH_ROMSWMAP_SW3EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW3ENR {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW3EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW3EN_CORE,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl FLASH_ROMSWMAP_SW3ENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW3ENR::FLASH_ROMSWMAP_SW3EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW3ENR::FLASH_ROMSWMAP_SW3EN_CORE => 1,
            FLASH_ROMSWMAP_SW3ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_ROMSWMAP_SW3ENR {
        match value {
            0 => FLASH_ROMSWMAP_SW3ENR::FLASH_ROMSWMAP_SW3EN_NOTVIS,
            1 => FLASH_ROMSWMAP_SW3ENR::FLASH_ROMSWMAP_SW3EN_CORE,
            i => FLASH_ROMSWMAP_SW3ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW3EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw3en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW3ENR::FLASH_ROMSWMAP_SW3EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW3EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw3en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW3ENR::FLASH_ROMSWMAP_SW3EN_CORE
    }
}
#[doc = "Values that can be written to the field `FLASH_ROMSWMAP_SW3EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW3ENW {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW3EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW3EN_CORE,
}
impl FLASH_ROMSWMAP_SW3ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW3ENW::FLASH_ROMSWMAP_SW3EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW3ENW::FLASH_ROMSWMAP_SW3EN_CORE => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_ROMSWMAP_SW3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_ROMSWMAP_SW3ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_ROMSWMAP_SW3ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw3en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW3ENW::FLASH_ROMSWMAP_SW3EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw3en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW3ENW::FLASH_ROMSWMAP_SW3EN_CORE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = "Possible values of the field `FLASH_ROMSWMAP_SW4EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW4ENR {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW4EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW4EN_CORE,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl FLASH_ROMSWMAP_SW4ENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW4ENR::FLASH_ROMSWMAP_SW4EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW4ENR::FLASH_ROMSWMAP_SW4EN_CORE => 1,
            FLASH_ROMSWMAP_SW4ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_ROMSWMAP_SW4ENR {
        match value {
            0 => FLASH_ROMSWMAP_SW4ENR::FLASH_ROMSWMAP_SW4EN_NOTVIS,
            1 => FLASH_ROMSWMAP_SW4ENR::FLASH_ROMSWMAP_SW4EN_CORE,
            i => FLASH_ROMSWMAP_SW4ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW4EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw4en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW4ENR::FLASH_ROMSWMAP_SW4EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW4EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw4en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW4ENR::FLASH_ROMSWMAP_SW4EN_CORE
    }
}
#[doc = "Values that can be written to the field `FLASH_ROMSWMAP_SW4EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW4ENW {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW4EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW4EN_CORE,
}
impl FLASH_ROMSWMAP_SW4ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW4ENW::FLASH_ROMSWMAP_SW4EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW4ENW::FLASH_ROMSWMAP_SW4EN_CORE => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_ROMSWMAP_SW4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_ROMSWMAP_SW4ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_ROMSWMAP_SW4ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw4en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW4ENW::FLASH_ROMSWMAP_SW4EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw4en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW4ENW::FLASH_ROMSWMAP_SW4EN_CORE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = "Possible values of the field `FLASH_ROMSWMAP_SW5EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW5ENR {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW5EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW5EN_CORE,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl FLASH_ROMSWMAP_SW5ENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW5ENR::FLASH_ROMSWMAP_SW5EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW5ENR::FLASH_ROMSWMAP_SW5EN_CORE => 1,
            FLASH_ROMSWMAP_SW5ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_ROMSWMAP_SW5ENR {
        match value {
            0 => FLASH_ROMSWMAP_SW5ENR::FLASH_ROMSWMAP_SW5EN_NOTVIS,
            1 => FLASH_ROMSWMAP_SW5ENR::FLASH_ROMSWMAP_SW5EN_CORE,
            i => FLASH_ROMSWMAP_SW5ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW5EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw5en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW5ENR::FLASH_ROMSWMAP_SW5EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW5EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw5en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW5ENR::FLASH_ROMSWMAP_SW5EN_CORE
    }
}
#[doc = "Values that can be written to the field `FLASH_ROMSWMAP_SW5EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW5ENW {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW5EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW5EN_CORE,
}
impl FLASH_ROMSWMAP_SW5ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW5ENW::FLASH_ROMSWMAP_SW5EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW5ENW::FLASH_ROMSWMAP_SW5EN_CORE => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_ROMSWMAP_SW5ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_ROMSWMAP_SW5ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_ROMSWMAP_SW5ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw5en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW5ENW::FLASH_ROMSWMAP_SW5EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw5en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW5ENW::FLASH_ROMSWMAP_SW5EN_CORE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 10);
        self.w.bits |= ((value as u32) & 3) << 10;
        self.w
    }
}
#[doc = "Possible values of the field `FLASH_ROMSWMAP_SW6EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW6ENR {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW6EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW6EN_CORE,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl FLASH_ROMSWMAP_SW6ENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW6ENR::FLASH_ROMSWMAP_SW6EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW6ENR::FLASH_ROMSWMAP_SW6EN_CORE => 1,
            FLASH_ROMSWMAP_SW6ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_ROMSWMAP_SW6ENR {
        match value {
            0 => FLASH_ROMSWMAP_SW6ENR::FLASH_ROMSWMAP_SW6EN_NOTVIS,
            1 => FLASH_ROMSWMAP_SW6ENR::FLASH_ROMSWMAP_SW6EN_CORE,
            i => FLASH_ROMSWMAP_SW6ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW6EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw6en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW6ENR::FLASH_ROMSWMAP_SW6EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW6EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw6en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW6ENR::FLASH_ROMSWMAP_SW6EN_CORE
    }
}
#[doc = "Values that can be written to the field `FLASH_ROMSWMAP_SW6EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW6ENW {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW6EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW6EN_CORE,
}
impl FLASH_ROMSWMAP_SW6ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW6ENW::FLASH_ROMSWMAP_SW6EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW6ENW::FLASH_ROMSWMAP_SW6EN_CORE => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_ROMSWMAP_SW6ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_ROMSWMAP_SW6ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_ROMSWMAP_SW6ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw6en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW6ENW::FLASH_ROMSWMAP_SW6EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw6en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW6ENW::FLASH_ROMSWMAP_SW6EN_CORE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 12);
        self.w.bits |= ((value as u32) & 3) << 12;
        self.w
    }
}
#[doc = "Possible values of the field `FLASH_ROMSWMAP_SW7EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW7ENR {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW7EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW7EN_CORE,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl FLASH_ROMSWMAP_SW7ENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW7ENR::FLASH_ROMSWMAP_SW7EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW7ENR::FLASH_ROMSWMAP_SW7EN_CORE => 1,
            FLASH_ROMSWMAP_SW7ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_ROMSWMAP_SW7ENR {
        match value {
            0 => FLASH_ROMSWMAP_SW7ENR::FLASH_ROMSWMAP_SW7EN_NOTVIS,
            1 => FLASH_ROMSWMAP_SW7ENR::FLASH_ROMSWMAP_SW7EN_CORE,
            i => FLASH_ROMSWMAP_SW7ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW7EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw7en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW7ENR::FLASH_ROMSWMAP_SW7EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW7EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw7en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW7ENR::FLASH_ROMSWMAP_SW7EN_CORE
    }
}
#[doc = "Values that can be written to the field `FLASH_ROMSWMAP_SW7EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ROMSWMAP_SW7ENW {
    #[doc = "Software region not available to the core"]
    FLASH_ROMSWMAP_SW7EN_NOTVIS,
    #[doc = "Region available to core"]
    FLASH_ROMSWMAP_SW7EN_CORE,
}
impl FLASH_ROMSWMAP_SW7ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_ROMSWMAP_SW7ENW::FLASH_ROMSWMAP_SW7EN_NOTVIS => 0,
            FLASH_ROMSWMAP_SW7ENW::FLASH_ROMSWMAP_SW7EN_CORE => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_ROMSWMAP_SW7ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_ROMSWMAP_SW7ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_ROMSWMAP_SW7ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw7en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW7ENW::FLASH_ROMSWMAP_SW7EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw7en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW7ENW::FLASH_ROMSWMAP_SW7EN_CORE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 14);
        self.w.bits |= ((value as u32) & 3) << 14;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - ROM SW Region 0 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw0en(&self) -> FLASH_ROMSWMAP_SW0ENR {
        FLASH_ROMSWMAP_SW0ENR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 2:3 - ROM SW Region 1 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw1en(&self) -> FLASH_ROMSWMAP_SW1ENR {
        FLASH_ROMSWMAP_SW1ENR::_from(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ROM SW Region 2 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw2en(&self) -> FLASH_ROMSWMAP_SW2ENR {
        FLASH_ROMSWMAP_SW2ENR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - ROM SW Region 3 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw3en(&self) -> FLASH_ROMSWMAP_SW3ENR {
        FLASH_ROMSWMAP_SW3ENR::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ROM SW Region 4 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw4en(&self) -> FLASH_ROMSWMAP_SW4ENR {
        FLASH_ROMSWMAP_SW4ENR::_from(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ROM SW Region 5 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw5en(&self) -> FLASH_ROMSWMAP_SW5ENR {
        FLASH_ROMSWMAP_SW5ENR::_from(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - ROM SW Region 6 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw6en(&self) -> FLASH_ROMSWMAP_SW6ENR {
        FLASH_ROMSWMAP_SW6ENR::_from(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - ROM SW Region 7 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw7en(&self) -> FLASH_ROMSWMAP_SW7ENR {
        FLASH_ROMSWMAP_SW7ENR::_from(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - ROM SW Region 0 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw0en(&mut self) -> _FLASH_ROMSWMAP_SW0ENW {
        _FLASH_ROMSWMAP_SW0ENW { w: self }
    }
    #[doc = "Bits 2:3 - ROM SW Region 1 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw1en(&mut self) -> _FLASH_ROMSWMAP_SW1ENW {
        _FLASH_ROMSWMAP_SW1ENW { w: self }
    }
    #[doc = "Bits 4:5 - ROM SW Region 2 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw2en(&mut self) -> _FLASH_ROMSWMAP_SW2ENW {
        _FLASH_ROMSWMAP_SW2ENW { w: self }
    }
    #[doc = "Bits 6:7 - ROM SW Region 3 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw3en(&mut self) -> _FLASH_ROMSWMAP_SW3ENW {
        _FLASH_ROMSWMAP_SW3ENW { w: self }
    }
    #[doc = "Bits 8:9 - ROM SW Region 4 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw4en(&mut self) -> _FLASH_ROMSWMAP_SW4ENW {
        _FLASH_ROMSWMAP_SW4ENW { w: self }
    }
    #[doc = "Bits 10:11 - ROM SW Region 5 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw5en(&mut self) -> _FLASH_ROMSWMAP_SW5ENW {
        _FLASH_ROMSWMAP_SW5ENW { w: self }
    }
    #[doc = "Bits 12:13 - ROM SW Region 6 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw6en(&mut self) -> _FLASH_ROMSWMAP_SW6ENW {
        _FLASH_ROMSWMAP_SW6ENW { w: self }
    }
    #[doc = "Bits 14:15 - ROM SW Region 7 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw7en(&mut self) -> _FLASH_ROMSWMAP_SW7ENW {
        _FLASH_ROMSWMAP_SW7ENW { w: self }
    }
}
