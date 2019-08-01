#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DEVCTL {
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
pub struct USB_DEVCTL_SESSIONR {
    bits: bool,
}
impl USB_DEVCTL_SESSIONR {
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
pub struct _USB_DEVCTL_SESSIONW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DEVCTL_SESSIONW<'a> {
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
pub struct USB_DEVCTL_HOSTREQR {
    bits: bool,
}
impl USB_DEVCTL_HOSTREQR {
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
pub struct _USB_DEVCTL_HOSTREQW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DEVCTL_HOSTREQW<'a> {
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
pub struct USB_DEVCTL_HOSTR {
    bits: bool,
}
impl USB_DEVCTL_HOSTR {
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
pub struct _USB_DEVCTL_HOSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DEVCTL_HOSTW<'a> {
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
#[doc = "Possible values of the field `USB_DEVCTL_VBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_DEVCTL_VBUSR {
    #[doc = "Below SessionEnd"]
    USB_DEVCTL_VBUS_NONE,
    #[doc = "Above SessionEnd, below AValid"]
    USB_DEVCTL_VBUS_SEND,
    #[doc = "Above AValid, below VBUSValid"]
    USB_DEVCTL_VBUS_AVALID,
    #[doc = "Above VBUSValid"]
    USB_DEVCTL_VBUS_VALID,
}
impl USB_DEVCTL_VBUSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_NONE => 0,
            USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_SEND => 1,
            USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_AVALID => 2,
            USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_VALID => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_DEVCTL_VBUSR {
        match value {
            0 => USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_NONE,
            1 => USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_SEND,
            2 => USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_AVALID,
            3 => USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_VALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_DEVCTL_VBUS_NONE`"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_none(&self) -> bool {
        *self == USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_NONE
    }
    #[doc = "Checks if the value of the field is `USB_DEVCTL_VBUS_SEND`"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_send(&self) -> bool {
        *self == USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_SEND
    }
    #[doc = "Checks if the value of the field is `USB_DEVCTL_VBUS_AVALID`"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_avalid(&self) -> bool {
        *self == USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_AVALID
    }
    #[doc = "Checks if the value of the field is `USB_DEVCTL_VBUS_VALID`"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_valid(&self) -> bool {
        *self == USB_DEVCTL_VBUSR::USB_DEVCTL_VBUS_VALID
    }
}
#[doc = "Values that can be written to the field `USB_DEVCTL_VBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_DEVCTL_VBUSW {
    #[doc = "Below SessionEnd"]
    USB_DEVCTL_VBUS_NONE,
    #[doc = "Above SessionEnd, below AValid"]
    USB_DEVCTL_VBUS_SEND,
    #[doc = "Above AValid, below VBUSValid"]
    USB_DEVCTL_VBUS_AVALID,
    #[doc = "Above VBUSValid"]
    USB_DEVCTL_VBUS_VALID,
}
impl USB_DEVCTL_VBUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_DEVCTL_VBUSW::USB_DEVCTL_VBUS_NONE => 0,
            USB_DEVCTL_VBUSW::USB_DEVCTL_VBUS_SEND => 1,
            USB_DEVCTL_VBUSW::USB_DEVCTL_VBUS_AVALID => 2,
            USB_DEVCTL_VBUSW::USB_DEVCTL_VBUS_VALID => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_DEVCTL_VBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DEVCTL_VBUSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_DEVCTL_VBUSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Below SessionEnd"]
    #[inline(always)]
    pub fn usb_devctl_vbus_none(self) -> &'a mut W {
        self.variant(USB_DEVCTL_VBUSW::USB_DEVCTL_VBUS_NONE)
    }
    #[doc = "Above SessionEnd, below AValid"]
    #[inline(always)]
    pub fn usb_devctl_vbus_send(self) -> &'a mut W {
        self.variant(USB_DEVCTL_VBUSW::USB_DEVCTL_VBUS_SEND)
    }
    #[doc = "Above AValid, below VBUSValid"]
    #[inline(always)]
    pub fn usb_devctl_vbus_avalid(self) -> &'a mut W {
        self.variant(USB_DEVCTL_VBUSW::USB_DEVCTL_VBUS_AVALID)
    }
    #[doc = "Above VBUSValid"]
    #[inline(always)]
    pub fn usb_devctl_vbus_valid(self) -> &'a mut W {
        self.variant(USB_DEVCTL_VBUSW::USB_DEVCTL_VBUS_VALID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 3);
        self.w.bits |= ((value as u8) & 3) << 3;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_DEVCTL_LSDEVR {
    bits: bool,
}
impl USB_DEVCTL_LSDEVR {
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
pub struct _USB_DEVCTL_LSDEVW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DEVCTL_LSDEVW<'a> {
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
pub struct USB_DEVCTL_FSDEVR {
    bits: bool,
}
impl USB_DEVCTL_FSDEVR {
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
pub struct _USB_DEVCTL_FSDEVW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DEVCTL_FSDEVW<'a> {
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
pub struct USB_DEVCTL_DEVR {
    bits: bool,
}
impl USB_DEVCTL_DEVR {
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
pub struct _USB_DEVCTL_DEVW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_DEVCTL_DEVW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Session Start/End (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_session(&self) -> USB_DEVCTL_SESSIONR {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_DEVCTL_SESSIONR { bits }
    }
    #[doc = "Bit 1 - Host Request (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_hostreq(&self) -> USB_DEVCTL_HOSTREQR {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_DEVCTL_HOSTREQR { bits }
    }
    #[doc = "Bit 2 - Host Mode"]
    #[inline(always)]
    pub fn usb_devctl_host(&self) -> USB_DEVCTL_HOSTR {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_DEVCTL_HOSTR { bits }
    }
    #[doc = "Bits 3:4 - VBUS Level (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_vbus(&self) -> USB_DEVCTL_VBUSR {
        USB_DEVCTL_VBUSR::_from(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Low-Speed Device Detected"]
    #[inline(always)]
    pub fn usb_devctl_lsdev(&self) -> USB_DEVCTL_LSDEVR {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_DEVCTL_LSDEVR { bits }
    }
    #[doc = "Bit 6 - Full-Speed Device Detected"]
    #[inline(always)]
    pub fn usb_devctl_fsdev(&self) -> USB_DEVCTL_FSDEVR {
        let bits = ((self.bits >> 6) & 1) != 0;
        USB_DEVCTL_FSDEVR { bits }
    }
    #[doc = "Bit 7 - Device Mode (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_dev(&self) -> USB_DEVCTL_DEVR {
        let bits = ((self.bits >> 7) & 1) != 0;
        USB_DEVCTL_DEVR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Session Start/End (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_session(&mut self) -> _USB_DEVCTL_SESSIONW {
        _USB_DEVCTL_SESSIONW { w: self }
    }
    #[doc = "Bit 1 - Host Request (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_hostreq(&mut self) -> _USB_DEVCTL_HOSTREQW {
        _USB_DEVCTL_HOSTREQW { w: self }
    }
    #[doc = "Bit 2 - Host Mode"]
    #[inline(always)]
    pub fn usb_devctl_host(&mut self) -> _USB_DEVCTL_HOSTW {
        _USB_DEVCTL_HOSTW { w: self }
    }
    #[doc = "Bits 3:4 - VBUS Level (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_vbus(&mut self) -> _USB_DEVCTL_VBUSW {
        _USB_DEVCTL_VBUSW { w: self }
    }
    #[doc = "Bit 5 - Low-Speed Device Detected"]
    #[inline(always)]
    pub fn usb_devctl_lsdev(&mut self) -> _USB_DEVCTL_LSDEVW {
        _USB_DEVCTL_LSDEVW { w: self }
    }
    #[doc = "Bit 6 - Full-Speed Device Detected"]
    #[inline(always)]
    pub fn usb_devctl_fsdev(&mut self) -> _USB_DEVCTL_FSDEVW {
        _USB_DEVCTL_FSDEVW { w: self }
    }
    #[doc = "Bit 7 - Device Mode (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_dev(&mut self) -> _USB_DEVCTL_DEVW {
        _USB_DEVCTL_DEVW { w: self }
    }
}
