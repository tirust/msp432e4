#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEDONE {
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
pub struct EEPROM_EEDONE_WORKINGR {
    bits: bool,
}
impl EEPROM_EEDONE_WORKINGR {
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
pub struct _EEPROM_EEDONE_WORKINGW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_EEDONE_WORKINGW<'a> {
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
pub struct EEPROM_EEDONE_WKERASER {
    bits: bool,
}
impl EEPROM_EEDONE_WKERASER {
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
pub struct _EEPROM_EEDONE_WKERASEW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_EEDONE_WKERASEW<'a> {
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
pub struct EEPROM_EEDONE_WKCOPYR {
    bits: bool,
}
impl EEPROM_EEDONE_WKCOPYR {
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
pub struct _EEPROM_EEDONE_WKCOPYW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_EEDONE_WKCOPYW<'a> {
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
pub struct EEPROM_EEDONE_NOPERMR {
    bits: bool,
}
impl EEPROM_EEDONE_NOPERMR {
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
pub struct _EEPROM_EEDONE_NOPERMW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_EEDONE_NOPERMW<'a> {
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
pub struct EEPROM_EEDONE_WRBUSYR {
    bits: bool,
}
impl EEPROM_EEDONE_WRBUSYR {
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
pub struct _EEPROM_EEDONE_WRBUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_EEDONE_WRBUSYW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - EEPROM Working"]
    #[inline(always)]
    pub fn eeprom_eedone_working(&self) -> EEPROM_EEDONE_WORKINGR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EEPROM_EEDONE_WORKINGR { bits }
    }
    #[doc = "Bit 2 - Working on an Erase"]
    #[inline(always)]
    pub fn eeprom_eedone_wkerase(&self) -> EEPROM_EEDONE_WKERASER {
        let bits = ((self.bits >> 2) & 1) != 0;
        EEPROM_EEDONE_WKERASER { bits }
    }
    #[doc = "Bit 3 - Working on a Copy"]
    #[inline(always)]
    pub fn eeprom_eedone_wkcopy(&self) -> EEPROM_EEDONE_WKCOPYR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EEPROM_EEDONE_WKCOPYR { bits }
    }
    #[doc = "Bit 4 - Write Without Permission"]
    #[inline(always)]
    pub fn eeprom_eedone_noperm(&self) -> EEPROM_EEDONE_NOPERMR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EEPROM_EEDONE_NOPERMR { bits }
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn eeprom_eedone_wrbusy(&self) -> EEPROM_EEDONE_WRBUSYR {
        let bits = ((self.bits >> 5) & 1) != 0;
        EEPROM_EEDONE_WRBUSYR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - EEPROM Working"]
    #[inline(always)]
    pub fn eeprom_eedone_working(&mut self) -> _EEPROM_EEDONE_WORKINGW {
        _EEPROM_EEDONE_WORKINGW { w: self }
    }
    #[doc = "Bit 2 - Working on an Erase"]
    #[inline(always)]
    pub fn eeprom_eedone_wkerase(&mut self) -> _EEPROM_EEDONE_WKERASEW {
        _EEPROM_EEDONE_WKERASEW { w: self }
    }
    #[doc = "Bit 3 - Working on a Copy"]
    #[inline(always)]
    pub fn eeprom_eedone_wkcopy(&mut self) -> _EEPROM_EEDONE_WKCOPYW {
        _EEPROM_EEDONE_WKCOPYW { w: self }
    }
    #[doc = "Bit 4 - Write Without Permission"]
    #[inline(always)]
    pub fn eeprom_eedone_noperm(&mut self) -> _EEPROM_EEDONE_NOPERMW {
        _EEPROM_EEDONE_NOPERMW { w: self }
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn eeprom_eedone_wrbusy(&mut self) -> _EEPROM_EEDONE_WRBUSYW {
        _EEPROM_EEDONE_WRBUSYW { w: self }
    }
}
