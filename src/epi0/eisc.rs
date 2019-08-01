#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EISC {
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
pub struct EPI_EISC_TOUTR {
    bits: bool,
}
impl EPI_EISC_TOUTR {
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
pub struct _EPI_EISC_TOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_EISC_TOUTW<'a> {
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
pub struct EPI_EISC_RSTALLR {
    bits: bool,
}
impl EPI_EISC_RSTALLR {
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
pub struct _EPI_EISC_RSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_EISC_RSTALLW<'a> {
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
pub struct EPI_EISC_WTFULLR {
    bits: bool,
}
impl EPI_EISC_WTFULLR {
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
pub struct _EPI_EISC_WTFULLW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_EISC_WTFULLW<'a> {
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
pub struct EPI_EISC_DMARDICR {
    bits: bool,
}
impl EPI_EISC_DMARDICR {
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
pub struct _EPI_EISC_DMARDICW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_EISC_DMARDICW<'a> {
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
pub struct EPI_EISC_DMAWRICR {
    bits: bool,
}
impl EPI_EISC_DMAWRICR {
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
pub struct _EPI_EISC_DMAWRICW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_EISC_DMAWRICW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Timeout Error"]
    #[inline(always)]
    pub fn epi_eisc_tout(&self) -> EPI_EISC_TOUTR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EPI_EISC_TOUTR { bits }
    }
    #[doc = "Bit 1 - Read Stalled Error"]
    #[inline(always)]
    pub fn epi_eisc_rstall(&self) -> EPI_EISC_RSTALLR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EPI_EISC_RSTALLR { bits }
    }
    #[doc = "Bit 2 - Write FIFO Full Error"]
    #[inline(always)]
    pub fn epi_eisc_wtfull(&self) -> EPI_EISC_WTFULLR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EPI_EISC_WTFULLR { bits }
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn epi_eisc_dmardic(&self) -> EPI_EISC_DMARDICR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EPI_EISC_DMARDICR { bits }
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn epi_eisc_dmawric(&self) -> EPI_EISC_DMAWRICR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EPI_EISC_DMAWRICR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Timeout Error"]
    #[inline(always)]
    pub fn epi_eisc_tout(&mut self) -> _EPI_EISC_TOUTW {
        _EPI_EISC_TOUTW { w: self }
    }
    #[doc = "Bit 1 - Read Stalled Error"]
    #[inline(always)]
    pub fn epi_eisc_rstall(&mut self) -> _EPI_EISC_RSTALLW {
        _EPI_EISC_RSTALLW { w: self }
    }
    #[doc = "Bit 2 - Write FIFO Full Error"]
    #[inline(always)]
    pub fn epi_eisc_wtfull(&mut self) -> _EPI_EISC_WTFULLW {
        _EPI_EISC_WTFULLW { w: self }
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn epi_eisc_dmardic(&mut self) -> _EPI_EISC_DMARDICW {
        _EPI_EISC_DMARDICW { w: self }
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn epi_eisc_dmawric(&mut self) -> _EPI_EISC_DMAWRICW {
        _EPI_EISC_DMAWRICW { w: self }
    }
}
