#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PP {
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
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `USB_PP_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_PP_TYPER {
    #[doc = "The first-generation USB controller revision"]
    USB_PP_TYPE_0,
    #[doc = "The second-generation USB controller revision"]
    USB_PP_TYPE_1,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl USB_PP_TYPER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_PP_TYPER::USB_PP_TYPE_0 => 0,
            USB_PP_TYPER::USB_PP_TYPE_1 => 1,
            USB_PP_TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_PP_TYPER {
        match value {
            0 => USB_PP_TYPER::USB_PP_TYPE_0,
            1 => USB_PP_TYPER::USB_PP_TYPE_1,
            i => USB_PP_TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USB_PP_TYPE_0`"]
    #[inline(always)]
    pub fn is_usb_pp_type_0(&self) -> bool {
        *self == USB_PP_TYPER::USB_PP_TYPE_0
    }
    #[doc = "Checks if the value of the field is `USB_PP_TYPE_1`"]
    #[inline(always)]
    pub fn is_usb_pp_type_1(&self) -> bool {
        *self == USB_PP_TYPER::USB_PP_TYPE_1
    }
}
#[doc = "Values that can be written to the field `USB_PP_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_PP_TYPEW {
    #[doc = "The first-generation USB controller revision"]
    USB_PP_TYPE_0,
    #[doc = "The second-generation USB controller revision"]
    USB_PP_TYPE_1,
}
impl USB_PP_TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_PP_TYPEW::USB_PP_TYPE_0 => 0,
            USB_PP_TYPEW::USB_PP_TYPE_1 => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_PP_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_PP_TYPEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_PP_TYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The first-generation USB controller revision"]
    #[inline(always)]
    pub fn usb_pp_type_0(self) -> &'a mut W {
        self.variant(USB_PP_TYPEW::USB_PP_TYPE_0)
    }
    #[doc = "The second-generation USB controller revision"]
    #[inline(always)]
    pub fn usb_pp_type_1(self) -> &'a mut W {
        self.variant(USB_PP_TYPEW::USB_PP_TYPE_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_PP_PHYR {
    bits: bool,
}
impl USB_PP_PHYR {
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
pub struct _USB_PP_PHYW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_PP_PHYW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_PP_ULPIR {
    bits: bool,
}
impl USB_PP_ULPIR {
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
pub struct _USB_PP_ULPIW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_PP_ULPIW<'a> {
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
#[doc = "Possible values of the field `USB_PP_USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_PP_USBR {
    #[doc = "DEVICE"]
    USB_PP_USB_DEVICE,
    #[doc = "HOST"]
    USB_PP_USB_HOSTDEVICE,
    #[doc = "OTG"]
    USB_PP_USB_OTG,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl USB_PP_USBR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_PP_USBR::USB_PP_USB_DEVICE => 1,
            USB_PP_USBR::USB_PP_USB_HOSTDEVICE => 2,
            USB_PP_USBR::USB_PP_USB_OTG => 3,
            USB_PP_USBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_PP_USBR {
        match value {
            1 => USB_PP_USBR::USB_PP_USB_DEVICE,
            2 => USB_PP_USBR::USB_PP_USB_HOSTDEVICE,
            3 => USB_PP_USBR::USB_PP_USB_OTG,
            i => USB_PP_USBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USB_PP_USB_DEVICE`"]
    #[inline(always)]
    pub fn is_usb_pp_usb_device(&self) -> bool {
        *self == USB_PP_USBR::USB_PP_USB_DEVICE
    }
    #[doc = "Checks if the value of the field is `USB_PP_USB_HOSTDEVICE`"]
    #[inline(always)]
    pub fn is_usb_pp_usb_hostdevice(&self) -> bool {
        *self == USB_PP_USBR::USB_PP_USB_HOSTDEVICE
    }
    #[doc = "Checks if the value of the field is `USB_PP_USB_OTG`"]
    #[inline(always)]
    pub fn is_usb_pp_usb_otg(&self) -> bool {
        *self == USB_PP_USBR::USB_PP_USB_OTG
    }
}
#[doc = "Values that can be written to the field `USB_PP_USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_PP_USBW {
    #[doc = "DEVICE"]
    USB_PP_USB_DEVICE,
    #[doc = "HOST"]
    USB_PP_USB_HOSTDEVICE,
    #[doc = "OTG"]
    USB_PP_USB_OTG,
}
impl USB_PP_USBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_PP_USBW::USB_PP_USB_DEVICE => 1,
            USB_PP_USBW::USB_PP_USB_HOSTDEVICE => 2,
            USB_PP_USBW::USB_PP_USB_OTG => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_PP_USBW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_PP_USBW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_PP_USBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DEVICE"]
    #[inline(always)]
    pub fn usb_pp_usb_device(self) -> &'a mut W {
        self.variant(USB_PP_USBW::USB_PP_USB_DEVICE)
    }
    #[doc = "HOST"]
    #[inline(always)]
    pub fn usb_pp_usb_hostdevice(self) -> &'a mut W {
        self.variant(USB_PP_USBW::USB_PP_USB_HOSTDEVICE)
    }
    #[doc = "OTG"]
    #[inline(always)]
    pub fn usb_pp_usb_otg(self) -> &'a mut W {
        self.variant(USB_PP_USBW::USB_PP_USB_OTG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_PP_ECNTR {
    bits: u8,
}
impl USB_PP_ECNTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_PP_ECNTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_PP_ECNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 8);
        self.w.bits |= ((value as u32) & 255) << 8;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Controller Type"]
    #[inline(always)]
    pub fn usb_pp_type(&self) -> USB_PP_TYPER {
        USB_PP_TYPER::_from(((self.bits >> 0) & 15) as u8)
    }
    #[doc = "Bit 4 - PHY Present"]
    #[inline(always)]
    pub fn usb_pp_phy(&self) -> USB_PP_PHYR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_PP_PHYR { bits }
    }
    #[doc = "Bit 5 - ULPI Present"]
    #[inline(always)]
    pub fn usb_pp_ulpi(&self) -> USB_PP_ULPIR {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_PP_ULPIR { bits }
    }
    #[doc = "Bits 6:7 - USB Capability"]
    #[inline(always)]
    pub fn usb_pp_usb(&self) -> USB_PP_USBR {
        USB_PP_USBR::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Endpoint Count"]
    #[inline(always)]
    pub fn usb_pp_ecnt(&self) -> USB_PP_ECNTR {
        let bits = ((self.bits >> 8) & 255) as u8;
        USB_PP_ECNTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Controller Type"]
    #[inline(always)]
    pub fn usb_pp_type(&mut self) -> _USB_PP_TYPEW {
        _USB_PP_TYPEW { w: self }
    }
    #[doc = "Bit 4 - PHY Present"]
    #[inline(always)]
    pub fn usb_pp_phy(&mut self) -> _USB_PP_PHYW {
        _USB_PP_PHYW { w: self }
    }
    #[doc = "Bit 5 - ULPI Present"]
    #[inline(always)]
    pub fn usb_pp_ulpi(&mut self) -> _USB_PP_ULPIW {
        _USB_PP_ULPIW { w: self }
    }
    #[doc = "Bits 6:7 - USB Capability"]
    #[inline(always)]
    pub fn usb_pp_usb(&mut self) -> _USB_PP_USBW {
        _USB_PP_USBW { w: self }
    }
    #[doc = "Bits 8:15 - Endpoint Count"]
    #[inline(always)]
    pub fn usb_pp_ecnt(&mut self) -> _USB_PP_ECNTW {
        _USB_PP_ECNTW { w: self }
    }
}
