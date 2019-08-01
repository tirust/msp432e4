#[doc = r"Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::RXIS {
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
    pub const fn reset_value() -> u16 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Value of the field"]
pub struct USB_RXIS_EP1R {
    bits: bool,
}
impl USB_RXIS_EP1R {
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
pub struct _USB_RXIS_EP1W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXIS_EP1W<'a> {
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
        self.w.bits |= ((value as u16) & 1) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_RXIS_EP2R {
    bits: bool,
}
impl USB_RXIS_EP2R {
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
pub struct _USB_RXIS_EP2W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXIS_EP2W<'a> {
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
        self.w.bits |= ((value as u16) & 1) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_RXIS_EP3R {
    bits: bool,
}
impl USB_RXIS_EP3R {
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
pub struct _USB_RXIS_EP3W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXIS_EP3W<'a> {
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
        self.w.bits |= ((value as u16) & 1) << 3;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_RXIS_EP4R {
    bits: bool,
}
impl USB_RXIS_EP4R {
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
pub struct _USB_RXIS_EP4W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXIS_EP4W<'a> {
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
        self.w.bits |= ((value as u16) & 1) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_RXIS_EP5R {
    bits: bool,
}
impl USB_RXIS_EP5R {
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
pub struct _USB_RXIS_EP5W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXIS_EP5W<'a> {
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
        self.w.bits |= ((value as u16) & 1) << 5;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_RXIS_EP6R {
    bits: bool,
}
impl USB_RXIS_EP6R {
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
pub struct _USB_RXIS_EP6W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXIS_EP6W<'a> {
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
        self.w.bits |= ((value as u16) & 1) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_RXIS_EP7R {
    bits: bool,
}
impl USB_RXIS_EP7R {
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
pub struct _USB_RXIS_EP7W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXIS_EP7W<'a> {
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
        self.w.bits |= ((value as u16) & 1) << 7;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 1 - RX Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep1(&self) -> USB_RXIS_EP1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_RXIS_EP1R { bits }
    }
    #[doc = "Bit 2 - RX Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep2(&self) -> USB_RXIS_EP2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_RXIS_EP2R { bits }
    }
    #[doc = "Bit 3 - RX Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep3(&self) -> USB_RXIS_EP3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        USB_RXIS_EP3R { bits }
    }
    #[doc = "Bit 4 - RX Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep4(&self) -> USB_RXIS_EP4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_RXIS_EP4R { bits }
    }
    #[doc = "Bit 5 - RX Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep5(&self) -> USB_RXIS_EP5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_RXIS_EP5R { bits }
    }
    #[doc = "Bit 6 - RX Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep6(&self) -> USB_RXIS_EP6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        USB_RXIS_EP6R { bits }
    }
    #[doc = "Bit 7 - RX Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep7(&self) -> USB_RXIS_EP7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        USB_RXIS_EP7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - RX Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep1(&mut self) -> _USB_RXIS_EP1W {
        _USB_RXIS_EP1W { w: self }
    }
    #[doc = "Bit 2 - RX Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep2(&mut self) -> _USB_RXIS_EP2W {
        _USB_RXIS_EP2W { w: self }
    }
    #[doc = "Bit 3 - RX Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep3(&mut self) -> _USB_RXIS_EP3W {
        _USB_RXIS_EP3W { w: self }
    }
    #[doc = "Bit 4 - RX Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep4(&mut self) -> _USB_RXIS_EP4W {
        _USB_RXIS_EP4W { w: self }
    }
    #[doc = "Bit 5 - RX Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep5(&mut self) -> _USB_RXIS_EP5W {
        _USB_RXIS_EP5W { w: self }
    }
    #[doc = "Bit 6 - RX Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep6(&mut self) -> _USB_RXIS_EP6W {
        _USB_RXIS_EP6W { w: self }
    }
    #[doc = "Bit 7 - RX Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep7(&mut self) -> _USB_RXIS_EP7W {
        _USB_RXIS_EP7W { w: self }
    }
}
