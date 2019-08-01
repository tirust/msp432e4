#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCIM {
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
pub struct FLASH_FCIM_AMASKR {
    bits: bool,
}
impl FLASH_FCIM_AMASKR {
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
pub struct _FLASH_FCIM_AMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCIM_AMASKW<'a> {
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
pub struct FLASH_FCIM_PMASKR {
    bits: bool,
}
impl FLASH_FCIM_PMASKR {
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
pub struct _FLASH_FCIM_PMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCIM_PMASKW<'a> {
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
pub struct FLASH_FCIM_EMASKR {
    bits: bool,
}
impl FLASH_FCIM_EMASKR {
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
pub struct _FLASH_FCIM_EMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCIM_EMASKW<'a> {
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
pub struct FLASH_FCIM_VOLTMASKR {
    bits: bool,
}
impl FLASH_FCIM_VOLTMASKR {
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
pub struct _FLASH_FCIM_VOLTMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCIM_VOLTMASKW<'a> {
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
pub struct FLASH_FCIM_INVDMASKR {
    bits: bool,
}
impl FLASH_FCIM_INVDMASKR {
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
pub struct _FLASH_FCIM_INVDMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCIM_INVDMASKW<'a> {
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
pub struct FLASH_FCIM_ERMASKR {
    bits: bool,
}
impl FLASH_FCIM_ERMASKR {
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
pub struct _FLASH_FCIM_ERMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCIM_ERMASKW<'a> {
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
pub struct FLASH_FCIM_PROGMASKR {
    bits: bool,
}
impl FLASH_FCIM_PROGMASKR {
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
pub struct _FLASH_FCIM_PROGMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FCIM_PROGMASKW<'a> {
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
    #[doc = "Bit 0 - Access Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_amask(&self) -> FLASH_FCIM_AMASKR {
        let bits = ((self.bits >> 0) & 1) != 0;
        FLASH_FCIM_AMASKR { bits }
    }
    #[doc = "Bit 1 - Programming Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_pmask(&self) -> FLASH_FCIM_PMASKR {
        let bits = ((self.bits >> 1) & 1) != 0;
        FLASH_FCIM_PMASKR { bits }
    }
    #[doc = "Bit 2 - EEPROM Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_emask(&self) -> FLASH_FCIM_EMASKR {
        let bits = ((self.bits >> 2) & 1) != 0;
        FLASH_FCIM_EMASKR { bits }
    }
    #[doc = "Bit 9 - VOLT Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_voltmask(&self) -> FLASH_FCIM_VOLTMASKR {
        let bits = ((self.bits >> 9) & 1) != 0;
        FLASH_FCIM_VOLTMASKR { bits }
    }
    #[doc = "Bit 10 - Invalid Data Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_invdmask(&self) -> FLASH_FCIM_INVDMASKR {
        let bits = ((self.bits >> 10) & 1) != 0;
        FLASH_FCIM_INVDMASKR { bits }
    }
    #[doc = "Bit 11 - ERVER Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_ermask(&self) -> FLASH_FCIM_ERMASKR {
        let bits = ((self.bits >> 11) & 1) != 0;
        FLASH_FCIM_ERMASKR { bits }
    }
    #[doc = "Bit 13 - PROGVER Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_progmask(&self) -> FLASH_FCIM_PROGMASKR {
        let bits = ((self.bits >> 13) & 1) != 0;
        FLASH_FCIM_PROGMASKR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Access Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_amask(&mut self) -> _FLASH_FCIM_AMASKW {
        _FLASH_FCIM_AMASKW { w: self }
    }
    #[doc = "Bit 1 - Programming Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_pmask(&mut self) -> _FLASH_FCIM_PMASKW {
        _FLASH_FCIM_PMASKW { w: self }
    }
    #[doc = "Bit 2 - EEPROM Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_emask(&mut self) -> _FLASH_FCIM_EMASKW {
        _FLASH_FCIM_EMASKW { w: self }
    }
    #[doc = "Bit 9 - VOLT Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_voltmask(&mut self) -> _FLASH_FCIM_VOLTMASKW {
        _FLASH_FCIM_VOLTMASKW { w: self }
    }
    #[doc = "Bit 10 - Invalid Data Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_invdmask(&mut self) -> _FLASH_FCIM_INVDMASKW {
        _FLASH_FCIM_INVDMASKW { w: self }
    }
    #[doc = "Bit 11 - ERVER Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_ermask(&mut self) -> _FLASH_FCIM_ERMASKW {
        _FLASH_FCIM_ERMASKW { w: self }
    }
    #[doc = "Bit 13 - PROGVER Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_progmask(&mut self) -> _FLASH_FCIM_PROGMASKW {
        _FLASH_FCIM_PROGMASKW { w: self }
    }
}
