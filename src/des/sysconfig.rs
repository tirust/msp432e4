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
pub struct DES_SYSCONFIG_SOFTRESETR {
    bits: bool,
}
impl DES_SYSCONFIG_SOFTRESETR {
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
pub struct _DES_SYSCONFIG_SOFTRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_SYSCONFIG_SOFTRESETW<'a> {
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
#[doc = "Possible values of the field `DES_SYSCONFIG_SIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DES_SYSCONFIG_SIDLER {
    #[doc = "Force-idle mode"]
    DES_SYSCONFIG_SIDLE_FORCE,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl DES_SYSCONFIG_SIDLER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            DES_SYSCONFIG_SIDLER::DES_SYSCONFIG_SIDLE_FORCE => 0,
            DES_SYSCONFIG_SIDLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> DES_SYSCONFIG_SIDLER {
        match value {
            0 => DES_SYSCONFIG_SIDLER::DES_SYSCONFIG_SIDLE_FORCE,
            i => DES_SYSCONFIG_SIDLER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DES_SYSCONFIG_SIDLE_FORCE`"]
    #[inline(always)]
    pub fn is_des_sysconfig_sidle_force(&self) -> bool {
        *self == DES_SYSCONFIG_SIDLER::DES_SYSCONFIG_SIDLE_FORCE
    }
}
#[doc = "Values that can be written to the field `DES_SYSCONFIG_SIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DES_SYSCONFIG_SIDLEW {
    #[doc = "Force-idle mode"]
    DES_SYSCONFIG_SIDLE_FORCE,
}
impl DES_SYSCONFIG_SIDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DES_SYSCONFIG_SIDLEW::DES_SYSCONFIG_SIDLE_FORCE => 0,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DES_SYSCONFIG_SIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_SYSCONFIG_SIDLEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DES_SYSCONFIG_SIDLEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Force-idle mode"]
    #[inline(always)]
    pub fn des_sysconfig_sidle_force(self) -> &'a mut W {
        self.variant(DES_SYSCONFIG_SIDLEW::DES_SYSCONFIG_SIDLE_FORCE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct DES_SYSCONFIG_DMA_REQ_DATA_IN_ENR {
    bits: bool,
}
impl DES_SYSCONFIG_DMA_REQ_DATA_IN_ENR {
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
pub struct _DES_SYSCONFIG_DMA_REQ_DATA_IN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_SYSCONFIG_DMA_REQ_DATA_IN_ENW<'a> {
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
#[doc = r"Value of the field"]
pub struct DES_SYSCONFIG_DMA_REQ_DATA_OUT_ENR {
    bits: bool,
}
impl DES_SYSCONFIG_DMA_REQ_DATA_OUT_ENR {
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
pub struct _DES_SYSCONFIG_DMA_REQ_DATA_OUT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_SYSCONFIG_DMA_REQ_DATA_OUT_ENW<'a> {
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
        self.w.bits &= !(1 << 6);
        self.w.bits |= ((value as u32) & 1) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_ENR {
    bits: bool,
}
impl DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_ENR {
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
pub struct _DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_ENW<'a> {
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
    pub fn des_sysconfig_softreset(&self) -> DES_SYSCONFIG_SOFTRESETR {
        let bits = ((self.bits >> 1) & 1) != 0;
        DES_SYSCONFIG_SOFTRESETR { bits }
    }
    #[doc = "Bits 2:3 - Sidle mode"]
    #[inline(always)]
    pub fn des_sysconfig_sidle(&self) -> DES_SYSCONFIG_SIDLER {
        DES_SYSCONFIG_SIDLER::_from(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - DMA Request Data In Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_data_in_en(&self) -> DES_SYSCONFIG_DMA_REQ_DATA_IN_ENR {
        let bits = ((self.bits >> 5) & 1) != 0;
        DES_SYSCONFIG_DMA_REQ_DATA_IN_ENR { bits }
    }
    #[doc = "Bit 6 - DMA Request Data Out Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_data_out_en(&self) -> DES_SYSCONFIG_DMA_REQ_DATA_OUT_ENR {
        let bits = ((self.bits >> 6) & 1) != 0;
        DES_SYSCONFIG_DMA_REQ_DATA_OUT_ENR { bits }
    }
    #[doc = "Bit 7 - DMA Request Context In Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_context_in_en(&self) -> DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_ENR {
        let bits = ((self.bits >> 7) & 1) != 0;
        DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_ENR { bits }
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
    pub fn des_sysconfig_softreset(&mut self) -> _DES_SYSCONFIG_SOFTRESETW {
        _DES_SYSCONFIG_SOFTRESETW { w: self }
    }
    #[doc = "Bits 2:3 - Sidle mode"]
    #[inline(always)]
    pub fn des_sysconfig_sidle(&mut self) -> _DES_SYSCONFIG_SIDLEW {
        _DES_SYSCONFIG_SIDLEW { w: self }
    }
    #[doc = "Bit 5 - DMA Request Data In Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_data_in_en(&mut self) -> _DES_SYSCONFIG_DMA_REQ_DATA_IN_ENW {
        _DES_SYSCONFIG_DMA_REQ_DATA_IN_ENW { w: self }
    }
    #[doc = "Bit 6 - DMA Request Data Out Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_data_out_en(&mut self) -> _DES_SYSCONFIG_DMA_REQ_DATA_OUT_ENW {
        _DES_SYSCONFIG_DMA_REQ_DATA_OUT_ENW { w: self }
    }
    #[doc = "Bit 7 - DMA Request Context In Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_context_in_en(&mut self) -> _DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_ENW {
        _DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_ENW { w: self }
    }
}
