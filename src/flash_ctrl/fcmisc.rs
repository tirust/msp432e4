#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCMISC {
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
pub struct FLASH_FCMISC_AMISCR {
    bits: bool,
}
impl FLASH_FCMISC_AMISCR {
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
pub struct _FLASH_FCMISC_AMISCW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCMISC_AMISCW<'a> {
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
pub struct FLASH_FCMISC_PMISCR {
    bits: bool,
}
impl FLASH_FCMISC_PMISCR {
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
pub struct _FLASH_FCMISC_PMISCW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCMISC_PMISCW<'a> {
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
pub struct FLASH_FCMISC_EMISCR {
    bits: bool,
}
impl FLASH_FCMISC_EMISCR {
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
pub struct _FLASH_FCMISC_EMISCW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCMISC_EMISCW<'a> {
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
pub struct FLASH_FCMISC_VOLTMISCR {
    bits: bool,
}
impl FLASH_FCMISC_VOLTMISCR {
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
pub struct _FLASH_FCMISC_VOLTMISCW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCMISC_VOLTMISCW<'a> {
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
pub struct FLASH_FCMISC_INVDMISCR {
    bits: bool,
}
impl FLASH_FCMISC_INVDMISCR {
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
pub struct _FLASH_FCMISC_INVDMISCW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCMISC_INVDMISCW<'a> {
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
pub struct FLASH_FCMISC_ERMISCR {
    bits: bool,
}
impl FLASH_FCMISC_ERMISCR {
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
pub struct _FLASH_FCMISC_ERMISCW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCMISC_ERMISCW<'a> {
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
pub struct FLASH_FCMISC_PROGMISCR {
    bits: bool,
}
impl FLASH_FCMISC_PROGMISCR {
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
pub struct _FLASH_FCMISC_PROGMISCW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCMISC_PROGMISCW<'a> {
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
    #[doc = "Bit 0 - Access Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_amisc(&self) -> FLASH_FCMISC_AMISCR {
        let bits = ((self.bits >> 0) & 1) != 0;
        FLASH_FCMISC_AMISCR { bits }
    }
    #[doc = "Bit 1 - Programming Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_pmisc(&self) -> FLASH_FCMISC_PMISCR {
        let bits = ((self.bits >> 1) & 1) != 0;
        FLASH_FCMISC_PMISCR { bits }
    }
    #[doc = "Bit 2 - EEPROM Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_emisc(&self) -> FLASH_FCMISC_EMISCR {
        let bits = ((self.bits >> 2) & 1) != 0;
        FLASH_FCMISC_EMISCR { bits }
    }
    #[doc = "Bit 9 - VOLT Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_voltmisc(&self) -> FLASH_FCMISC_VOLTMISCR {
        let bits = ((self.bits >> 9) & 1) != 0;
        FLASH_FCMISC_VOLTMISCR { bits }
    }
    #[doc = "Bit 10 - Invalid Data Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_invdmisc(&self) -> FLASH_FCMISC_INVDMISCR {
        let bits = ((self.bits >> 10) & 1) != 0;
        FLASH_FCMISC_INVDMISCR { bits }
    }
    #[doc = "Bit 11 - ERVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_ermisc(&self) -> FLASH_FCMISC_ERMISCR {
        let bits = ((self.bits >> 11) & 1) != 0;
        FLASH_FCMISC_ERMISCR { bits }
    }
    #[doc = "Bit 13 - PROGVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_progmisc(&self) -> FLASH_FCMISC_PROGMISCR {
        let bits = ((self.bits >> 13) & 1) != 0;
        FLASH_FCMISC_PROGMISCR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Access Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_amisc(&mut self) -> _FLASH_FCMISC_AMISCW {
        _FLASH_FCMISC_AMISCW { w: self }
    }
    #[doc = "Bit 1 - Programming Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_pmisc(&mut self) -> _FLASH_FCMISC_PMISCW {
        _FLASH_FCMISC_PMISCW { w: self }
    }
    #[doc = "Bit 2 - EEPROM Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_emisc(&mut self) -> _FLASH_FCMISC_EMISCW {
        _FLASH_FCMISC_EMISCW { w: self }
    }
    #[doc = "Bit 9 - VOLT Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_voltmisc(&mut self) -> _FLASH_FCMISC_VOLTMISCW {
        _FLASH_FCMISC_VOLTMISCW { w: self }
    }
    #[doc = "Bit 10 - Invalid Data Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_invdmisc(&mut self) -> _FLASH_FCMISC_INVDMISCW {
        _FLASH_FCMISC_INVDMISCW { w: self }
    }
    #[doc = "Bit 11 - ERVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_ermisc(&mut self) -> _FLASH_FCMISC_ERMISCW {
        _FLASH_FCMISC_ERMISCW { w: self }
    }
    #[doc = "Bit 13 - PROGVER Masked Interrupt Status and Clear"]
    #[inline(always)]
    pub fn flash_fcmisc_progmisc(&mut self) -> _FLASH_FCMISC_PROGMISCW {
        _FLASH_FCMISC_PROGMISCW { w: self }
    }
}
