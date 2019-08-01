#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IM {
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
pub struct EPI_IM_ERRIMR {
    bits: bool,
}
impl EPI_IM_ERRIMR {
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
pub struct _EPI_IM_ERRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_IM_ERRIMW<'a> {
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
pub struct EPI_IM_RDIMR {
    bits: bool,
}
impl EPI_IM_RDIMR {
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
pub struct _EPI_IM_RDIMW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_IM_RDIMW<'a> {
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
pub struct EPI_IM_WRIMR {
    bits: bool,
}
impl EPI_IM_WRIMR {
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
pub struct _EPI_IM_WRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_IM_WRIMW<'a> {
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
pub struct EPI_IM_DMARDIMR {
    bits: bool,
}
impl EPI_IM_DMARDIMR {
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
pub struct _EPI_IM_DMARDIMW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_IM_DMARDIMW<'a> {
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
pub struct EPI_IM_DMAWRIMR {
    bits: bool,
}
impl EPI_IM_DMAWRIMR {
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
pub struct _EPI_IM_DMAWRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_IM_DMAWRIMW<'a> {
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
    #[doc = "Bit 0 - Error Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_errim(&self) -> EPI_IM_ERRIMR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EPI_IM_ERRIMR { bits }
    }
    #[doc = "Bit 1 - Read FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_rdim(&self) -> EPI_IM_RDIMR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EPI_IM_RDIMR { bits }
    }
    #[doc = "Bit 2 - Write FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_wrim(&self) -> EPI_IM_WRIMR {
        let bits = ((self.bits >> 2) & 1) != 0;
        EPI_IM_WRIMR { bits }
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_dmardim(&self) -> EPI_IM_DMARDIMR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EPI_IM_DMARDIMR { bits }
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_dmawrim(&self) -> EPI_IM_DMAWRIMR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EPI_IM_DMAWRIMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Error Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_errim(&mut self) -> _EPI_IM_ERRIMW {
        _EPI_IM_ERRIMW { w: self }
    }
    #[doc = "Bit 1 - Read FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_rdim(&mut self) -> _EPI_IM_RDIMW {
        _EPI_IM_RDIMW { w: self }
    }
    #[doc = "Bit 2 - Write FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_wrim(&mut self) -> _EPI_IM_WRIMW {
        _EPI_IM_WRIMW { w: self }
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_dmardim(&mut self) -> _EPI_IM_DMARDIMW {
        _EPI_IM_DMARDIMW { w: self }
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_dmawrim(&mut self) -> _EPI_IM_DMAWRIMW {
        _EPI_IM_DMAWRIMW { w: self }
    }
}
