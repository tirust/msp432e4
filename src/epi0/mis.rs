#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MIS {
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
pub struct EPI_MIS_ERRMISR {
    bits: bool,
}
impl EPI_MIS_ERRMISR {
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
pub struct _EPI_MIS_ERRMISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_MIS_ERRMISW<'a> {
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
pub struct EPI_MIS_RDMISR {
    bits: bool,
}
impl EPI_MIS_RDMISR {
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
pub struct _EPI_MIS_RDMISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_MIS_RDMISW<'a> {
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
pub struct EPI_MIS_WRMISR {
    bits: bool,
}
impl EPI_MIS_WRMISR {
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
pub struct _EPI_MIS_WRMISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_MIS_WRMISW<'a> {
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
pub struct EPI_MIS_DMARDMISR {
    bits: bool,
}
impl EPI_MIS_DMARDMISR {
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
pub struct _EPI_MIS_DMARDMISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_MIS_DMARDMISW<'a> {
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
pub struct EPI_MIS_DMAWRMISR {
    bits: bool,
}
impl EPI_MIS_DMAWRMISR {
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
pub struct _EPI_MIS_DMAWRMISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_MIS_DMAWRMISW<'a> {
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
    #[doc = "Bit 0 - Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_errmis(&self) -> EPI_MIS_ERRMISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EPI_MIS_ERRMISR { bits }
    }
    #[doc = "Bit 1 - Read Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_rdmis(&self) -> EPI_MIS_RDMISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EPI_MIS_RDMISR { bits }
    }
    #[doc = "Bit 2 - Write Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_wrmis(&self) -> EPI_MIS_WRMISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EPI_MIS_WRMISR { bits }
    }
    #[doc = "Bit 3 - Read uDMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_dmardmis(&self) -> EPI_MIS_DMARDMISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EPI_MIS_DMARDMISR { bits }
    }
    #[doc = "Bit 4 - Write uDMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_dmawrmis(&self) -> EPI_MIS_DMAWRMISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EPI_MIS_DMAWRMISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_errmis(&mut self) -> _EPI_MIS_ERRMISW {
        _EPI_MIS_ERRMISW { w: self }
    }
    #[doc = "Bit 1 - Read Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_rdmis(&mut self) -> _EPI_MIS_RDMISW {
        _EPI_MIS_RDMISW { w: self }
    }
    #[doc = "Bit 2 - Write Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_wrmis(&mut self) -> _EPI_MIS_WRMISW {
        _EPI_MIS_WRMISW { w: self }
    }
    #[doc = "Bit 3 - Read uDMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_dmardmis(&mut self) -> _EPI_MIS_DMARDMISW {
        _EPI_MIS_DMARDMISW { w: self }
    }
    #[doc = "Bit 4 - Write uDMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_dmawrmis(&mut self) -> _EPI_MIS_DMAWRMISW {
        _EPI_MIS_DMAWRMISW { w: self }
    }
}
