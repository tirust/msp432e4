#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::RXTYPE5 {
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
pub struct USB_RXTYPE5_TEPR {
    bits: u8,
}
impl USB_RXTYPE5_TEPR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_RXTYPE5_TEPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXTYPE5_TEPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u8) & 15) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `USB_RXTYPE5_PROTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_RXTYPE5_PROTOR {
    #[doc = "Control"]
    USB_RXTYPE5_PROTO_CTRL,
    #[doc = "Isochronous"]
    USB_RXTYPE5_PROTO_ISOC,
    #[doc = "Bulk"]
    USB_RXTYPE5_PROTO_BULK,
    #[doc = "Interrupt"]
    USB_RXTYPE5_PROTO_INT,
}
impl USB_RXTYPE5_PROTOR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_CTRL => 0,
            USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_ISOC => 1,
            USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_BULK => 2,
            USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_INT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_RXTYPE5_PROTOR {
        match value {
            0 => USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_CTRL,
            1 => USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_ISOC,
            2 => USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_BULK,
            3 => USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_INT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE5_PROTO_CTRL`"]
    #[inline(always)]
    pub fn is_usb_rxtype5_proto_ctrl(&self) -> bool {
        *self == USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_CTRL
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE5_PROTO_ISOC`"]
    #[inline(always)]
    pub fn is_usb_rxtype5_proto_isoc(&self) -> bool {
        *self == USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_ISOC
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE5_PROTO_BULK`"]
    #[inline(always)]
    pub fn is_usb_rxtype5_proto_bulk(&self) -> bool {
        *self == USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_BULK
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE5_PROTO_INT`"]
    #[inline(always)]
    pub fn is_usb_rxtype5_proto_int(&self) -> bool {
        *self == USB_RXTYPE5_PROTOR::USB_RXTYPE5_PROTO_INT
    }
}
#[doc = "Values that can be written to the field `USB_RXTYPE5_PROTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_RXTYPE5_PROTOW {
    #[doc = "Control"]
    USB_RXTYPE5_PROTO_CTRL,
    #[doc = "Isochronous"]
    USB_RXTYPE5_PROTO_ISOC,
    #[doc = "Bulk"]
    USB_RXTYPE5_PROTO_BULK,
    #[doc = "Interrupt"]
    USB_RXTYPE5_PROTO_INT,
}
impl USB_RXTYPE5_PROTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_RXTYPE5_PROTOW::USB_RXTYPE5_PROTO_CTRL => 0,
            USB_RXTYPE5_PROTOW::USB_RXTYPE5_PROTO_ISOC => 1,
            USB_RXTYPE5_PROTOW::USB_RXTYPE5_PROTO_BULK => 2,
            USB_RXTYPE5_PROTOW::USB_RXTYPE5_PROTO_INT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_RXTYPE5_PROTOW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXTYPE5_PROTOW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_RXTYPE5_PROTOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn usb_rxtype5_proto_ctrl(self) -> &'a mut W {
        self.variant(USB_RXTYPE5_PROTOW::USB_RXTYPE5_PROTO_CTRL)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn usb_rxtype5_proto_isoc(self) -> &'a mut W {
        self.variant(USB_RXTYPE5_PROTOW::USB_RXTYPE5_PROTO_ISOC)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn usb_rxtype5_proto_bulk(self) -> &'a mut W {
        self.variant(USB_RXTYPE5_PROTOW::USB_RXTYPE5_PROTO_BULK)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn usb_rxtype5_proto_int(self) -> &'a mut W {
        self.variant(USB_RXTYPE5_PROTOW::USB_RXTYPE5_PROTO_INT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u8) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `USB_RXTYPE5_SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_RXTYPE5_SPEEDR {
    #[doc = "Default"]
    USB_RXTYPE5_SPEED_DFLT,
    #[doc = "High"]
    USB_RXTYPE5_SPEED_HIGH,
    #[doc = "Full"]
    USB_RXTYPE5_SPEED_FULL,
    #[doc = "Low"]
    USB_RXTYPE5_SPEED_LOW,
}
impl USB_RXTYPE5_SPEEDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_DFLT => 0,
            USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_HIGH => 1,
            USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_FULL => 2,
            USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_LOW => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_RXTYPE5_SPEEDR {
        match value {
            0 => USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_DFLT,
            1 => USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_HIGH,
            2 => USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_FULL,
            3 => USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE5_SPEED_DFLT`"]
    #[inline(always)]
    pub fn is_usb_rxtype5_speed_dflt(&self) -> bool {
        *self == USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_DFLT
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE5_SPEED_HIGH`"]
    #[inline(always)]
    pub fn is_usb_rxtype5_speed_high(&self) -> bool {
        *self == USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_HIGH
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE5_SPEED_FULL`"]
    #[inline(always)]
    pub fn is_usb_rxtype5_speed_full(&self) -> bool {
        *self == USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_FULL
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE5_SPEED_LOW`"]
    #[inline(always)]
    pub fn is_usb_rxtype5_speed_low(&self) -> bool {
        *self == USB_RXTYPE5_SPEEDR::USB_RXTYPE5_SPEED_LOW
    }
}
#[doc = "Values that can be written to the field `USB_RXTYPE5_SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_RXTYPE5_SPEEDW {
    #[doc = "Default"]
    USB_RXTYPE5_SPEED_DFLT,
    #[doc = "High"]
    USB_RXTYPE5_SPEED_HIGH,
    #[doc = "Full"]
    USB_RXTYPE5_SPEED_FULL,
    #[doc = "Low"]
    USB_RXTYPE5_SPEED_LOW,
}
impl USB_RXTYPE5_SPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_RXTYPE5_SPEEDW::USB_RXTYPE5_SPEED_DFLT => 0,
            USB_RXTYPE5_SPEEDW::USB_RXTYPE5_SPEED_HIGH => 1,
            USB_RXTYPE5_SPEEDW::USB_RXTYPE5_SPEED_FULL => 2,
            USB_RXTYPE5_SPEEDW::USB_RXTYPE5_SPEED_LOW => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_RXTYPE5_SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXTYPE5_SPEEDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_RXTYPE5_SPEEDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default"]
    #[inline(always)]
    pub fn usb_rxtype5_speed_dflt(self) -> &'a mut W {
        self.variant(USB_RXTYPE5_SPEEDW::USB_RXTYPE5_SPEED_DFLT)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn usb_rxtype5_speed_high(self) -> &'a mut W {
        self.variant(USB_RXTYPE5_SPEEDW::USB_RXTYPE5_SPEED_HIGH)
    }
    #[doc = "Full"]
    #[inline(always)]
    pub fn usb_rxtype5_speed_full(self) -> &'a mut W {
        self.variant(USB_RXTYPE5_SPEEDW::USB_RXTYPE5_SPEED_FULL)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn usb_rxtype5_speed_low(self) -> &'a mut W {
        self.variant(USB_RXTYPE5_SPEEDW::USB_RXTYPE5_SPEED_LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u8) & 3) << 6;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Target Endpoint Number"]
    #[inline(always)]
    pub fn usb_rxtype5_tep(&self) -> USB_RXTYPE5_TEPR {
        let bits = ((self.bits >> 0) & 15) as u8;
        USB_RXTYPE5_TEPR { bits }
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline(always)]
    pub fn usb_rxtype5_proto(&self) -> USB_RXTYPE5_PROTOR {
        USB_RXTYPE5_PROTOR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_rxtype5_speed(&self) -> USB_RXTYPE5_SPEEDR {
        USB_RXTYPE5_SPEEDR::_from(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Target Endpoint Number"]
    #[inline(always)]
    pub fn usb_rxtype5_tep(&mut self) -> _USB_RXTYPE5_TEPW {
        _USB_RXTYPE5_TEPW { w: self }
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline(always)]
    pub fn usb_rxtype5_proto(&mut self) -> _USB_RXTYPE5_PROTOW {
        _USB_RXTYPE5_PROTOW { w: self }
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_rxtype5_speed(&mut self) -> _USB_RXTYPE5_SPEEDW {
        _USB_RXTYPE5_SPEEDW { w: self }
    }
}
