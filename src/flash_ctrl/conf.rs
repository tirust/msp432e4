#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONF {
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
pub struct FLASH_CONF_FPFOFFR {
    bits: bool,
}
impl FLASH_CONF_FPFOFFR {
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
pub struct _FLASH_CONF_FPFOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_CONF_FPFOFFW<'a> {
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
        self.w.bits &= !(1 << 16);
        self.w.bits |= ((value as u32) & 1) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct FLASH_CONF_FPFONR {
    bits: bool,
}
impl FLASH_CONF_FPFONR {
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
pub struct _FLASH_CONF_FPFONW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_CONF_FPFONW<'a> {
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
        self.w.bits &= !(1 << 17);
        self.w.bits |= ((value as u32) & 1) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct FLASH_CONF_CLRTVR {
    bits: bool,
}
impl FLASH_CONF_CLRTVR {
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
pub struct _FLASH_CONF_CLRTVW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_CONF_CLRTVW<'a> {
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
        self.w.bits &= !(1 << 20);
        self.w.bits |= ((value as u32) & 1) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct FLASH_CONF_SPFER {
    bits: bool,
}
impl FLASH_CONF_SPFER {
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
pub struct _FLASH_CONF_SPFEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_CONF_SPFEW<'a> {
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
pub struct FLASH_CONF_FMMER {
    bits: bool,
}
impl FLASH_CONF_FMMER {
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
pub struct _FLASH_CONF_FMMEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_CONF_FMMEW<'a> {
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
    #[doc = "Bit 16 - Force Prefetch Off"]
    #[inline(always)]
    pub fn flash_conf_fpfoff(&self) -> FLASH_CONF_FPFOFFR {
        let bits = ((self.bits >> 16) & 1) != 0;
        FLASH_CONF_FPFOFFR { bits }
    }
    #[doc = "Bit 17 - Force Prefetch On"]
    #[inline(always)]
    pub fn flash_conf_fpfon(&self) -> FLASH_CONF_FPFONR {
        let bits = ((self.bits >> 17) & 1) != 0;
        FLASH_CONF_FPFONR { bits }
    }
    #[doc = "Bit 20 - Clear Valid Tags"]
    #[inline(always)]
    pub fn flash_conf_clrtv(&self) -> FLASH_CONF_CLRTVR {
        let bits = ((self.bits >> 20) & 1) != 0;
        FLASH_CONF_CLRTVR { bits }
    }
    #[doc = "Bit 29 - Single Prefetch Mode Enable"]
    #[inline(always)]
    pub fn flash_conf_spfe(&self) -> FLASH_CONF_SPFER {
        let bits = ((self.bits >> 29) & 1) != 0;
        FLASH_CONF_SPFER { bits }
    }
    #[doc = "Bit 30 - Flash Mirror Mode Enable"]
    #[inline(always)]
    pub fn flash_conf_fmme(&self) -> FLASH_CONF_FMMER {
        let bits = ((self.bits >> 30) & 1) != 0;
        FLASH_CONF_FMMER { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 16 - Force Prefetch Off"]
    #[inline(always)]
    pub fn flash_conf_fpfoff(&mut self) -> _FLASH_CONF_FPFOFFW {
        _FLASH_CONF_FPFOFFW { w: self }
    }
    #[doc = "Bit 17 - Force Prefetch On"]
    #[inline(always)]
    pub fn flash_conf_fpfon(&mut self) -> _FLASH_CONF_FPFONW {
        _FLASH_CONF_FPFONW { w: self }
    }
    #[doc = "Bit 20 - Clear Valid Tags"]
    #[inline(always)]
    pub fn flash_conf_clrtv(&mut self) -> _FLASH_CONF_CLRTVW {
        _FLASH_CONF_CLRTVW { w: self }
    }
    #[doc = "Bit 29 - Single Prefetch Mode Enable"]
    #[inline(always)]
    pub fn flash_conf_spfe(&mut self) -> _FLASH_CONF_SPFEW {
        _FLASH_CONF_SPFEW { w: self }
    }
    #[doc = "Bit 30 - Flash Mirror Mode Enable"]
    #[inline(always)]
    pub fn flash_conf_fmme(&mut self) -> _FLASH_CONF_FMMEW {
        _FLASH_CONF_FMMEW { w: self }
    }
}
