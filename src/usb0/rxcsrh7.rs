#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::RXCSRH7 {
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
pub struct USB_RXCSRH7_INCOMPRXR {
    bits: bool,
}
impl USB_RXCSRH7_INCOMPRXR {
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
pub struct _USB_RXCSRH7_INCOMPRXW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXCSRH7_INCOMPRXW<'a> {
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
pub struct USB_RXCSRH7_DTR {
    bits: bool,
}
impl USB_RXCSRH7_DTR {
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
pub struct _USB_RXCSRH7_DTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXCSRH7_DTW<'a> {
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
pub struct USB_RXCSRH7_DTWER {
    bits: bool,
}
impl USB_RXCSRH7_DTWER {
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
pub struct _USB_RXCSRH7_DTWEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXCSRH7_DTWEW<'a> {
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
pub struct USB_RXCSRH7_DMAMODR {
    bits: bool,
}
impl USB_RXCSRH7_DMAMODR {
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
pub struct _USB_RXCSRH7_DMAMODW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXCSRH7_DMAMODW<'a> {
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
pub struct USB_RXCSRH7_PIDERRR {
    bits: bool,
}
impl USB_RXCSRH7_PIDERRR {
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
pub struct _USB_RXCSRH7_PIDERRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXCSRH7_PIDERRW<'a> {
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
pub struct USB_RXCSRH7_DMAENR {
    bits: bool,
}
impl USB_RXCSRH7_DMAENR {
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
pub struct _USB_RXCSRH7_DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXCSRH7_DMAENW<'a> {
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
pub struct USB_RXCSRH7_AUTORQR {
    bits: bool,
}
impl USB_RXCSRH7_AUTORQR {
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
pub struct _USB_RXCSRH7_AUTORQW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXCSRH7_AUTORQW<'a> {
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
pub struct USB_RXCSRH7_AUTOCLR {
    bits: bool,
}
impl USB_RXCSRH7_AUTOCLR {
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
pub struct _USB_RXCSRH7_AUTOCLW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXCSRH7_AUTOCLW<'a> {
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
pub struct USB_RXCSRH7_DISNYETR {
    bits: bool,
}
impl USB_RXCSRH7_DISNYETR {
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
pub struct _USB_RXCSRH7_DISNYETW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXCSRH7_DISNYETW<'a> {
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
pub struct USB_RXCSRH7_ISOR {
    bits: bool,
}
impl USB_RXCSRH7_ISOR {
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
pub struct _USB_RXCSRH7_ISOW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXCSRH7_ISOW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Incomplete RX Transmission Status"]
    #[inline(always)]
    pub fn usb_rxcsrh7_incomprx(&self) -> USB_RXCSRH7_INCOMPRXR {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_RXCSRH7_INCOMPRXR { bits }
    }
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dt(&self) -> USB_RXCSRH7_DTR {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_RXCSRH7_DTR { bits }
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dtwe(&self) -> USB_RXCSRH7_DTWER {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_RXCSRH7_DTWER { bits }
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dmamod(&self) -> USB_RXCSRH7_DMAMODR {
        let bits = ((self.bits >> 3) & 1) != 0;
        USB_RXCSRH7_DMAMODR { bits }
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn usb_rxcsrh7_piderr(&self) -> USB_RXCSRH7_PIDERRR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_RXCSRH7_PIDERRR { bits }
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dmaen(&self) -> USB_RXCSRH7_DMAENR {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_RXCSRH7_DMAENR { bits }
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn usb_rxcsrh7_autorq(&self) -> USB_RXCSRH7_AUTORQR {
        let bits = ((self.bits >> 6) & 1) != 0;
        USB_RXCSRH7_AUTORQR { bits }
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn usb_rxcsrh7_autocl(&self) -> USB_RXCSRH7_AUTOCLR {
        let bits = ((self.bits >> 7) & 1) != 0;
        USB_RXCSRH7_AUTOCLR { bits }
    }
    #[doc = "Bit 4 - Disable NYET"]
    #[inline(always)]
    pub fn usb_rxcsrh7_disnyet(&self) -> USB_RXCSRH7_DISNYETR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_RXCSRH7_DISNYETR { bits }
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    pub fn usb_rxcsrh7_iso(&self) -> USB_RXCSRH7_ISOR {
        let bits = ((self.bits >> 6) & 1) != 0;
        USB_RXCSRH7_ISOR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Incomplete RX Transmission Status"]
    #[inline(always)]
    pub fn usb_rxcsrh7_incomprx(&mut self) -> _USB_RXCSRH7_INCOMPRXW {
        _USB_RXCSRH7_INCOMPRXW { w: self }
    }
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dt(&mut self) -> _USB_RXCSRH7_DTW {
        _USB_RXCSRH7_DTW { w: self }
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dtwe(&mut self) -> _USB_RXCSRH7_DTWEW {
        _USB_RXCSRH7_DTWEW { w: self }
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dmamod(&mut self) -> _USB_RXCSRH7_DMAMODW {
        _USB_RXCSRH7_DMAMODW { w: self }
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn usb_rxcsrh7_piderr(&mut self) -> _USB_RXCSRH7_PIDERRW {
        _USB_RXCSRH7_PIDERRW { w: self }
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh7_dmaen(&mut self) -> _USB_RXCSRH7_DMAENW {
        _USB_RXCSRH7_DMAENW { w: self }
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn usb_rxcsrh7_autorq(&mut self) -> _USB_RXCSRH7_AUTORQW {
        _USB_RXCSRH7_AUTORQW { w: self }
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn usb_rxcsrh7_autocl(&mut self) -> _USB_RXCSRH7_AUTOCLW {
        _USB_RXCSRH7_AUTOCLW { w: self }
    }
    #[doc = "Bit 4 - Disable NYET"]
    #[inline(always)]
    pub fn usb_rxcsrh7_disnyet(&mut self) -> _USB_RXCSRH7_DISNYETW {
        _USB_RXCSRH7_DISNYETW { w: self }
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    pub fn usb_rxcsrh7_iso(&mut self) -> _USB_RXCSRH7_ISOW {
        _USB_RXCSRH7_ISOW { w: self }
    }
}
