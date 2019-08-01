#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CSRL0 {
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
    pub const fn reset_value() -> u8 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_RXRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_RXRDYW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 0;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_TXRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_TXRDYW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 1;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_STALLEDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_STALLEDW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 2;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_DATAENDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_DATAENDW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 3;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_SETENDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_SETENDW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 4;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_STALLW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_STALLW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 5;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_RXRDYCW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_RXRDYCW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 6;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_SETENDCW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_SETENDCW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 7;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_SETUPW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 3;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_ERRORW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 4;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_REQPKTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_REQPKTW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 5;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_STATUSW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 6;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _USB_CSRL0_NAKTOW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CSRL0_NAKTOW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 7;
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Receive Packet Ready"]
    #[inline(always)]
    pub fn usb_csrl0_rxrdy(&mut self) -> _USB_CSRL0_RXRDYW {
        _USB_CSRL0_RXRDYW { w: self }
    }
    #[doc = "Bit 1 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn usb_csrl0_txrdy(&mut self) -> _USB_CSRL0_TXRDYW {
        _USB_CSRL0_TXRDYW { w: self }
    }
    #[doc = "Bit 2 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_csrl0_stalled(&mut self) -> _USB_CSRL0_STALLEDW {
        _USB_CSRL0_STALLEDW { w: self }
    }
    #[doc = "Bit 3 - Data End"]
    #[inline(always)]
    pub fn usb_csrl0_dataend(&mut self) -> _USB_CSRL0_DATAENDW {
        _USB_CSRL0_DATAENDW { w: self }
    }
    #[doc = "Bit 4 - Setup End"]
    #[inline(always)]
    pub fn usb_csrl0_setend(&mut self) -> _USB_CSRL0_SETENDW {
        _USB_CSRL0_SETENDW { w: self }
    }
    #[doc = "Bit 5 - Send Stall"]
    #[inline(always)]
    pub fn usb_csrl0_stall(&mut self) -> _USB_CSRL0_STALLW {
        _USB_CSRL0_STALLW { w: self }
    }
    #[doc = "Bit 6 - RXRDY Clear"]
    #[inline(always)]
    pub fn usb_csrl0_rxrdyc(&mut self) -> _USB_CSRL0_RXRDYCW {
        _USB_CSRL0_RXRDYCW { w: self }
    }
    #[doc = "Bit 7 - Setup End Clear"]
    #[inline(always)]
    pub fn usb_csrl0_setendc(&mut self) -> _USB_CSRL0_SETENDCW {
        _USB_CSRL0_SETENDCW { w: self }
    }
    #[doc = "Bit 3 - Setup Packet"]
    #[inline(always)]
    pub fn usb_csrl0_setup(&mut self) -> _USB_CSRL0_SETUPW {
        _USB_CSRL0_SETUPW { w: self }
    }
    #[doc = "Bit 4 - Error"]
    #[inline(always)]
    pub fn usb_csrl0_error(&mut self) -> _USB_CSRL0_ERRORW {
        _USB_CSRL0_ERRORW { w: self }
    }
    #[doc = "Bit 5 - Request Packet"]
    #[inline(always)]
    pub fn usb_csrl0_reqpkt(&mut self) -> _USB_CSRL0_REQPKTW {
        _USB_CSRL0_REQPKTW { w: self }
    }
    #[doc = "Bit 6 - STATUS Packet"]
    #[inline(always)]
    pub fn usb_csrl0_status(&mut self) -> _USB_CSRL0_STATUSW {
        _USB_CSRL0_STATUSW { w: self }
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_csrl0_nakto(&mut self) -> _USB_CSRL0_NAKTOW {
        _USB_CSRL0_NAKTOW { w: self }
    }
}
