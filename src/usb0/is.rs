#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::IS {
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
pub struct USB_IS_SUSPENDR {
    bits: bool,
}
impl USB_IS_SUSPENDR {
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
pub struct _USB_IS_SUSPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IS_SUSPENDW<'a> {
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
pub struct USB_IS_RESUMER {
    bits: bool,
}
impl USB_IS_RESUMER {
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
pub struct _USB_IS_RESUMEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IS_RESUMEW<'a> {
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
pub struct USB_IS_BABBLER {
    bits: bool,
}
impl USB_IS_BABBLER {
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
pub struct _USB_IS_BABBLEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IS_BABBLEW<'a> {
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
pub struct USB_IS_SOFR {
    bits: bool,
}
impl USB_IS_SOFR {
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
pub struct _USB_IS_SOFW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IS_SOFW<'a> {
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
pub struct USB_IS_CONNR {
    bits: bool,
}
impl USB_IS_CONNR {
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
pub struct _USB_IS_CONNW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IS_CONNW<'a> {
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
pub struct USB_IS_DISCONR {
    bits: bool,
}
impl USB_IS_DISCONR {
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
pub struct _USB_IS_DISCONW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IS_DISCONW<'a> {
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
pub struct USB_IS_SESREQR {
    bits: bool,
}
impl USB_IS_SESREQR {
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
pub struct _USB_IS_SESREQW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IS_SESREQW<'a> {
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
pub struct USB_IS_VBUSERRR {
    bits: bool,
}
impl USB_IS_VBUSERRR {
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
pub struct _USB_IS_VBUSERRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IS_VBUSERRW<'a> {
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
pub struct USB_IS_RESETR {
    bits: bool,
}
impl USB_IS_RESETR {
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
pub struct _USB_IS_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_IS_RESETW<'a> {
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
    #[doc = "Bit 0 - SUSPEND Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_suspend(&self) -> USB_IS_SUSPENDR {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_IS_SUSPENDR { bits }
    }
    #[doc = "Bit 1 - RESUME Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_resume(&self) -> USB_IS_RESUMER {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_IS_RESUMER { bits }
    }
    #[doc = "Bit 2 - Babble Detected"]
    #[inline(always)]
    pub fn usb_is_babble(&self) -> USB_IS_BABBLER {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_IS_BABBLER { bits }
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn usb_is_sof(&self) -> USB_IS_SOFR {
        let bits = ((self.bits >> 3) & 1) != 0;
        USB_IS_SOFR { bits }
    }
    #[doc = "Bit 4 - Session Connect"]
    #[inline(always)]
    pub fn usb_is_conn(&self) -> USB_IS_CONNR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_IS_CONNR { bits }
    }
    #[doc = "Bit 5 - Session Disconnect (OTG only)"]
    #[inline(always)]
    pub fn usb_is_discon(&self) -> USB_IS_DISCONR {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_IS_DISCONR { bits }
    }
    #[doc = "Bit 6 - SESSION REQUEST (OTG only)"]
    #[inline(always)]
    pub fn usb_is_sesreq(&self) -> USB_IS_SESREQR {
        let bits = ((self.bits >> 6) & 1) != 0;
        USB_IS_SESREQR { bits }
    }
    #[doc = "Bit 7 - VBUS Error (OTG only)"]
    #[inline(always)]
    pub fn usb_is_vbuserr(&self) -> USB_IS_VBUSERRR {
        let bits = ((self.bits >> 7) & 1) != 0;
        USB_IS_VBUSERRR { bits }
    }
    #[doc = "Bit 2 - RESET Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_reset(&self) -> USB_IS_RESETR {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_IS_RESETR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SUSPEND Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_suspend(&mut self) -> _USB_IS_SUSPENDW {
        _USB_IS_SUSPENDW { w: self }
    }
    #[doc = "Bit 1 - RESUME Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_resume(&mut self) -> _USB_IS_RESUMEW {
        _USB_IS_RESUMEW { w: self }
    }
    #[doc = "Bit 2 - Babble Detected"]
    #[inline(always)]
    pub fn usb_is_babble(&mut self) -> _USB_IS_BABBLEW {
        _USB_IS_BABBLEW { w: self }
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn usb_is_sof(&mut self) -> _USB_IS_SOFW {
        _USB_IS_SOFW { w: self }
    }
    #[doc = "Bit 4 - Session Connect"]
    #[inline(always)]
    pub fn usb_is_conn(&mut self) -> _USB_IS_CONNW {
        _USB_IS_CONNW { w: self }
    }
    #[doc = "Bit 5 - Session Disconnect (OTG only)"]
    #[inline(always)]
    pub fn usb_is_discon(&mut self) -> _USB_IS_DISCONW {
        _USB_IS_DISCONW { w: self }
    }
    #[doc = "Bit 6 - SESSION REQUEST (OTG only)"]
    #[inline(always)]
    pub fn usb_is_sesreq(&mut self) -> _USB_IS_SESREQW {
        _USB_IS_SESREQW { w: self }
    }
    #[doc = "Bit 7 - VBUS Error (OTG only)"]
    #[inline(always)]
    pub fn usb_is_vbuserr(&mut self) -> _USB_IS_VBUSERRW {
        _USB_IS_VBUSERRW { w: self }
    }
    #[doc = "Bit 2 - RESET Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_reset(&mut self) -> _USB_IS_RESETW {
        _USB_IS_RESETW { w: self }
    }
}
