#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FMC {
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
pub struct FLASH_FMC_WRITER {
    bits: bool,
}
impl FLASH_FMC_WRITER {
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
pub struct _FLASH_FMC_WRITEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FMC_WRITEW<'a> {
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
pub struct FLASH_FMC_ERASER {
    bits: bool,
}
impl FLASH_FMC_ERASER {
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
pub struct _FLASH_FMC_ERASEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FMC_ERASEW<'a> {
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
pub struct FLASH_FMC_MERASER {
    bits: bool,
}
impl FLASH_FMC_MERASER {
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
pub struct _FLASH_FMC_MERASEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FMC_MERASEW<'a> {
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
pub struct FLASH_FMC_COMTR {
    bits: bool,
}
impl FLASH_FMC_COMTR {
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
pub struct _FLASH_FMC_COMTW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FMC_COMTW<'a> {
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
pub struct FLASH_FMC_WRKEYR {
    bits: u16,
}
impl FLASH_FMC_WRKEYR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_FMC_WRKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_FMC_WRKEYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(32767 << 17);
        self.w.bits |= ((value as u32) & 32767) << 17;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Write a Word into Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_write(&self) -> FLASH_FMC_WRITER {
        let bits = ((self.bits >> 0) & 1) != 0;
        FLASH_FMC_WRITER { bits }
    }
    #[doc = "Bit 1 - Erase a Page of Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_erase(&self) -> FLASH_FMC_ERASER {
        let bits = ((self.bits >> 1) & 1) != 0;
        FLASH_FMC_ERASER { bits }
    }
    #[doc = "Bit 2 - Mass Erase Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_merase(&self) -> FLASH_FMC_MERASER {
        let bits = ((self.bits >> 2) & 1) != 0;
        FLASH_FMC_MERASER { bits }
    }
    #[doc = "Bit 3 - Commit Register Value"]
    #[inline(always)]
    pub fn flash_fmc_comt(&self) -> FLASH_FMC_COMTR {
        let bits = ((self.bits >> 3) & 1) != 0;
        FLASH_FMC_COMTR { bits }
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    pub fn flash_fmc_wrkey(&self) -> FLASH_FMC_WRKEYR {
        let bits = ((self.bits >> 17) & 32767) as u16;
        FLASH_FMC_WRKEYR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Write a Word into Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_write(&mut self) -> _FLASH_FMC_WRITEW {
        _FLASH_FMC_WRITEW { w: self }
    }
    #[doc = "Bit 1 - Erase a Page of Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_erase(&mut self) -> _FLASH_FMC_ERASEW {
        _FLASH_FMC_ERASEW { w: self }
    }
    #[doc = "Bit 2 - Mass Erase Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_merase(&mut self) -> _FLASH_FMC_MERASEW {
        _FLASH_FMC_MERASEW { w: self }
    }
    #[doc = "Bit 3 - Commit Register Value"]
    #[inline(always)]
    pub fn flash_fmc_comt(&mut self) -> _FLASH_FMC_COMTW {
        _FLASH_FMC_COMTW { w: self }
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    pub fn flash_fmc_wrkey(&mut self) -> _FLASH_FMC_WRKEYW {
        _FLASH_FMC_WRKEYW { w: self }
    }
}
