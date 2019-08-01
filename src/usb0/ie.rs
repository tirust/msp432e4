#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::IE {
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
pub struct USB_IE_SUSPNDR {
    bits: bool,
}
impl USB_IE_SUSPNDR {
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
pub struct _USB_IE_SUSPNDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IE_SUSPNDW<'a> {
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
pub struct USB_IE_RESUMER {
    bits: bool,
}
impl USB_IE_RESUMER {
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
pub struct _USB_IE_RESUMEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IE_RESUMEW<'a> {
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
pub struct USB_IE_BABBLER {
    bits: bool,
}
impl USB_IE_BABBLER {
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
pub struct _USB_IE_BABBLEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IE_BABBLEW<'a> {
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
pub struct USB_IE_SOFR {
    bits: bool,
}
impl USB_IE_SOFR {
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
pub struct _USB_IE_SOFW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IE_SOFW<'a> {
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
pub struct USB_IE_CONNR {
    bits: bool,
}
impl USB_IE_CONNR {
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
pub struct _USB_IE_CONNW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IE_CONNW<'a> {
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
pub struct USB_IE_DISCONR {
    bits: bool,
}
impl USB_IE_DISCONR {
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
pub struct _USB_IE_DISCONW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IE_DISCONW<'a> {
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
pub struct USB_IE_SESREQR {
    bits: bool,
}
impl USB_IE_SESREQR {
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
pub struct _USB_IE_SESREQW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IE_SESREQW<'a> {
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
pub struct USB_IE_VBUSERRR {
    bits: bool,
}
impl USB_IE_VBUSERRR {
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
pub struct _USB_IE_VBUSERRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IE_VBUSERRW<'a> {
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
pub struct USB_IE_RESETR {
    bits: bool,
}
impl USB_IE_RESETR {
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
pub struct _USB_IE_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IE_RESETW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Enable SUSPEND Interrupt"]
    #[inline(always)]
    pub fn usb_ie_suspnd(&self) -> USB_IE_SUSPNDR {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_IE_SUSPNDR { bits }
    }
    #[doc = "Bit 1 - Enable RESUME Interrupt"]
    #[inline(always)]
    pub fn usb_ie_resume(&self) -> USB_IE_RESUMER {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_IE_RESUMER { bits }
    }
    #[doc = "Bit 2 - Enable Babble Interrupt"]
    #[inline(always)]
    pub fn usb_ie_babble(&self) -> USB_IE_BABBLER {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_IE_BABBLER { bits }
    }
    #[doc = "Bit 3 - Enable Start-of-Frame Interrupt"]
    #[inline(always)]
    pub fn usb_ie_sof(&self) -> USB_IE_SOFR {
        let bits = ((self.bits >> 3) & 1) != 0;
        USB_IE_SOFR { bits }
    }
    #[doc = "Bit 4 - Enable Connect Interrupt"]
    #[inline(always)]
    pub fn usb_ie_conn(&self) -> USB_IE_CONNR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_IE_CONNR { bits }
    }
    #[doc = "Bit 5 - Enable Disconnect Interrupt"]
    #[inline(always)]
    pub fn usb_ie_discon(&self) -> USB_IE_DISCONR {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_IE_DISCONR { bits }
    }
    #[doc = "Bit 6 - Enable Session Request (OTG only)"]
    #[inline(always)]
    pub fn usb_ie_sesreq(&self) -> USB_IE_SESREQR {
        let bits = ((self.bits >> 6) & 1) != 0;
        USB_IE_SESREQR { bits }
    }
    #[doc = "Bit 7 - Enable VBUS Error Interrupt (OTG only)"]
    #[inline(always)]
    pub fn usb_ie_vbuserr(&self) -> USB_IE_VBUSERRR {
        let bits = ((self.bits >> 7) & 1) != 0;
        USB_IE_VBUSERRR { bits }
    }
    #[doc = "Bit 2 - Enable RESET Interrupt"]
    #[inline(always)]
    pub fn usb_ie_reset(&self) -> USB_IE_RESETR {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_IE_RESETR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable SUSPEND Interrupt"]
    #[inline(always)]
    pub fn usb_ie_suspnd(&mut self) -> _USB_IE_SUSPNDW {
        _USB_IE_SUSPNDW { w: self }
    }
    #[doc = "Bit 1 - Enable RESUME Interrupt"]
    #[inline(always)]
    pub fn usb_ie_resume(&mut self) -> _USB_IE_RESUMEW {
        _USB_IE_RESUMEW { w: self }
    }
    #[doc = "Bit 2 - Enable Babble Interrupt"]
    #[inline(always)]
    pub fn usb_ie_babble(&mut self) -> _USB_IE_BABBLEW {
        _USB_IE_BABBLEW { w: self }
    }
    #[doc = "Bit 3 - Enable Start-of-Frame Interrupt"]
    #[inline(always)]
    pub fn usb_ie_sof(&mut self) -> _USB_IE_SOFW {
        _USB_IE_SOFW { w: self }
    }
    #[doc = "Bit 4 - Enable Connect Interrupt"]
    #[inline(always)]
    pub fn usb_ie_conn(&mut self) -> _USB_IE_CONNW {
        _USB_IE_CONNW { w: self }
    }
    #[doc = "Bit 5 - Enable Disconnect Interrupt"]
    #[inline(always)]
    pub fn usb_ie_discon(&mut self) -> _USB_IE_DISCONW {
        _USB_IE_DISCONW { w: self }
    }
    #[doc = "Bit 6 - Enable Session Request (OTG only)"]
    #[inline(always)]
    pub fn usb_ie_sesreq(&mut self) -> _USB_IE_SESREQW {
        _USB_IE_SESREQW { w: self }
    }
    #[doc = "Bit 7 - Enable VBUS Error Interrupt (OTG only)"]
    #[inline(always)]
    pub fn usb_ie_vbuserr(&mut self) -> _USB_IE_VBUSERRW {
        _USB_IE_VBUSERRW { w: self }
    }
    #[doc = "Bit 2 - Enable RESET Interrupt"]
    #[inline(always)]
    pub fn usb_ie_reset(&mut self) -> _USB_IE_RESETW {
        _USB_IE_RESETW { w: self }
    }
}
