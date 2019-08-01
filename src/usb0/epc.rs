#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPC {
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
#[doc = "Possible values of the field `USB_EPC_EPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_EPC_EPENR {
    #[doc = "Power Enable Active Low"]
    USB_EPC_EPEN_LOW,
    #[doc = "Power Enable Active High"]
    USB_EPC_EPEN_HIGH,
    #[doc = "Power Enable High if VBUS Low (OTG only)"]
    USB_EPC_EPEN_VBLOW,
    #[doc = "Power Enable High if VBUS High (OTG only)"]
    USB_EPC_EPEN_VBHIGH,
}
impl USB_EPC_EPENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_EPC_EPENR::USB_EPC_EPEN_LOW => 0,
            USB_EPC_EPENR::USB_EPC_EPEN_HIGH => 1,
            USB_EPC_EPENR::USB_EPC_EPEN_VBLOW => 2,
            USB_EPC_EPENR::USB_EPC_EPEN_VBHIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_EPC_EPENR {
        match value {
            0 => USB_EPC_EPENR::USB_EPC_EPEN_LOW,
            1 => USB_EPC_EPENR::USB_EPC_EPEN_HIGH,
            2 => USB_EPC_EPENR::USB_EPC_EPEN_VBLOW,
            3 => USB_EPC_EPENR::USB_EPC_EPEN_VBHIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_EPC_EPEN_LOW`"]
    #[inline(always)]
    pub fn is_usb_epc_epen_low(&self) -> bool {
        *self == USB_EPC_EPENR::USB_EPC_EPEN_LOW
    }
    #[doc = "Checks if the value of the field is `USB_EPC_EPEN_HIGH`"]
    #[inline(always)]
    pub fn is_usb_epc_epen_high(&self) -> bool {
        *self == USB_EPC_EPENR::USB_EPC_EPEN_HIGH
    }
    #[doc = "Checks if the value of the field is `USB_EPC_EPEN_VBLOW`"]
    #[inline(always)]
    pub fn is_usb_epc_epen_vblow(&self) -> bool {
        *self == USB_EPC_EPENR::USB_EPC_EPEN_VBLOW
    }
    #[doc = "Checks if the value of the field is `USB_EPC_EPEN_VBHIGH`"]
    #[inline(always)]
    pub fn is_usb_epc_epen_vbhigh(&self) -> bool {
        *self == USB_EPC_EPENR::USB_EPC_EPEN_VBHIGH
    }
}
#[doc = "Values that can be written to the field `USB_EPC_EPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_EPC_EPENW {
    #[doc = "Power Enable Active Low"]
    USB_EPC_EPEN_LOW,
    #[doc = "Power Enable Active High"]
    USB_EPC_EPEN_HIGH,
    #[doc = "Power Enable High if VBUS Low (OTG only)"]
    USB_EPC_EPEN_VBLOW,
    #[doc = "Power Enable High if VBUS High (OTG only)"]
    USB_EPC_EPEN_VBHIGH,
}
impl USB_EPC_EPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_EPC_EPENW::USB_EPC_EPEN_LOW => 0,
            USB_EPC_EPENW::USB_EPC_EPEN_HIGH => 1,
            USB_EPC_EPENW::USB_EPC_EPEN_VBLOW => 2,
            USB_EPC_EPENW::USB_EPC_EPEN_VBHIGH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_EPC_EPENW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_EPC_EPENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_EPC_EPENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Power Enable Active Low"]
    #[inline(always)]
    pub fn usb_epc_epen_low(self) -> &'a mut W {
        self.variant(USB_EPC_EPENW::USB_EPC_EPEN_LOW)
    }
    #[doc = "Power Enable Active High"]
    #[inline(always)]
    pub fn usb_epc_epen_high(self) -> &'a mut W {
        self.variant(USB_EPC_EPENW::USB_EPC_EPEN_HIGH)
    }
    #[doc = "Power Enable High if VBUS Low (OTG only)"]
    #[inline(always)]
    pub fn usb_epc_epen_vblow(self) -> &'a mut W {
        self.variant(USB_EPC_EPENW::USB_EPC_EPEN_VBLOW)
    }
    #[doc = "Power Enable High if VBUS High (OTG only)"]
    #[inline(always)]
    pub fn usb_epc_epen_vbhigh(self) -> &'a mut W {
        self.variant(USB_EPC_EPENW::USB_EPC_EPEN_VBHIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_EPC_EPENDER {
    bits: bool,
}
impl USB_EPC_EPENDER {
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
pub struct _USB_EPC_EPENDEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_EPC_EPENDEW<'a> {
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
        self.w.bits |= ((value as u32) & 1) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_EPC_PFLTENR {
    bits: bool,
}
impl USB_EPC_PFLTENR {
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
pub struct _USB_EPC_PFLTENW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_EPC_PFLTENW<'a> {
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
pub struct USB_EPC_PFLTSEN_HIGHR {
    bits: bool,
}
impl USB_EPC_PFLTSEN_HIGHR {
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
pub struct _USB_EPC_PFLTSEN_HIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_EPC_PFLTSEN_HIGHW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_EPC_PFLTAENR {
    bits: bool,
}
impl USB_EPC_PFLTAENR {
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
pub struct _USB_EPC_PFLTAENW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_EPC_PFLTAENW<'a> {
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
        self.w.bits |= ((value as u32) & 1) << 6;
        self.w
    }
}
#[doc = "Possible values of the field `USB_EPC_PFLTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_EPC_PFLTACTR {
    #[doc = "Unchanged"]
    USB_EPC_PFLTACT_UNCHG,
    #[doc = "Tristate"]
    USB_EPC_PFLTACT_TRIS,
    #[doc = "Low"]
    USB_EPC_PFLTACT_LOW,
    #[doc = "High"]
    USB_EPC_PFLTACT_HIGH,
}
impl USB_EPC_PFLTACTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_EPC_PFLTACTR::USB_EPC_PFLTACT_UNCHG => 0,
            USB_EPC_PFLTACTR::USB_EPC_PFLTACT_TRIS => 1,
            USB_EPC_PFLTACTR::USB_EPC_PFLTACT_LOW => 2,
            USB_EPC_PFLTACTR::USB_EPC_PFLTACT_HIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_EPC_PFLTACTR {
        match value {
            0 => USB_EPC_PFLTACTR::USB_EPC_PFLTACT_UNCHG,
            1 => USB_EPC_PFLTACTR::USB_EPC_PFLTACT_TRIS,
            2 => USB_EPC_PFLTACTR::USB_EPC_PFLTACT_LOW,
            3 => USB_EPC_PFLTACTR::USB_EPC_PFLTACT_HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_EPC_PFLTACT_UNCHG`"]
    #[inline(always)]
    pub fn is_usb_epc_pfltact_unchg(&self) -> bool {
        *self == USB_EPC_PFLTACTR::USB_EPC_PFLTACT_UNCHG
    }
    #[doc = "Checks if the value of the field is `USB_EPC_PFLTACT_TRIS`"]
    #[inline(always)]
    pub fn is_usb_epc_pfltact_tris(&self) -> bool {
        *self == USB_EPC_PFLTACTR::USB_EPC_PFLTACT_TRIS
    }
    #[doc = "Checks if the value of the field is `USB_EPC_PFLTACT_LOW`"]
    #[inline(always)]
    pub fn is_usb_epc_pfltact_low(&self) -> bool {
        *self == USB_EPC_PFLTACTR::USB_EPC_PFLTACT_LOW
    }
    #[doc = "Checks if the value of the field is `USB_EPC_PFLTACT_HIGH`"]
    #[inline(always)]
    pub fn is_usb_epc_pfltact_high(&self) -> bool {
        *self == USB_EPC_PFLTACTR::USB_EPC_PFLTACT_HIGH
    }
}
#[doc = "Values that can be written to the field `USB_EPC_PFLTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_EPC_PFLTACTW {
    #[doc = "Unchanged"]
    USB_EPC_PFLTACT_UNCHG,
    #[doc = "Tristate"]
    USB_EPC_PFLTACT_TRIS,
    #[doc = "Low"]
    USB_EPC_PFLTACT_LOW,
    #[doc = "High"]
    USB_EPC_PFLTACT_HIGH,
}
impl USB_EPC_PFLTACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_EPC_PFLTACTW::USB_EPC_PFLTACT_UNCHG => 0,
            USB_EPC_PFLTACTW::USB_EPC_PFLTACT_TRIS => 1,
            USB_EPC_PFLTACTW::USB_EPC_PFLTACT_LOW => 2,
            USB_EPC_PFLTACTW::USB_EPC_PFLTACT_HIGH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_EPC_PFLTACTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_EPC_PFLTACTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_EPC_PFLTACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Unchanged"]
    #[inline(always)]
    pub fn usb_epc_pfltact_unchg(self) -> &'a mut W {
        self.variant(USB_EPC_PFLTACTW::USB_EPC_PFLTACT_UNCHG)
    }
    #[doc = "Tristate"]
    #[inline(always)]
    pub fn usb_epc_pfltact_tris(self) -> &'a mut W {
        self.variant(USB_EPC_PFLTACTW::USB_EPC_PFLTACT_TRIS)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn usb_epc_pfltact_low(self) -> &'a mut W {
        self.variant(USB_EPC_PFLTACTW::USB_EPC_PFLTACT_LOW)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn usb_epc_pfltact_high(self) -> &'a mut W {
        self.variant(USB_EPC_PFLTACTW::USB_EPC_PFLTACT_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - External Power Supply Enable Configuration"]
    #[inline(always)]
    pub fn usb_epc_epen(&self) -> USB_EPC_EPENR {
        USB_EPC_EPENR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bit 2 - EPEN Drive Enable"]
    #[inline(always)]
    pub fn usb_epc_epende(&self) -> USB_EPC_EPENDER {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_EPC_EPENDER { bits }
    }
    #[doc = "Bit 4 - Power Fault Input Enable"]
    #[inline(always)]
    pub fn usb_epc_pflten(&self) -> USB_EPC_PFLTENR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_EPC_PFLTENR { bits }
    }
    #[doc = "Bit 5 - Power Fault Sense"]
    #[inline(always)]
    pub fn usb_epc_pfltsen_high(&self) -> USB_EPC_PFLTSEN_HIGHR {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_EPC_PFLTSEN_HIGHR { bits }
    }
    #[doc = "Bit 6 - Power Fault Action Enable"]
    #[inline(always)]
    pub fn usb_epc_pfltaen(&self) -> USB_EPC_PFLTAENR {
        let bits = ((self.bits >> 6) & 1) != 0;
        USB_EPC_PFLTAENR { bits }
    }
    #[doc = "Bits 8:9 - Power Fault Action"]
    #[inline(always)]
    pub fn usb_epc_pfltact(&self) -> USB_EPC_PFLTACTR {
        USB_EPC_PFLTACTR::_from(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - External Power Supply Enable Configuration"]
    #[inline(always)]
    pub fn usb_epc_epen(&mut self) -> _USB_EPC_EPENW {
        _USB_EPC_EPENW { w: self }
    }
    #[doc = "Bit 2 - EPEN Drive Enable"]
    #[inline(always)]
    pub fn usb_epc_epende(&mut self) -> _USB_EPC_EPENDEW {
        _USB_EPC_EPENDEW { w: self }
    }
    #[doc = "Bit 4 - Power Fault Input Enable"]
    #[inline(always)]
    pub fn usb_epc_pflten(&mut self) -> _USB_EPC_PFLTENW {
        _USB_EPC_PFLTENW { w: self }
    }
    #[doc = "Bit 5 - Power Fault Sense"]
    #[inline(always)]
    pub fn usb_epc_pfltsen_high(&mut self) -> _USB_EPC_PFLTSEN_HIGHW {
        _USB_EPC_PFLTSEN_HIGHW { w: self }
    }
    #[doc = "Bit 6 - Power Fault Action Enable"]
    #[inline(always)]
    pub fn usb_epc_pfltaen(&mut self) -> _USB_EPC_PFLTAENW {
        _USB_EPC_PFLTAENW { w: self }
    }
    #[doc = "Bits 8:9 - Power Fault Action"]
    #[inline(always)]
    pub fn usb_epc_pfltact(&mut self) -> _USB_EPC_PFLTACTW {
        _USB_EPC_PFLTACTW { w: self }
    }
}
