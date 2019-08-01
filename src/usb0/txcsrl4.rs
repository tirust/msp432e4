#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TXCSRL4 {
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
    pub const fn reset_value() -> u8 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Value of the field"]
pub struct USB_TXCSRL4_TXRDYR {
    bits: bool,
}
impl USB_TXCSRL4_TXRDYR {
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
pub struct _USB_TXCSRL4_TXRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXCSRL4_TXRDYW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_TXCSRL4_FIFONER {
    bits: bool,
}
impl USB_TXCSRL4_FIFONER {
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
pub struct _USB_TXCSRL4_FIFONEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXCSRL4_FIFONEW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_TXCSRL4_ERRORR {
    bits: bool,
}
impl USB_TXCSRL4_ERRORR {
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
pub struct _USB_TXCSRL4_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXCSRL4_ERRORW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_TXCSRL4_FLUSHR {
    bits: bool,
}
impl USB_TXCSRL4_FLUSHR {
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
pub struct _USB_TXCSRL4_FLUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXCSRL4_FLUSHW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_TXCSRL4_SETUPR {
    bits: bool,
}
impl USB_TXCSRL4_SETUPR {
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
pub struct _USB_TXCSRL4_SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXCSRL4_SETUPW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_TXCSRL4_STALLEDR {
    bits: bool,
}
impl USB_TXCSRL4_STALLEDR {
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
pub struct _USB_TXCSRL4_STALLEDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXCSRL4_STALLEDW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_TXCSRL4_CLRDTR {
    bits: bool,
}
impl USB_TXCSRL4_CLRDTR {
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
pub struct _USB_TXCSRL4_CLRDTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXCSRL4_CLRDTW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_TXCSRL4_NAKTOR {
    bits: bool,
}
impl USB_TXCSRL4_NAKTOR {
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
pub struct _USB_TXCSRL4_NAKTOW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXCSRL4_NAKTOW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_TXCSRL4_UNDRNR {
    bits: bool,
}
impl USB_TXCSRL4_UNDRNR {
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
pub struct _USB_TXCSRL4_UNDRNW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXCSRL4_UNDRNW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_TXCSRL4_STALLR {
    bits: bool,
}
impl USB_TXCSRL4_STALLR {
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
pub struct _USB_TXCSRL4_STALLW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXCSRL4_STALLW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn usb_txcsrl4_txrdy(&self) -> USB_TXCSRL4_TXRDYR {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_TXCSRL4_TXRDYR { bits }
    }
    #[doc = "Bit 1 - FIFO Not Empty"]
    #[inline(always)]
    pub fn usb_txcsrl4_fifone(&self) -> USB_TXCSRL4_FIFONER {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_TXCSRL4_FIFONER { bits }
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn usb_txcsrl4_error(&self) -> USB_TXCSRL4_ERRORR {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_TXCSRL4_ERRORR { bits }
    }
    #[doc = "Bit 3 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_txcsrl4_flush(&self) -> USB_TXCSRL4_FLUSHR {
        let bits = ((self.bits >> 3) & 1) != 0;
        USB_TXCSRL4_FLUSHR { bits }
    }
    #[doc = "Bit 4 - Setup Packet"]
    #[inline(always)]
    pub fn usb_txcsrl4_setup(&self) -> USB_TXCSRL4_SETUPR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_TXCSRL4_SETUPR { bits }
    }
    #[doc = "Bit 5 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_txcsrl4_stalled(&self) -> USB_TXCSRL4_STALLEDR {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_TXCSRL4_STALLEDR { bits }
    }
    #[doc = "Bit 6 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrl4_clrdt(&self) -> USB_TXCSRL4_CLRDTR {
        let bits = ((self.bits >> 6) & 1) != 0;
        USB_TXCSRL4_CLRDTR { bits }
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_txcsrl4_nakto(&self) -> USB_TXCSRL4_NAKTOR {
        let bits = ((self.bits >> 7) & 1) != 0;
        USB_TXCSRL4_NAKTOR { bits }
    }
    #[doc = "Bit 2 - Underrun"]
    #[inline(always)]
    pub fn usb_txcsrl4_undrn(&self) -> USB_TXCSRL4_UNDRNR {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_TXCSRL4_UNDRNR { bits }
    }
    #[doc = "Bit 4 - Send STALL"]
    #[inline(always)]
    pub fn usb_txcsrl4_stall(&self) -> USB_TXCSRL4_STALLR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_TXCSRL4_STALLR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn usb_txcsrl4_txrdy(&mut self) -> _USB_TXCSRL4_TXRDYW {
        _USB_TXCSRL4_TXRDYW { w: self }
    }
    #[doc = "Bit 1 - FIFO Not Empty"]
    #[inline(always)]
    pub fn usb_txcsrl4_fifone(&mut self) -> _USB_TXCSRL4_FIFONEW {
        _USB_TXCSRL4_FIFONEW { w: self }
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn usb_txcsrl4_error(&mut self) -> _USB_TXCSRL4_ERRORW {
        _USB_TXCSRL4_ERRORW { w: self }
    }
    #[doc = "Bit 3 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_txcsrl4_flush(&mut self) -> _USB_TXCSRL4_FLUSHW {
        _USB_TXCSRL4_FLUSHW { w: self }
    }
    #[doc = "Bit 4 - Setup Packet"]
    #[inline(always)]
    pub fn usb_txcsrl4_setup(&mut self) -> _USB_TXCSRL4_SETUPW {
        _USB_TXCSRL4_SETUPW { w: self }
    }
    #[doc = "Bit 5 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_txcsrl4_stalled(&mut self) -> _USB_TXCSRL4_STALLEDW {
        _USB_TXCSRL4_STALLEDW { w: self }
    }
    #[doc = "Bit 6 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrl4_clrdt(&mut self) -> _USB_TXCSRL4_CLRDTW {
        _USB_TXCSRL4_CLRDTW { w: self }
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_txcsrl4_nakto(&mut self) -> _USB_TXCSRL4_NAKTOW {
        _USB_TXCSRL4_NAKTOW { w: self }
    }
    #[doc = "Bit 2 - Underrun"]
    #[inline(always)]
    pub fn usb_txcsrl4_undrn(&mut self) -> _USB_TXCSRL4_UNDRNW {
        _USB_TXCSRL4_UNDRNW { w: self }
    }
    #[doc = "Bit 4 - Send STALL"]
    #[inline(always)]
    pub fn usb_txcsrl4_stall(&mut self) -> _USB_TXCSRL4_STALLW {
        _USB_TXCSRL4_STALLW { w: self }
    }
}