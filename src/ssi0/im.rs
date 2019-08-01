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
pub struct SSI_IM_RORIMR {
    bits: bool,
}
impl SSI_IM_RORIMR {
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
pub struct _SSI_IM_RORIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_IM_RORIMW<'a> {
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
pub struct SSI_IM_RTIMR {
    bits: bool,
}
impl SSI_IM_RTIMR {
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
pub struct _SSI_IM_RTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_IM_RTIMW<'a> {
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
pub struct SSI_IM_RXIMR {
    bits: bool,
}
impl SSI_IM_RXIMR {
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
pub struct _SSI_IM_RXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_IM_RXIMW<'a> {
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
pub struct SSI_IM_TXIMR {
    bits: bool,
}
impl SSI_IM_TXIMR {
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
pub struct _SSI_IM_TXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_IM_TXIMW<'a> {
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
pub struct SSI_IM_DMARXIMR {
    bits: bool,
}
impl SSI_IM_DMARXIMR {
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
pub struct _SSI_IM_DMARXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_IM_DMARXIMW<'a> {
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
#[doc = r"Value of the field"]
pub struct SSI_IM_DMATXIMR {
    bits: bool,
}
impl SSI_IM_DMATXIMR {
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
pub struct _SSI_IM_DMATXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_IM_DMATXIMW<'a> {
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
pub struct SSI_IM_EOTIMR {
    bits: bool,
}
impl SSI_IM_EOTIMR {
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
pub struct _SSI_IM_EOTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_IM_EOTIMW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rorim(&self) -> SSI_IM_RORIMR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SSI_IM_RORIMR { bits }
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rtim(&self) -> SSI_IM_RTIMR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SSI_IM_RTIMR { bits }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rxim(&self) -> SSI_IM_RXIMR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SSI_IM_RXIMR { bits }
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_txim(&self) -> SSI_IM_TXIMR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SSI_IM_TXIMR { bits }
    }
    #[doc = "Bit 4 - SSI Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_dmarxim(&self) -> SSI_IM_DMARXIMR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SSI_IM_DMARXIMR { bits }
    }
    #[doc = "Bit 5 - SSI Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_dmatxim(&self) -> SSI_IM_DMATXIMR {
        let bits = ((self.bits >> 5) & 1) != 0;
        SSI_IM_DMATXIMR { bits }
    }
    #[doc = "Bit 6 - End of Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_eotim(&self) -> SSI_IM_EOTIMR {
        let bits = ((self.bits >> 6) & 1) != 0;
        SSI_IM_EOTIMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rorim(&mut self) -> _SSI_IM_RORIMW {
        _SSI_IM_RORIMW { w: self }
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rtim(&mut self) -> _SSI_IM_RTIMW {
        _SSI_IM_RTIMW { w: self }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rxim(&mut self) -> _SSI_IM_RXIMW {
        _SSI_IM_RXIMW { w: self }
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_txim(&mut self) -> _SSI_IM_TXIMW {
        _SSI_IM_TXIMW { w: self }
    }
    #[doc = "Bit 4 - SSI Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_dmarxim(&mut self) -> _SSI_IM_DMARXIMW {
        _SSI_IM_DMARXIMW { w: self }
    }
    #[doc = "Bit 5 - SSI Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_dmatxim(&mut self) -> _SSI_IM_DMATXIMW {
        _SSI_IM_DMATXIMW { w: self }
    }
    #[doc = "Bit 6 - End of Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_eotim(&mut self) -> _SSI_IM_EOTIMW {
        _SSI_IM_EOTIMW { w: self }
    }
}
