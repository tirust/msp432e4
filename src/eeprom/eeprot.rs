#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEPROT {
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
#[doc = "Possible values of the field `EEPROM_EEPROT_PROT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEPROM_EEPROT_PROTR {
    #[doc = "This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    EEPROM_EEPROT_PROT_RWNPW,
    #[doc = "If there is a password, the block is readable or writable only when unlocked"]
    EEPROM_EEPROT_PROT_RWPW,
    #[doc = "If there is no password, the block is readable, not writable"]
    EEPROM_EEPROT_PROT_RONPW,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EEPROM_EEPROT_PROTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EEPROM_EEPROT_PROTR::EEPROM_EEPROT_PROT_RWNPW => 0,
            EEPROM_EEPROT_PROTR::EEPROM_EEPROT_PROT_RWPW => 1,
            EEPROM_EEPROT_PROTR::EEPROM_EEPROT_PROT_RONPW => 2,
            EEPROM_EEPROT_PROTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EEPROM_EEPROT_PROTR {
        match value {
            0 => EEPROM_EEPROT_PROTR::EEPROM_EEPROT_PROT_RWNPW,
            1 => EEPROM_EEPROT_PROTR::EEPROM_EEPROT_PROT_RWPW,
            2 => EEPROM_EEPROT_PROTR::EEPROM_EEPROT_PROT_RONPW,
            i => EEPROM_EEPROT_PROTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EEPROM_EEPROT_PROT_RWNPW`"]
    #[inline(always)]
    pub fn is_eeprom_eeprot_prot_rwnpw(&self) -> bool {
        *self == EEPROM_EEPROT_PROTR::EEPROM_EEPROT_PROT_RWNPW
    }
    #[doc = "Checks if the value of the field is `EEPROM_EEPROT_PROT_RWPW`"]
    #[inline(always)]
    pub fn is_eeprom_eeprot_prot_rwpw(&self) -> bool {
        *self == EEPROM_EEPROT_PROTR::EEPROM_EEPROT_PROT_RWPW
    }
    #[doc = "Checks if the value of the field is `EEPROM_EEPROT_PROT_RONPW`"]
    #[inline(always)]
    pub fn is_eeprom_eeprot_prot_ronpw(&self) -> bool {
        *self == EEPROM_EEPROT_PROTR::EEPROM_EEPROT_PROT_RONPW
    }
}
#[doc = "Values that can be written to the field `EEPROM_EEPROT_PROT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEPROM_EEPROT_PROTW {
    #[doc = "This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    EEPROM_EEPROT_PROT_RWNPW,
    #[doc = "If there is a password, the block is readable or writable only when unlocked"]
    EEPROM_EEPROT_PROT_RWPW,
    #[doc = "If there is no password, the block is readable, not writable"]
    EEPROM_EEPROT_PROT_RONPW,
}
impl EEPROM_EEPROT_PROTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EEPROM_EEPROT_PROTW::EEPROM_EEPROT_PROT_RWNPW => 0,
            EEPROM_EEPROT_PROTW::EEPROM_EEPROT_PROT_RWPW => 1,
            EEPROM_EEPROT_PROTW::EEPROM_EEPROT_PROT_RONPW => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EEPROM_EEPROT_PROTW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_EEPROT_PROTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEPROM_EEPROT_PROTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot_rwnpw(self) -> &'a mut W {
        self.variant(EEPROM_EEPROT_PROTW::EEPROM_EEPROT_PROT_RWNPW)
    }
    #[doc = "If there is a password, the block is readable or writable only when unlocked"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot_rwpw(self) -> &'a mut W {
        self.variant(EEPROM_EEPROT_PROTW::EEPROM_EEPROT_PROT_RWPW)
    }
    #[doc = "If there is no password, the block is readable, not writable"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot_ronpw(self) -> &'a mut W {
        self.variant(EEPROM_EEPROT_PROTW::EEPROM_EEPROT_PROT_RONPW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 0);
        self.w.bits |= ((value as u32) & 7) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EEPROM_EEPROT_ACCR {
    bits: bool,
}
impl EEPROM_EEPROT_ACCR {
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
pub struct _EEPROM_EEPROT_ACCW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_EEPROT_ACCW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Protection Control"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot(&self) -> EEPROM_EEPROT_PROTR {
        EEPROM_EEPROT_PROTR::_from(((self.bits >> 0) & 7) as u8)
    }
    #[doc = "Bit 3 - Access Control"]
    #[inline(always)]
    pub fn eeprom_eeprot_acc(&self) -> EEPROM_EEPROT_ACCR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EEPROM_EEPROT_ACCR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Protection Control"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot(&mut self) -> _EEPROM_EEPROT_PROTW {
        _EEPROM_EEPROT_PROTW { w: self }
    }
    #[doc = "Bit 3 - Access Control"]
    #[inline(always)]
    pub fn eeprom_eeprot_acc(&mut self) -> _EEPROM_EEPROT_ACCW {
        _EEPROM_EEPROT_ACCW { w: self }
    }
}
