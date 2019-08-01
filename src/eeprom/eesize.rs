#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EESIZE {
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
pub struct EEPROM_EESIZE_WORDCNTR {
    bits: u16,
}
impl EEPROM_EESIZE_WORDCNTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EEPROM_EESIZE_WORDCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_EESIZE_WORDCNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(65535 << 0);
        self.w.bits |= ((value as u32) & 65535) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EEPROM_EESIZE_BLKCNTR {
    bits: u16,
}
impl EEPROM_EESIZE_BLKCNTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EEPROM_EESIZE_BLKCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_EESIZE_BLKCNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(2047 << 16);
        self.w.bits |= ((value as u32) & 2047) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Number of 32-Bit Words"]
    #[inline(always)]
    pub fn eeprom_eesize_wordcnt(&self) -> EEPROM_EESIZE_WORDCNTR {
        let bits = ((self.bits >> 0) & 65535) as u16;
        EEPROM_EESIZE_WORDCNTR { bits }
    }
    #[doc = "Bits 16:26 - Number of 16-Word Blocks"]
    #[inline(always)]
    pub fn eeprom_eesize_blkcnt(&self) -> EEPROM_EESIZE_BLKCNTR {
        let bits = ((self.bits >> 16) & 2047) as u16;
        EEPROM_EESIZE_BLKCNTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Number of 32-Bit Words"]
    #[inline(always)]
    pub fn eeprom_eesize_wordcnt(&mut self) -> _EEPROM_EESIZE_WORDCNTW {
        _EEPROM_EESIZE_WORDCNTW { w: self }
    }
    #[doc = "Bits 16:26 - Number of 16-Word Blocks"]
    #[inline(always)]
    pub fn eeprom_eesize_blkcnt(&mut self) -> _EEPROM_EESIZE_BLKCNTW {
        _EEPROM_EESIZE_BLKCNTW { w: self }
    }
}
