#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICR {
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
#[doc = r"Proxy"]
pub struct _UART_ICR_RIMICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_RIMICW<'a> {
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
#[doc = r"Proxy"]
pub struct _UART_ICR_CTSMICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_CTSMICW<'a> {
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
#[doc = r"Proxy"]
pub struct _UART_ICR_DCDMICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_DCDMICW<'a> {
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
#[doc = r"Proxy"]
pub struct _UART_ICR_DSRMICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_DSRMICW<'a> {
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
#[doc = r"Proxy"]
pub struct _UART_ICR_RXICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_RXICW<'a> {
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
#[doc = r"Proxy"]
pub struct _UART_ICR_TXICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_TXICW<'a> {
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
#[doc = r"Proxy"]
pub struct _UART_ICR_RTICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_RTICW<'a> {
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
#[doc = r"Proxy"]
pub struct _UART_ICR_FEICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_FEICW<'a> {
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
#[doc = r"Proxy"]
pub struct _UART_ICR_PEICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_PEICW<'a> {
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
        self.w.bits &= !(1 << 8);
        self.w.bits |= ((value as u32) & 1) << 8;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _UART_ICR_BEICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_BEICW<'a> {
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
        self.w.bits &= !(1 << 9);
        self.w.bits |= ((value as u32) & 1) << 9;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _UART_ICR_OEICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_OEICW<'a> {
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
        self.w.bits &= !(1 << 10);
        self.w.bits |= ((value as u32) & 1) << 10;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _UART_ICR_EOTICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_EOTICW<'a> {
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
        self.w.bits &= !(1 << 11);
        self.w.bits |= ((value as u32) & 1) << 11;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _UART_ICR_9BITICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_9BITICW<'a> {
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
        self.w.bits &= !(1 << 12);
        self.w.bits |= ((value as u32) & 1) << 12;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _UART_ICR_DMARXICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_DMARXICW<'a> {
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
        self.w.bits &= !(1 << 16);
        self.w.bits |= ((value as u32) & 1) << 16;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _UART_ICR_DMATXICW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ICR_DMATXICW<'a> {
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
        self.w.bits &= !(1 << 17);
        self.w.bits |= ((value as u32) & 1) << 17;
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - UART Ring Indicator Modem Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_rimic(&mut self) -> _UART_ICR_RIMICW {
        _UART_ICR_RIMICW { w: self }
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_ctsmic(&mut self) -> _UART_ICR_CTSMICW {
        _UART_ICR_CTSMICW { w: self }
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_dcdmic(&mut self) -> _UART_ICR_DCDMICW {
        _UART_ICR_DCDMICW { w: self }
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_dsrmic(&mut self) -> _UART_ICR_DSRMICW {
        _UART_ICR_DSRMICW { w: self }
    }
    #[doc = "Bit 4 - Receive Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_rxic(&mut self) -> _UART_ICR_RXICW {
        _UART_ICR_RXICW { w: self }
    }
    #[doc = "Bit 5 - Transmit Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_txic(&mut self) -> _UART_ICR_TXICW {
        _UART_ICR_TXICW { w: self }
    }
    #[doc = "Bit 6 - Receive Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_rtic(&mut self) -> _UART_ICR_RTICW {
        _UART_ICR_RTICW { w: self }
    }
    #[doc = "Bit 7 - Framing Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_feic(&mut self) -> _UART_ICR_FEICW {
        _UART_ICR_FEICW { w: self }
    }
    #[doc = "Bit 8 - Parity Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_peic(&mut self) -> _UART_ICR_PEICW {
        _UART_ICR_PEICW { w: self }
    }
    #[doc = "Bit 9 - Break Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_beic(&mut self) -> _UART_ICR_BEICW {
        _UART_ICR_BEICW { w: self }
    }
    #[doc = "Bit 10 - Overrun Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_oeic(&mut self) -> _UART_ICR_OEICW {
        _UART_ICR_OEICW { w: self }
    }
    #[doc = "Bit 11 - End of Transmission Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_eotic(&mut self) -> _UART_ICR_EOTICW {
        _UART_ICR_EOTICW { w: self }
    }
    #[doc = "Bit 12 - 9-Bit Mode Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_9bitic(&mut self) -> _UART_ICR_9BITICW {
        _UART_ICR_9BITICW { w: self }
    }
    #[doc = "Bit 16 - Receive DMA Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_dmarxic(&mut self) -> _UART_ICR_DMARXICW {
        _UART_ICR_DMARXICW { w: self }
    }
    #[doc = "Bit 17 - Transmit DMA Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_dmatxic(&mut self) -> _UART_ICR_DMATXICW {
        _UART_ICR_DMATXICW { w: self }
    }
}
