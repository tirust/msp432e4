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
pub struct SSI_MIS_RORMISR {
    bits: bool,
}
impl SSI_MIS_RORMISR {
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
pub struct _SSI_MIS_RORMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_MIS_RORMISW<'a> {
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
pub struct SSI_MIS_RTMISR {
    bits: bool,
}
impl SSI_MIS_RTMISR {
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
pub struct _SSI_MIS_RTMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_MIS_RTMISW<'a> {
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
pub struct SSI_MIS_RXMISR {
    bits: bool,
}
impl SSI_MIS_RXMISR {
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
pub struct _SSI_MIS_RXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_MIS_RXMISW<'a> {
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
pub struct SSI_MIS_TXMISR {
    bits: bool,
}
impl SSI_MIS_TXMISR {
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
pub struct _SSI_MIS_TXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_MIS_TXMISW<'a> {
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
pub struct SSI_MIS_DMARXMISR {
    bits: bool,
}
impl SSI_MIS_DMARXMISR {
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
pub struct _SSI_MIS_DMARXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_MIS_DMARXMISW<'a> {
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
pub struct SSI_MIS_DMATXMISR {
    bits: bool,
}
impl SSI_MIS_DMATXMISR {
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
pub struct _SSI_MIS_DMATXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_MIS_DMATXMISW<'a> {
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
pub struct SSI_MIS_EOTMISR {
    bits: bool,
}
impl SSI_MIS_EOTMISR {
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
pub struct _SSI_MIS_EOTMISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_MIS_EOTMISW<'a> {
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
    #[doc = "Bit 0 - SSI Receive Overrun Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rormis(&self) -> SSI_MIS_RORMISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SSI_MIS_RORMISR { bits }
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rtmis(&self) -> SSI_MIS_RTMISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SSI_MIS_RTMISR { bits }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rxmis(&self) -> SSI_MIS_RXMISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SSI_MIS_RXMISR { bits }
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_txmis(&self) -> SSI_MIS_TXMISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SSI_MIS_TXMISR { bits }
    }
    #[doc = "Bit 4 - SSI Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_dmarxmis(&self) -> SSI_MIS_DMARXMISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SSI_MIS_DMARXMISR { bits }
    }
    #[doc = "Bit 5 - SSI Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_dmatxmis(&self) -> SSI_MIS_DMATXMISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        SSI_MIS_DMATXMISR { bits }
    }
    #[doc = "Bit 6 - End of Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_eotmis(&self) -> SSI_MIS_EOTMISR {
        let bits = ((self.bits >> 6) & 1) != 0;
        SSI_MIS_EOTMISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SSI Receive Overrun Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rormis(&mut self) -> _SSI_MIS_RORMISW {
        _SSI_MIS_RORMISW { w: self }
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rtmis(&mut self) -> _SSI_MIS_RTMISW {
        _SSI_MIS_RTMISW { w: self }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rxmis(&mut self) -> _SSI_MIS_RXMISW {
        _SSI_MIS_RXMISW { w: self }
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_txmis(&mut self) -> _SSI_MIS_TXMISW {
        _SSI_MIS_TXMISW { w: self }
    }
    #[doc = "Bit 4 - SSI Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_dmarxmis(&mut self) -> _SSI_MIS_DMARXMISW {
        _SSI_MIS_DMARXMISW { w: self }
    }
    #[doc = "Bit 5 - SSI Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_dmatxmis(&mut self) -> _SSI_MIS_DMATXMISW {
        _SSI_MIS_DMATXMISW { w: self }
    }
    #[doc = "Bit 6 - End of Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_eotmis(&mut self) -> _SSI_MIS_EOTMISW {
        _SSI_MIS_EOTMISW { w: self }
    }
}
