#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSCONFIG {
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
pub struct SHAMD5_SYSCONFIG_SOFTRESETR {
    bits: bool,
}
impl SHAMD5_SYSCONFIG_SOFTRESETR {
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
pub struct _SHAMD5_SYSCONFIG_SOFTRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_SYSCONFIG_SOFTRESETW<'a> {
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
pub struct SHAMD5_SYSCONFIG_IT_ENR {
    bits: bool,
}
impl SHAMD5_SYSCONFIG_IT_ENR {
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
pub struct _SHAMD5_SYSCONFIG_IT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_SYSCONFIG_IT_ENW<'a> {
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
pub struct SHAMD5_SYSCONFIG_DMA_ENR {
    bits: bool,
}
impl SHAMD5_SYSCONFIG_DMA_ENR {
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
pub struct _SHAMD5_SYSCONFIG_DMA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_SYSCONFIG_DMA_ENW<'a> {
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
#[doc = "Possible values of the field `SHAMD5_SYSCONFIG_SIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHAMD5_SYSCONFIG_SIDLER {
    #[doc = "Force-idle mode"]
    SHAMD5_SYSCONFIG_SIDLE_FORCE,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SHAMD5_SYSCONFIG_SIDLER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SHAMD5_SYSCONFIG_SIDLER::SHAMD5_SYSCONFIG_SIDLE_FORCE => 0,
            SHAMD5_SYSCONFIG_SIDLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SHAMD5_SYSCONFIG_SIDLER {
        match value {
            0 => SHAMD5_SYSCONFIG_SIDLER::SHAMD5_SYSCONFIG_SIDLE_FORCE,
            i => SHAMD5_SYSCONFIG_SIDLER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SHAMD5_SYSCONFIG_SIDLE_FORCE`"]
    #[inline(always)]
    pub fn is_shamd5_sysconfig_sidle_force(&self) -> bool {
        *self == SHAMD5_SYSCONFIG_SIDLER::SHAMD5_SYSCONFIG_SIDLE_FORCE
    }
}
#[doc = "Values that can be written to the field `SHAMD5_SYSCONFIG_SIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHAMD5_SYSCONFIG_SIDLEW {
    #[doc = "Force-idle mode"]
    SHAMD5_SYSCONFIG_SIDLE_FORCE,
}
impl SHAMD5_SYSCONFIG_SIDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SHAMD5_SYSCONFIG_SIDLEW::SHAMD5_SYSCONFIG_SIDLE_FORCE => 0,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SHAMD5_SYSCONFIG_SIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_SYSCONFIG_SIDLEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHAMD5_SYSCONFIG_SIDLEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Force-idle mode"]
    #[inline(always)]
    pub fn shamd5_sysconfig_sidle_force(self) -> &'a mut W {
        self.variant(SHAMD5_SYSCONFIG_SIDLEW::SHAMD5_SYSCONFIG_SIDLE_FORCE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SHAMD5_SYSCONFIG_SADVANCEDR {
    bits: bool,
}
impl SHAMD5_SYSCONFIG_SADVANCEDR {
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
pub struct _SHAMD5_SYSCONFIG_SADVANCEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_SYSCONFIG_SADVANCEDW<'a> {
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
        self.w.bits &= !(1 << 7);
        self.w.bits |= ((value as u32) & 1) << 7;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Soft reset"]
    #[inline(always)]
    pub fn shamd5_sysconfig_softreset(&self) -> SHAMD5_SYSCONFIG_SOFTRESETR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SHAMD5_SYSCONFIG_SOFTRESETR { bits }
    }
    #[doc = "Bit 2 - Interrupt Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_it_en(&self) -> SHAMD5_SYSCONFIG_IT_ENR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SHAMD5_SYSCONFIG_IT_ENR { bits }
    }
    #[doc = "Bit 3 - uDMA Request Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_dma_en(&self) -> SHAMD5_SYSCONFIG_DMA_ENR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SHAMD5_SYSCONFIG_DMA_ENR { bits }
    }
    #[doc = "Bits 4:5 - Sidle mode"]
    #[inline(always)]
    pub fn shamd5_sysconfig_sidle(&self) -> SHAMD5_SYSCONFIG_SIDLER {
        SHAMD5_SYSCONFIG_SIDLER::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Advanced Mode Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_sadvanced(&self) -> SHAMD5_SYSCONFIG_SADVANCEDR {
        let bits = ((self.bits >> 7) & 1) != 0;
        SHAMD5_SYSCONFIG_SADVANCEDR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Soft reset"]
    #[inline(always)]
    pub fn shamd5_sysconfig_softreset(&mut self) -> _SHAMD5_SYSCONFIG_SOFTRESETW {
        _SHAMD5_SYSCONFIG_SOFTRESETW { w: self }
    }
    #[doc = "Bit 2 - Interrupt Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_it_en(&mut self) -> _SHAMD5_SYSCONFIG_IT_ENW {
        _SHAMD5_SYSCONFIG_IT_ENW { w: self }
    }
    #[doc = "Bit 3 - uDMA Request Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_dma_en(&mut self) -> _SHAMD5_SYSCONFIG_DMA_ENW {
        _SHAMD5_SYSCONFIG_DMA_ENW { w: self }
    }
    #[doc = "Bits 4:5 - Sidle mode"]
    #[inline(always)]
    pub fn shamd5_sysconfig_sidle(&mut self) -> _SHAMD5_SYSCONFIG_SIDLEW {
        _SHAMD5_SYSCONFIG_SIDLEW { w: self }
    }
    #[doc = "Bit 7 - Advanced Mode Enable"]
    #[inline(always)]
    pub fn shamd5_sysconfig_sadvanced(&mut self) -> _SHAMD5_SYSCONFIG_SADVANCEDW {
        _SHAMD5_SYSCONFIG_SADVANCEDW { w: self }
    }
}
