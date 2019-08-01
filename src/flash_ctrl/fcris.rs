#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCRIS {
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
pub struct FLASH_FCRIS_ARISR {
    bits: bool,
}
impl FLASH_FCRIS_ARISR {
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
pub struct _FLASH_FCRIS_ARISW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCRIS_ARISW<'a> {
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
pub struct FLASH_FCRIS_PRISR {
    bits: bool,
}
impl FLASH_FCRIS_PRISR {
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
pub struct _FLASH_FCRIS_PRISW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCRIS_PRISW<'a> {
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
pub struct FLASH_FCRIS_ERISR {
    bits: bool,
}
impl FLASH_FCRIS_ERISR {
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
pub struct _FLASH_FCRIS_ERISW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCRIS_ERISW<'a> {
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
pub struct FLASH_FCRIS_VOLTRISR {
    bits: bool,
}
impl FLASH_FCRIS_VOLTRISR {
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
pub struct _FLASH_FCRIS_VOLTRISW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCRIS_VOLTRISW<'a> {
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
pub struct FLASH_FCRIS_INVDRISR {
    bits: bool,
}
impl FLASH_FCRIS_INVDRISR {
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
pub struct _FLASH_FCRIS_INVDRISW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCRIS_INVDRISW<'a> {
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
pub struct FLASH_FCRIS_ERRISR {
    bits: bool,
}
impl FLASH_FCRIS_ERRISR {
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
pub struct _FLASH_FCRIS_ERRISW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCRIS_ERRISW<'a> {
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
pub struct FLASH_FCRIS_PROGRISR {
    bits: bool,
}
impl FLASH_FCRIS_PROGRISR {
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
pub struct _FLASH_FCRIS_PROGRISW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCRIS_PROGRISW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Access Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_aris(&self) -> FLASH_FCRIS_ARISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        FLASH_FCRIS_ARISR { bits }
    }
    #[doc = "Bit 1 - Programming Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_pris(&self) -> FLASH_FCRIS_PRISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        FLASH_FCRIS_PRISR { bits }
    }
    #[doc = "Bit 2 - EEPROM Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_eris(&self) -> FLASH_FCRIS_ERISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        FLASH_FCRIS_ERISR { bits }
    }
    #[doc = "Bit 9 - Pump Voltage Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_voltris(&self) -> FLASH_FCRIS_VOLTRISR {
        let bits = ((self.bits >> 9) & 1) != 0;
        FLASH_FCRIS_VOLTRISR { bits }
    }
    #[doc = "Bit 10 - Invalid Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_invdris(&self) -> FLASH_FCRIS_INVDRISR {
        let bits = ((self.bits >> 10) & 1) != 0;
        FLASH_FCRIS_INVDRISR { bits }
    }
    #[doc = "Bit 11 - Erase Verify Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_erris(&self) -> FLASH_FCRIS_ERRISR {
        let bits = ((self.bits >> 11) & 1) != 0;
        FLASH_FCRIS_ERRISR { bits }
    }
    #[doc = "Bit 13 - Program Verify Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_progris(&self) -> FLASH_FCRIS_PROGRISR {
        let bits = ((self.bits >> 13) & 1) != 0;
        FLASH_FCRIS_PROGRISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Access Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_aris(&mut self) -> _FLASH_FCRIS_ARISW {
        _FLASH_FCRIS_ARISW { w: self }
    }
    #[doc = "Bit 1 - Programming Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_pris(&mut self) -> _FLASH_FCRIS_PRISW {
        _FLASH_FCRIS_PRISW { w: self }
    }
    #[doc = "Bit 2 - EEPROM Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_eris(&mut self) -> _FLASH_FCRIS_ERISW {
        _FLASH_FCRIS_ERISW { w: self }
    }
    #[doc = "Bit 9 - Pump Voltage Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_voltris(&mut self) -> _FLASH_FCRIS_VOLTRISW {
        _FLASH_FCRIS_VOLTRISW { w: self }
    }
    #[doc = "Bit 10 - Invalid Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_invdris(&mut self) -> _FLASH_FCRIS_INVDRISW {
        _FLASH_FCRIS_INVDRISW { w: self }
    }
    #[doc = "Bit 11 - Erase Verify Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_erris(&mut self) -> _FLASH_FCRIS_ERRISW {
        _FLASH_FCRIS_ERRISW { w: self }
    }
    #[doc = "Bit 13 - Program Verify Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_progris(&mut self) -> _FLASH_FCRIS_PROGRISW {
        _FLASH_FCRIS_PROGRISW { w: self }
    }
}
