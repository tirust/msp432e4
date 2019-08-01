#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RIS {
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
pub struct EPI_RIS_ERRRISR {
    bits: bool,
}
impl EPI_RIS_ERRRISR {
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
pub struct _EPI_RIS_ERRRISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_RIS_ERRRISW<'a> {
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
pub struct EPI_RIS_RDRISR {
    bits: bool,
}
impl EPI_RIS_RDRISR {
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
pub struct _EPI_RIS_RDRISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_RIS_RDRISW<'a> {
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
pub struct EPI_RIS_WRRISR {
    bits: bool,
}
impl EPI_RIS_WRRISR {
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
pub struct _EPI_RIS_WRRISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_RIS_WRRISW<'a> {
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
pub struct EPI_RIS_DMARDRISR {
    bits: bool,
}
impl EPI_RIS_DMARDRISR {
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
pub struct _EPI_RIS_DMARDRISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_RIS_DMARDRISW<'a> {
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
pub struct EPI_RIS_DMAWRRISR {
    bits: bool,
}
impl EPI_RIS_DMAWRRISR {
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
pub struct _EPI_RIS_DMAWRRISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_RIS_DMAWRRISW<'a> {
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
    #[doc = "Bit 0 - Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_errris(&self) -> EPI_RIS_ERRRISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EPI_RIS_ERRRISR { bits }
    }
    #[doc = "Bit 1 - Read Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_rdris(&self) -> EPI_RIS_RDRISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EPI_RIS_RDRISR { bits }
    }
    #[doc = "Bit 2 - Write Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_wrris(&self) -> EPI_RIS_WRRISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EPI_RIS_WRRISR { bits }
    }
    #[doc = "Bit 3 - Read uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_dmardris(&self) -> EPI_RIS_DMARDRISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EPI_RIS_DMARDRISR { bits }
    }
    #[doc = "Bit 4 - Write uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_dmawrris(&self) -> EPI_RIS_DMAWRRISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EPI_RIS_DMAWRRISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_errris(&mut self) -> _EPI_RIS_ERRRISW {
        _EPI_RIS_ERRRISW { w: self }
    }
    #[doc = "Bit 1 - Read Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_rdris(&mut self) -> _EPI_RIS_RDRISW {
        _EPI_RIS_RDRISW { w: self }
    }
    #[doc = "Bit 2 - Write Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_wrris(&mut self) -> _EPI_RIS_WRRISW {
        _EPI_RIS_WRRISW { w: self }
    }
    #[doc = "Bit 3 - Read uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_dmardris(&mut self) -> _EPI_RIS_DMARDRISW {
        _EPI_RIS_DMARDRISW { w: self }
    }
    #[doc = "Bit 4 - Write uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_dmawrris(&mut self) -> _EPI_RIS_DMAWRRISW {
        _EPI_RIS_DMAWRRISW { w: self }
    }
}
