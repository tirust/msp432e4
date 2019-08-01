#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TXTYPE3 {
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
pub struct USB_TXTYPE3_TEPR {
    bits: u8,
}
impl USB_TXTYPE3_TEPR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_TXTYPE3_TEPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXTYPE3_TEPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u8) & 15) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `USB_TXTYPE3_PROTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_TXTYPE3_PROTOR {
    #[doc = "Control"]
    USB_TXTYPE3_PROTO_CTRL,
    #[doc = "Isochronous"]
    USB_TXTYPE3_PROTO_ISOC,
    #[doc = "Bulk"]
    USB_TXTYPE3_PROTO_BULK,
    #[doc = "Interrupt"]
    USB_TXTYPE3_PROTO_INT,
}
impl USB_TXTYPE3_PROTOR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_CTRL => 0,
            USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_ISOC => 1,
            USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_BULK => 2,
            USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_INT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_TXTYPE3_PROTOR {
        match value {
            0 => USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_CTRL,
            1 => USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_ISOC,
            2 => USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_BULK,
            3 => USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_INT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_TXTYPE3_PROTO_CTRL`"]
    #[inline(always)]
    pub fn is_usb_txtype3_proto_ctrl(&self) -> bool {
        *self == USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_CTRL
    }
    #[doc = "Checks if the value of the field is `USB_TXTYPE3_PROTO_ISOC`"]
    #[inline(always)]
    pub fn is_usb_txtype3_proto_isoc(&self) -> bool {
        *self == USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_ISOC
    }
    #[doc = "Checks if the value of the field is `USB_TXTYPE3_PROTO_BULK`"]
    #[inline(always)]
    pub fn is_usb_txtype3_proto_bulk(&self) -> bool {
        *self == USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_BULK
    }
    #[doc = "Checks if the value of the field is `USB_TXTYPE3_PROTO_INT`"]
    #[inline(always)]
    pub fn is_usb_txtype3_proto_int(&self) -> bool {
        *self == USB_TXTYPE3_PROTOR::USB_TXTYPE3_PROTO_INT
    }
}
#[doc = "Values that can be written to the field `USB_TXTYPE3_PROTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_TXTYPE3_PROTOW {
    #[doc = "Control"]
    USB_TXTYPE3_PROTO_CTRL,
    #[doc = "Isochronous"]
    USB_TXTYPE3_PROTO_ISOC,
    #[doc = "Bulk"]
    USB_TXTYPE3_PROTO_BULK,
    #[doc = "Interrupt"]
    USB_TXTYPE3_PROTO_INT,
}
impl USB_TXTYPE3_PROTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_TXTYPE3_PROTOW::USB_TXTYPE3_PROTO_CTRL => 0,
            USB_TXTYPE3_PROTOW::USB_TXTYPE3_PROTO_ISOC => 1,
            USB_TXTYPE3_PROTOW::USB_TXTYPE3_PROTO_BULK => 2,
            USB_TXTYPE3_PROTOW::USB_TXTYPE3_PROTO_INT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_TXTYPE3_PROTOW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXTYPE3_PROTOW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_TXTYPE3_PROTOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn usb_txtype3_proto_ctrl(self) -> &'a mut W {
        self.variant(USB_TXTYPE3_PROTOW::USB_TXTYPE3_PROTO_CTRL)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn usb_txtype3_proto_isoc(self) -> &'a mut W {
        self.variant(USB_TXTYPE3_PROTOW::USB_TXTYPE3_PROTO_ISOC)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn usb_txtype3_proto_bulk(self) -> &'a mut W {
        self.variant(USB_TXTYPE3_PROTOW::USB_TXTYPE3_PROTO_BULK)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn usb_txtype3_proto_int(self) -> &'a mut W {
        self.variant(USB_TXTYPE3_PROTOW::USB_TXTYPE3_PROTO_INT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u8) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `USB_TXTYPE3_SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_TXTYPE3_SPEEDR {
    #[doc = "Default"]
    USB_TXTYPE3_SPEED_DFLT,
    #[doc = "High"]
    USB_TXTYPE3_SPEED_HIGH,
    #[doc = "Full"]
    USB_TXTYPE3_SPEED_FULL,
    #[doc = "Low"]
    USB_TXTYPE3_SPEED_LOW,
}
impl USB_TXTYPE3_SPEEDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_DFLT => 0,
            USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_HIGH => 1,
            USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_FULL => 2,
            USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_LOW => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_TXTYPE3_SPEEDR {
        match value {
            0 => USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_DFLT,
            1 => USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_HIGH,
            2 => USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_FULL,
            3 => USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_TXTYPE3_SPEED_DFLT`"]
    #[inline(always)]
    pub fn is_usb_txtype3_speed_dflt(&self) -> bool {
        *self == USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_DFLT
    }
    #[doc = "Checks if the value of the field is `USB_TXTYPE3_SPEED_HIGH`"]
    #[inline(always)]
    pub fn is_usb_txtype3_speed_high(&self) -> bool {
        *self == USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_HIGH
    }
    #[doc = "Checks if the value of the field is `USB_TXTYPE3_SPEED_FULL`"]
    #[inline(always)]
    pub fn is_usb_txtype3_speed_full(&self) -> bool {
        *self == USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_FULL
    }
    #[doc = "Checks if the value of the field is `USB_TXTYPE3_SPEED_LOW`"]
    #[inline(always)]
    pub fn is_usb_txtype3_speed_low(&self) -> bool {
        *self == USB_TXTYPE3_SPEEDR::USB_TXTYPE3_SPEED_LOW
    }
}
#[doc = "Values that can be written to the field `USB_TXTYPE3_SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_TXTYPE3_SPEEDW {
    #[doc = "Default"]
    USB_TXTYPE3_SPEED_DFLT,
    #[doc = "High"]
    USB_TXTYPE3_SPEED_HIGH,
    #[doc = "Full"]
    USB_TXTYPE3_SPEED_FULL,
    #[doc = "Low"]
    USB_TXTYPE3_SPEED_LOW,
}
impl USB_TXTYPE3_SPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_TXTYPE3_SPEEDW::USB_TXTYPE3_SPEED_DFLT => 0,
            USB_TXTYPE3_SPEEDW::USB_TXTYPE3_SPEED_HIGH => 1,
            USB_TXTYPE3_SPEEDW::USB_TXTYPE3_SPEED_FULL => 2,
            USB_TXTYPE3_SPEEDW::USB_TXTYPE3_SPEED_LOW => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_TXTYPE3_SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXTYPE3_SPEEDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_TXTYPE3_SPEEDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default"]
    #[inline(always)]
    pub fn usb_txtype3_speed_dflt(self) -> &'a mut W {
        self.variant(USB_TXTYPE3_SPEEDW::USB_TXTYPE3_SPEED_DFLT)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn usb_txtype3_speed_high(self) -> &'a mut W {
        self.variant(USB_TXTYPE3_SPEEDW::USB_TXTYPE3_SPEED_HIGH)
    }
    #[doc = "Full"]
    #[inline(always)]
    pub fn usb_txtype3_speed_full(self) -> &'a mut W {
        self.variant(USB_TXTYPE3_SPEEDW::USB_TXTYPE3_SPEED_FULL)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn usb_txtype3_speed_low(self) -> &'a mut W {
        self.variant(USB_TXTYPE3_SPEEDW::USB_TXTYPE3_SPEED_LOW)
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
    pub fn usb_txtype3_tep(&self) -> USB_TXTYPE3_TEPR {
        let bits = ((self.bits >> 0) & 15) as u8;
        USB_TXTYPE3_TEPR { bits }
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline(always)]
    pub fn usb_txtype3_proto(&self) -> USB_TXTYPE3_PROTOR {
        USB_TXTYPE3_PROTOR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_txtype3_speed(&self) -> USB_TXTYPE3_SPEEDR {
        USB_TXTYPE3_SPEEDR::_from(((self.bits >> 6) & 3) as u8)
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
    pub fn usb_txtype3_tep(&mut self) -> _USB_TXTYPE3_TEPW {
        _USB_TXTYPE3_TEPW { w: self }
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline(always)]
    pub fn usb_txtype3_proto(&mut self) -> _USB_TXTYPE3_PROTOW {
        _USB_TXTYPE3_PROTOW { w: self }
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_txtype3_speed(&mut self) -> _USB_TXTYPE3_SPEEDW {
        _USB_TXTYPE3_SPEEDW { w: self }
    }
}