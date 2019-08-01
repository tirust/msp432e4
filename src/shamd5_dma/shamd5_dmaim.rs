#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHAMD5_DMAIM {
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
pub struct SHAMD5_DMAIM_CINR {
    bits: bool,
}
impl SHAMD5_DMAIM_CINR {
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
pub struct _SHAMD5_DMAIM_CINW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_DMAIM_CINW<'a> {
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
pub struct SHAMD5_DMAIM_COUTR {
    bits: bool,
}
impl SHAMD5_DMAIM_COUTR {
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
pub struct _SHAMD5_DMAIM_COUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_DMAIM_COUTW<'a> {
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
pub struct SHAMD5_DMAIM_DINR {
    bits: bool,
}
impl SHAMD5_DMAIM_DINR {
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
pub struct _SHAMD5_DMAIM_DINW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_DMAIM_DINW<'a> {
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
pub struct SHAMD5_DMAIM_DOUTR {
    bits: bool,
}
impl SHAMD5_DMAIM_DOUTR {
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
pub struct _SHAMD5_DMAIM_DOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SHAMD5_DMAIM_DOUTW<'a> {
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
    #[doc = "Bit 0 - Context In DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn shamd5_dmaim_cin(&self) -> SHAMD5_DMAIM_CINR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SHAMD5_DMAIM_CINR { bits }
    }
    #[doc = "Bit 1 - Context Out DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn shamd5_dmaim_cout(&self) -> SHAMD5_DMAIM_COUTR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SHAMD5_DMAIM_COUTR { bits }
    }
    #[doc = "Bit 2 - Data In DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn shamd5_dmaim_din(&self) -> SHAMD5_DMAIM_DINR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SHAMD5_DMAIM_DINR { bits }
    }
    #[doc = "Bit 3 - Data Out DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn shamd5_dmaim_dout(&self) -> SHAMD5_DMAIM_DOUTR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SHAMD5_DMAIM_DOUTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Context In DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn shamd5_dmaim_cin(&mut self) -> _SHAMD5_DMAIM_CINW {
        _SHAMD5_DMAIM_CINW { w: self }
    }
    #[doc = "Bit 1 - Context Out DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn shamd5_dmaim_cout(&mut self) -> _SHAMD5_DMAIM_COUTW {
        _SHAMD5_DMAIM_COUTW { w: self }
    }
    #[doc = "Bit 2 - Data In DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn shamd5_dmaim_din(&mut self) -> _SHAMD5_DMAIM_DINW {
        _SHAMD5_DMAIM_DINW { w: self }
    }
    #[doc = "Bit 3 - Data Out DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn shamd5_dmaim_dout(&mut self) -> _SHAMD5_DMAIM_DOUTW {
        _SHAMD5_DMAIM_DOUTW { w: self }
    }
}