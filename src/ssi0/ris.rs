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
pub struct SSI_RIS_RORRISR {
    bits: bool,
}
impl SSI_RIS_RORRISR {
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
pub struct _SSI_RIS_RORRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_RIS_RORRISW<'a> {
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
pub struct SSI_RIS_RTRISR {
    bits: bool,
}
impl SSI_RIS_RTRISR {
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
pub struct _SSI_RIS_RTRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_RIS_RTRISW<'a> {
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
pub struct SSI_RIS_RXRISR {
    bits: bool,
}
impl SSI_RIS_RXRISR {
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
pub struct _SSI_RIS_RXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_RIS_RXRISW<'a> {
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
pub struct SSI_RIS_TXRISR {
    bits: bool,
}
impl SSI_RIS_TXRISR {
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
pub struct _SSI_RIS_TXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_RIS_TXRISW<'a> {
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
pub struct SSI_RIS_DMARXRISR {
    bits: bool,
}
impl SSI_RIS_DMARXRISR {
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
pub struct _SSI_RIS_DMARXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_RIS_DMARXRISW<'a> {
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
pub struct SSI_RIS_DMATXRISR {
    bits: bool,
}
impl SSI_RIS_DMATXRISR {
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
pub struct _SSI_RIS_DMATXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_RIS_DMATXRISW<'a> {
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
pub struct SSI_RIS_EOTRISR {
    bits: bool,
}
impl SSI_RIS_EOTRISR {
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
pub struct _SSI_RIS_EOTRISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_RIS_EOTRISW<'a> {
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
    #[doc = "Bit 0 - SSI Receive Overrun Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rorris(&self) -> SSI_RIS_RORRISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SSI_RIS_RORRISR { bits }
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rtris(&self) -> SSI_RIS_RTRISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SSI_RIS_RTRISR { bits }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rxris(&self) -> SSI_RIS_RXRISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SSI_RIS_RXRISR { bits }
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_txris(&self) -> SSI_RIS_TXRISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SSI_RIS_TXRISR { bits }
    }
    #[doc = "Bit 4 - SSI Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_dmarxris(&self) -> SSI_RIS_DMARXRISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SSI_RIS_DMARXRISR { bits }
    }
    #[doc = "Bit 5 - SSI Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_dmatxris(&self) -> SSI_RIS_DMATXRISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        SSI_RIS_DMATXRISR { bits }
    }
    #[doc = "Bit 6 - End of Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_eotris(&self) -> SSI_RIS_EOTRISR {
        let bits = ((self.bits >> 6) & 1) != 0;
        SSI_RIS_EOTRISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SSI Receive Overrun Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rorris(&mut self) -> _SSI_RIS_RORRISW {
        _SSI_RIS_RORRISW { w: self }
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rtris(&mut self) -> _SSI_RIS_RTRISW {
        _SSI_RIS_RTRISW { w: self }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rxris(&mut self) -> _SSI_RIS_RXRISW {
        _SSI_RIS_RXRISW { w: self }
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_txris(&mut self) -> _SSI_RIS_TXRISW {
        _SSI_RIS_TXRISW { w: self }
    }
    #[doc = "Bit 4 - SSI Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_dmarxris(&mut self) -> _SSI_RIS_DMARXRISW {
        _SSI_RIS_DMARXRISW { w: self }
    }
    #[doc = "Bit 5 - SSI Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_dmatxris(&mut self) -> _SSI_RIS_DMATXRISW {
        _SSI_RIS_DMATXRISW { w: self }
    }
    #[doc = "Bit 6 - End of Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_eotris(&mut self) -> _SSI_RIS_EOTRISW {
        _SSI_RIS_EOTRISW { w: self }
    }
}
